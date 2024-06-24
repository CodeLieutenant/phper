// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [zend_module_entry].

mod extern_c;
mod function_entry;

use smallvec::SmallVec;
use std::mem::ManuallyDrop;
use std::{
    collections::HashMap,
    ffi::CString,
    mem::{size_of, zeroed},
    os::raw::{c_uchar, c_uint, c_ushort},
    ptr::{null, null_mut},
};

use crate::{
    classes::{entity::ClassEntity, InterfaceEntity},
    constants::{Constant, Flags},
    errors::Throwable,
    functions::{Function, FunctionEntity, FunctionEntry},
    ini,
    sys::*,
    values::ZVal,
};

use crate::functions::Callable;
use crate::modules::function_entry::PHP_FUNCTIONS;
use extern_c::{module_info, module_shutdown, module_startup, request_shutdown, request_startup};

/// Global pointer hold the Module builder.
/// Because PHP is single threaded, so there is no lock here.
static mut GLOBAL_MODULE: *mut Module = null_mut();
pub(crate) static mut GLOBAL_MODULE_NUMBER: i32 = 0;

static mut GLOBAL_MODULE_ENTRY: *mut zend_module_entry = null_mut();

unsafe fn get_module() -> &'static mut Module {
    unsafe { GLOBAL_MODULE.as_mut().unwrap_unchecked() }
}

/// PHP Module information
pub struct ModuleInfo {
    /// Module Type
    pub ty: i32,

    /// Module Number -> Order in which modules are loaded
    pub number: i32,
}

pub(crate) trait Registerer {
    fn register(self, module_number: i32) -> Result<(), Box<dyn std::error::Error>>;
}

/// Builder for registering PHP Module.
#[derive(Default)]
pub struct Module {
    name: CString,
    version: CString,
    author: CString,
    module_init: Option<Box<dyn FnOnce(ModuleInfo)>>,
    module_shutdown: Option<Box<dyn FnOnce(ModuleInfo)>>,
    request_init: Option<&'static dyn Fn(ModuleInfo)>,
    request_shutdown: Option<&'static dyn Fn(ModuleInfo)>,
    entities: Vec<Entities>,
    ini_entities: Vec<zend_ini_entry_def>,
    infos: HashMap<CString, CString>,
    functions: SmallVec<[zend_function_entry; 64]>,
}

pub(crate) enum Entities {
    Constant(Constant),
    Class(ClassEntity<()>),
    Interface(InterfaceEntity),
}

impl Registerer for Entities {
    fn register(self, module_number: i32) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Entities::Constant(con) => con.register(module_number),
            Entities::Class(class) => class.register(module_number),
            Entities::Interface(interface) => interface.register(module_number),
        }
    }
}

impl Module {
    /// Construct the `Module` with base metadata.
    pub fn new(name: impl AsRef<str>, version: impl AsRef<str>, author: impl AsRef<str>) -> Self {
        Self {
            name: CString::new(name.as_ref()).expect("Failed to allocate CString, param: name"),
            version: CString::new(version.as_ref())
                .expect("Failed to allocate CString, param: version"),
            author: CString::new(author.as_ref())
                .expect("Failed to allocate CString, param: author"),
            ..Default::default()
        }
    }

    /// Register `MINIT` hook.
    pub fn on_module_init(&mut self, func: impl FnOnce(ModuleInfo) + 'static) {
        self.module_init = Some(Box::new(func));
    }

    /// Register `MSHUTDOWN` hook.
    pub fn on_module_shutdown(&mut self, func: impl FnOnce(ModuleInfo) + 'static) {
        self.module_shutdown = Some(Box::new(func));
    }

    /// Register `RINIT` hook.
    pub fn on_request_init(&mut self, func: impl Fn(ModuleInfo) + 'static) {
        self.request_init = Some(Box::leak(Box::new(func)));
    }

    /// Register `RSHUTDOWN` hook.
    pub fn on_request_shutdown(&mut self, func: impl Fn(ModuleInfo) + 'static) {
        self.request_shutdown = Some(Box::leak(Box::new(func)));
    }

