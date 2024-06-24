// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [zend_class_entry].

pub mod entity;
/// Zend Class Entry
pub mod entry;
pub mod interfaces;
pub mod methods;
pub mod zend_classes;

pub use entity::*;
pub use interfaces::*;
pub use methods::MethodEntity;
pub use zend_classes::*;

use crate::{
    objects::{StateObj, StateObject, ZObject},
    sys::*,
    values::ZVal,
};
use std::{
    any::Any,
    mem::size_of,
    os::raw::c_int,
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
};

use self::entry::ClassEntry;

#[inline]
fn find_global_class_entry_ptr(name: impl AsRef<str>) -> *mut zend_class_entry {
    let name = name.as_ref();
    let name = name.to_lowercase();
    unsafe {
        phper_zend_hash_str_find_ptr(
            compiler_globals.class_table,
            name.as_ptr().cast(),
            name.len(),
        )
        .cast()
    }
}

/// The [StaticStateClass] holds
/// [zend_class_entry] and inner state, always as
/// the static variable, and then be bind to [ClassEntity].
///
/// When the class registered (module initialized), the [StaticStateClass] will
/// be initialized, so you can use the [StaticStateClass] to new stateful
/// object, etc.
///
/// So, You shouldn't use [StaticStateClass] in `module_init` stage, because it
/// hasn't initialized.
///
/// # Examples
///
/// ```rust
/// use phper::classes::{ClassEntity, StaticStateClass};
///
/// pub static FOO_CLASS: StaticStateClass<FooState> = StaticStateClass::null();
///
/// #[derive(Default)]
/// pub struct FooState;
///
/// fn make_foo_class() -> ClassEntity<FooState> {
///     let mut class = ClassEntity::new_with_default_state_constructor("Foo");
///     class.bind(&FOO_CLASS);
///     class
/// }
/// ```

#[repr(transparent)]
pub struct StaticStateClass<T>(AtomicPtr<zend_class_entry>, std::marker::PhantomData<T>);

impl<T> StaticStateClass<T> {
    /// Create empty [StaticStateClass], with null
    /// [zend_class_entry].
    pub const fn null() -> Self {
        Self(AtomicPtr::new(null_mut()), std::marker::PhantomData)
    }

    fn bind(&'static self, ptr: *mut zend_class_entry) {
        self.0.store(ptr, Ordering::Relaxed);
    }

