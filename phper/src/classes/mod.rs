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
pub mod zend_classes;

use crate::{
    functions::{FunctionEntry, MethodEntity},
    objects::{StateObj, StateObject, ZObject},
    sys::*,
    types::Scalar,
    values::ZVal,
};
use std::ptr::null;
use std::{
    any::Any,
    convert::TryInto,
    mem::{size_of, zeroed},
    os::raw::c_int,
    ptr::null_mut,
    slice,
    sync::atomic::{AtomicPtr, Ordering},
};

use self::entry::ClassEntry;

#[allow(clippy::useless_conversion)]
fn find_global_class_entry_ptr(name: impl AsRef<str>) -> *mut zend_class_entry {
    let name = name.as_ref();
    let name = name.to_lowercase();
    unsafe {
        phper_zend_hash_str_find_ptr(
            compiler_globals.class_table,
            name.as_ptr().cast(),
            name.len().try_into().unwrap(),
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
pub struct StaticStateClass {
    inner: AtomicPtr<zend_class_entry>,
}

impl StaticStateClass {
    /// Create empty [StaticStateClass], with null
    /// [zend_class_entry].
    pub const fn null() -> Self {
        Self {
            inner: AtomicPtr::new(null_mut()),
        }
    }

    fn bind(&'static self, ptr: *mut zend_class_entry) {
        self.inner.store(ptr, Ordering::Relaxed);
    }

    /// Converts to class entry.
    pub fn as_class_entry(&'static self) -> &'static ClassEntry {
        unsafe { ClassEntry::from_mut_ptr(self.inner.load(Ordering::Relaxed)) }
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

unsafe impl Sync for StaticStateClass {}

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
#[repr(transparent)]
pub struct StaticInterface {
    inner: AtomicPtr<zend_class_entry>,
}

impl StaticInterface {
    /// Create empty [StaticInterface], with null
    /// [zend_class_entry].
    pub const fn null() -> Self {
        Self {
            inner: AtomicPtr::new(null_mut()),
        }
    }

    fn bind(&'static self, ptr: *mut zend_class_entry) {
        self.inner.store(ptr, Ordering::Relaxed);
    }

    /// Converts to class entry.
    pub fn as_class_entry(&'static self) -> &'static ClassEntry {
        unsafe { ClassEntry::from_mut_ptr(self.inner.load(Ordering::Relaxed)) }
    }
}

pub(crate) type StateConstructor = dyn Fn() -> *mut dyn Any;

pub(crate) type StateCloner = dyn Fn(*const dyn Any) -> *mut dyn Any;

/// Builder for registering interface.
pub struct InterfaceEntity {
    interface: zend_class_entry,
    method_entities: Vec<MethodEntity>,
    extends: Vec<Box<dyn Fn() -> &'static ClassEntry>>,
    bind_interface: Option<&'static StaticInterface>,
}

impl InterfaceEntity {
    /// Construct a new `InterfaceEntity` with interface name.
    pub fn new(interface_name: impl AsRef<str>) -> Self {
        let interface_name = interface_name.as_ref();
        let interface_name_len = interface_name.len();

        Self {
            interface: unsafe {
                phper_init_interface_entry(interface_name.as_ptr().cast(), interface_name_len)
            },
            method_entities: Vec::new(),
            extends: Vec::new(),
            bind_interface: None,
        }
    }

    /// Add member method to interface, with mandatory visibility public
    /// abstract.
    pub fn add_method(
        &mut self,
        name: impl AsRef<str>,
        args: &'static [zend_internal_arg_info],
    ) -> &mut MethodEntity {
        let mut entity = MethodEntity::new(name, None, Visibility::Public, args);
        entity.set_vis_abstract();
        self.method_entities.push(entity);
        self.method_entities.last_mut().unwrap()
    }

    /// Register interface to `extends` the interfaces, due to the interface can
    /// extends multi interface, so this method can be called multi time.
    ///
    /// *Because in the `MINIT` phase, the class starts to register, so the*
    /// *closure is used to return the `ClassEntry` to delay the acquisition of*
    /// *the class.*
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use phper::classes::{ClassEntry, InterfaceEntity};
    ///
    /// let mut interface = InterfaceEntity::new("MyInterface");
    /// interface.extends(|| ClassEntry::from_globals("Stringable").unwrap());
    /// ```
    pub fn extends(&mut self, interface: impl Fn() -> &'static ClassEntry + 'static) {
        self.extends.push(Box::new(interface));
    }

    /// Bind to static [StaticInterface].
    ///
    /// When the interface registered, the [StaticInterface] will be
    /// initialized, so you can use the [StaticInterface] to be implemented
    /// by other class.
    pub fn bind(&mut self, i: &'static StaticInterface) {
        self.bind_interface = Some(i);
    }

    unsafe fn function_entries(&self) -> *const zend_function_entry {
        let mut methods = self
            .method_entities
            .iter()
            .map(|method| FunctionEntry::from_method_entity(method))
            .collect::<Vec<_>>();

        methods.push(zeroed::<zend_function_entry>());

        Box::into_raw(methods.into_boxed_slice()).cast()
    }
}

impl crate::modules::Registerer for InterfaceEntity {
    fn register(&mut self, _: i32) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let class_ce =
                phper_register_interface_entry(&mut self.interface, self.function_entries());

            if let Some(bind_interface) = self.bind_interface {
                bind_interface.bind(class_ce);
            }

            for interface in &self.extends {
                let interface_ce = interface().as_ptr();
                zend_class_implements(class_ce, 1, interface_ce);
            }
        };

        Ok(())
    }
}

/// Builder for declare class property.
struct PropertyEntity {
    name: String,
    visibility: RawVisibility,
    value: Scalar,
}

impl PropertyEntity {
    fn new(name: impl Into<String>, visibility: Visibility, value: impl Into<Scalar>) -> Self {
        Self {
            name: name.into(),
            visibility: visibility as RawVisibility,
            value: value.into(),
        }
    }

    #[inline]
    pub(crate) fn set_vis_static(&mut self) -> &mut Self {
        self.visibility |= ZEND_ACC_STATIC;
        self
    }

    pub(crate) fn declare(&self, ce: *mut zend_class_entry) {
        let name = self.name.as_ptr().cast();
        let name_length = self.name.len();
        let access_type = self.visibility as i32;

        unsafe {
            match &self.value {
                Scalar::Null => {
                    zend_declare_property_null(ce, name, name_length, access_type);
                }
                Scalar::Bool(b) => {
                    zend_declare_property_bool(ce, name, name_length, *b as zend_long, access_type);
                }
                Scalar::I64(i) => {
                    zend_declare_property_long(ce, name, name_length, *i, access_type);
                }
                Scalar::F64(f) => {
                    zend_declare_property_double(ce, name, name_length, *f, access_type);
                }
                Scalar::String(s) => {
                    // If the `ce` is `ZEND_INTERNAL_CLASS`, then the `zend_string` is allocated
                    // as persistent.
                    zend_declare_property_stringl(
                        ce,
                        name,
                        name_length,
                        s.as_ptr().cast(),
                        s.len(),
                        access_type,
                    );
                }
                Scalar::Bytes(b) => {
                    zend_declare_property_stringl(
                        ce,
                        name,
                        name_length,
                        b.as_ptr().cast(),
                        b.len(),
                        access_type,
                    );
                }
            }
        }
    }
}

/// Visibility of class properties and methods.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum Visibility {
    /// Public.
    #[default]
    Public = ZEND_ACC_PUBLIC,

    /// Protected.
    Protected = ZEND_ACC_PROTECTED,

    /// Private.
    Private = ZEND_ACC_PRIVATE,
}

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
    func_ptr = func_ptr.offset(1);
    let has_state_cloner =
        slice::from_raw_parts(func_ptr as *const u8, size_of::<*const StateCloner>())
            != [0u8; size_of::<*const StateCloner>()];

    // Common initialize process.
    let object = state_object.as_mut_object().as_mut_ptr();
    zend_object_std_init(object, ce);
    object_properties_init(object, ce);
    rebuild_object_properties(object);

    // Set handlers
    let mut handlers = Box::new(std_object_handlers);
    handlers.offset = StateObj::offset() as c_int;
    handlers.free_obj = Some(free_object);
    handlers.clone_obj = has_state_cloner.then_some(clone_object);
    (*object).handlers = Box::into_raw(handlers);

    // Call the state constructor and store the state.
    let data = (state_constructor)();
    *state_object.as_mut_any_state() = data;

    object
}

unsafe extern "C" fn clone_object(object: *mut zend_object) -> *mut zend_object {
    clone_object_common(object)
}

unsafe fn clone_object_common(object: *mut zend_object) -> *mut zend_object {
    let ce = (*object).ce;

    // Alloc more memory size to store state data.
    let new_state_object = phper_zend_object_alloc(size_of::<StateObj>(), ce);
    let new_state_object = StateObj::from_mut_ptr(new_state_object);

    // Find the hack elements hidden behind null builtin_function.
    let mut func_ptr = (*(*object).ce).info.internal.builtin_functions;
    while !(*func_ptr).fname.is_null() {
        func_ptr = func_ptr.offset(1);
    }

    // Get state cloner.
    func_ptr = func_ptr.offset(2);
    let state_cloner = func_ptr as *mut *const StateCloner;
    let state_cloner = state_cloner.read().as_ref().unwrap();

    // Initialize and clone members
    let new_object = new_state_object.as_mut_object().as_mut_ptr();
    zend_object_std_init(new_object, ce);
    object_properties_init(new_object, ce);
    zend_objects_clone_members(new_object, object);

    // Set handlers
    (*new_object).handlers = (*object).handlers;

    // Call the state cloner and store the state.
    let state_object = StateObj::from_mut_object_ptr(object);
    let data = (state_cloner)(*state_object.as_mut_any_state());
    *new_state_object.as_mut_any_state() = data;

    new_object
}

unsafe extern "C" fn free_object(object: *mut zend_object) {
    let state_object = StateObj::from_mut_object_ptr(object);

    // Drop the state.
    state_object.drop_state();

    // Original destroy call.
    zend_object_std_dtor(object);
}