    /// Register function to module.
    pub fn add_function<F, Z, E>(
        &mut self,
        name: impl AsRef<str>,
        arguments: &'static [zend_internal_arg_info],
        handler: F,
    ) -> &mut Self
    where
        F: Fn(&mut [ZVal]) -> Result<Z, E> + 'static,
        Z: Into<ZVal> + 'static,
        E: Throwable + 'static,
    {
        let handler = Box::new(Function::new(handler));
        let entry = FunctionEntity::new(name, handler, arguments);

        unsafe {
            let val = ManuallyDrop::new(FunctionEntry::from_function_entity(entry));
            self.functions.push(val.0);
        }

        self
    }

    /// Register class to module.
    pub fn add_class<T>(&mut self, class: ClassEntity<T>) -> &mut Self {
        self.entities.push(Entities::Class(unsafe {
            std::mem::transmute::<ClassEntity<T>, ClassEntity<()>>(class)
        }));

        self
    }

    /// Register interface to module.
    pub fn add_interface(&mut self, interface: InterfaceEntity) -> &mut Self {
        self.entities.push(Entities::Interface(interface));

        self
    }

    /// Register constant to module.
    pub fn add_constant(
        &mut self,
        name: impl AsRef<str>,
        value: impl Into<ZVal>,
        flags: Option<Flags>,
    ) -> &mut Self {
        self.entities
            .push(Entities::Constant(Constant::new(name, value, flags)));

        self
    }

    /// Register ini configuration to module.
    pub fn add_ini(
        &mut self,
        name: impl AsRef<str>,
        default_value: impl ini::IntoIniValue,
        policy: ini::Policy,
    ) -> &mut Self {
        let ini = ini::create_ini_entry_ex(
            name.as_ref(),
            default_value.into_ini_value(),
            policy as u32,
            Option::<()>::None,
        );
        self.ini_entities.push(ini);

        self
    }

    /// Register info item.
    ///
    /// # Panics
    ///
    /// Panic if key or value contains '\0'.
    pub fn add_info(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        let key = CString::new(key.into()).expect("key contains '\0'");
        let value = CString::new(value.into()).expect("value contains '\0'");
        self.infos.insert(key, value);

        self
    }

    #[doc(hidden)]
    pub unsafe fn module_entry(mut self) -> *const zend_module_entry {
        if !GLOBAL_MODULE_ENTRY.is_null() {
            return GLOBAL_MODULE_ENTRY;
        }

        self.functions.push(zeroed::<zend_function_entry>());
        self.functions.shrink_to_fit();
        let fns = ManuallyDrop::new(std::mem::take(&mut self.functions).into_boxed_slice());

        let module = Box::leak(Box::new(self));

        let entry = Box::new(zend_module_entry {
            size: size_of::<zend_module_entry>() as c_ushort,
            zend_api: ZEND_MODULE_API_NO as c_uint,
            zend_debug: ZEND_DEBUG as c_uchar,
            zts: USING_ZTS as c_uchar,
            ini_entry: null(),
            deps: null(),
            name: module.name.as_ptr(),
            functions: fns.as_ptr(),
            module_startup_func: Some(module_startup),
            module_shutdown_func: Some(module_shutdown),
            request_startup_func: if module.request_init.is_some() {
                Some(request_startup)
            } else {
                None
            },
            request_shutdown_func: if module.request_shutdown.is_some() {
                Some(request_shutdown)
            } else {
                None
            },
            info_func: Some(module_info),
            version: module.version.as_ptr(),
            globals_size: 0,
            #[cfg(phper_zts)]
            globals_id_ptr: null_mut(),
            #[cfg(not(phper_zts))]
            globals_ptr: null_mut(),
            globals_ctor: None,
            globals_dtor: None,
            post_deactivate_func: None,
            module_started: 0,
            type_: 0,
            handle: null_mut(),
            module_number: 0,
            build_id: phper_get_zend_module_build_id(),
        });

        GLOBAL_MODULE = module;
        GLOBAL_MODULE_ENTRY = Box::into_raw(entry);

        GLOBAL_MODULE_ENTRY
    }
}