    /// Converts to class entry.
    pub fn as_class_entry(&'static self) -> &'static ClassEntry {
        unsafe { ClassEntry::from_mut_ptr(self.0.load(Ordering::Relaxed)) }
    }

    /// Create the object from class and call `__construct` with arguments.
    ///
    /// If the `__construct` is private, or protected and the called scope isn't
    /// parent class, it will throw PHP Error.
    pub fn new_object(&'static self, arguments: impl AsMut<[ZVal]>) -> crate::Result<StateObject> {
        self.as_class_entry()
            .new_object(arguments)
            .map(ZObject::into_raw)
            .map(StateObject::from_raw_object)
    }

    /// Create the object from class, without calling `__construct`.
    ///
    /// **Be careful when `__construct` is necessary.**
    pub fn init_object(&'static self) -> crate::Result<StateObject> {
        self.as_class_entry()
            .init_object()
            .map(ZObject::into_raw)
            .map(StateObject::from_raw_object)
    }
}

unsafe impl<T> Sync for StaticStateClass<T> {}

/// The [StaticInterface]  holds
/// [zend_class_entry], always as the static
/// variable, and then be bind to [InterfaceEntity].
///
/// When the interface registered (module initialized), the [StaticInterface]
/// will be initialized.
///
/// So, You shouldn't use [StaticInterface] in `module_init` stage, because it
/// hasn't initialized.
///
/// # Examples
///
/// ```rust
/// use phper::classes::{InterfaceEntity, StaticInterface};
///
/// pub static FOO_INTERFACE: StaticInterface = StaticInterface::null();
///
/// fn make_foo_interface() -> InterfaceEntity {
///     let mut interface = InterfaceEntity::new("Foo");
///     interface.bind(&FOO_INTERFACE);
///     interface
/// }
/// ```

pub(crate) type StateConstructor = dyn Fn() -> Box<dyn Any>;

// pub(crate) type StateCloner = dyn Fn(*const dyn Any) -> *mut dyn Any;

/// Raw visibility flag.
pub(crate) type RawVisibility = u32;

unsafe extern "C" fn create_object(ce: *mut zend_class_entry) -> *mut zend_object {
    // Alloc more memory size to store state data.
    let state_object = phper_zend_object_alloc(size_of::<StateObj>(), ce);
    let state_object = StateObj::from_mut_ptr(state_object);

    // Find the hack elements hidden behind null builtin_function.
    let mut func_ptr = (*ce).info.internal.builtin_functions;
    while !(*func_ptr).fname.is_null() {
        func_ptr = func_ptr.offset(1);
    }

    // Get state constructor.
    func_ptr = func_ptr.offset(1);
    let state_constructor = func_ptr as *mut *const StateConstructor;
    let state_constructor = state_constructor.read().as_ref().unwrap();

    // Get state cloner.
    // func_ptr = func_ptr.offset(1);
    // let has_state_cloner =
    //     slice::from_raw_parts(func_ptr as *const u8, size_of::<*const StateCloner>())
    //         != [0u8; size_of::<*const StateCloner>()];

    // Common initialize process.
    let object = state_object.as_mut_object().as_mut_ptr();
    zend_object_std_init(object, ce);
    object_properties_init(object, ce);
    rebuild_object_properties(object);

    // Set handlers
    let mut handlers = Box::new(std_object_handlers);
    handlers.offset = StateObj::offset() as c_int;
    handlers.free_obj = Some(free_object);
    // handlers.clone_obj = has_state_cloner.then_some(clone_object);
    (*object).handlers = Box::into_raw(handlers);

    // Call the state constructor and store the state.
    state_object.state = Some(state_constructor());

    object
}

// unsafe extern "C" fn clone_object(object: *mut zend_object) -> *mut zend_object {
//     // clone_object_common(object)
//     object
// }

// unsafe fn clone_object_common(object: *mut zend_object) -> *mut zend_object {
//     let ce = (*object).ce;
//
//     // Alloc more memory size to store state data.
//     let new_state_object = phper_zend_object_alloc(size_of::<StateObj>(), ce);
//     let new_state_object = StateObj::from_mut_ptr(new_state_object);
//
//     // Find the hack elements hidden behind null builtin_function.
//     let mut func_ptr = (*(*object).ce).info.internal.builtin_functions;
//     while !(*func_ptr).fname.is_null() {
//         func_ptr = func_ptr.offset(1);
//     }
//
//     // Get state cloner.
//     func_ptr = func_ptr.offset(2);
//     let state_cloner = func_ptr as *mut *const StateCloner;
//     let state_cloner = state_cloner.read().as_ref().unwrap();
//
//     // Initialize and clone members
//     let new_object = new_state_object.as_mut_object().as_mut_ptr();
//     zend_object_std_init(new_object, ce);
//     object_properties_init(new_object, ce);
//     zend_objects_clone_members(new_object, object);
//
//     // Set handlers
//     (*new_object).handlers = (*object).handlers;
//
//     // Call the state cloner and store the state.
//     let state_object = StateObj::from_mut_object_ptr(object);
//     let data = (state_cloner)(*state_object.as_mut_any_state());
//     *new_state_object.as_mut_any_state() = data;
//
//     new_object
// }

unsafe extern "C" fn free_object(object: *mut zend_object) {
    let state_object = StateObj::from_mut_object_ptr(object);

    // Drop the state.
    state_object.drop_state();

    // Original destroy call.
    zend_object_std_dtor(object);
}
