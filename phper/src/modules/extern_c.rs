use crate::ini;
use crate::modules::{get_module, Module, ModuleInfo, Registerer, GLOBAL_MODULE_NUMBER};
use crate::zend_result::ZResult;
use phper_macros::c_str_ptr;
use phper_sys::{
    display_ini_entries, php_info_print_table_end, php_info_print_table_row,
    php_info_print_table_start, zend_module_entry,
};
use std::mem::take;
use std::os::raw::c_int;

pub(super) unsafe extern "C" fn module_info(zend_module: *mut zend_module_entry) {
    let module = get_module();

    php_info_print_table_start();
    if !module.version.as_bytes().is_empty() {
        php_info_print_table_row(2, c_str_ptr!("version"), module.version.as_ptr());
    }
    if !module.author.as_bytes().is_empty() {
        php_info_print_table_row(2, c_str_ptr!("authors"), module.author.as_ptr());
    }
    for (key, value) in &module.infos {
        php_info_print_table_row(2, key.as_ptr(), value.as_ptr());
    }
    php_info_print_table_end();

    display_ini_entries(zend_module);
}

pub(super) unsafe extern "C" fn module_startup(_type: c_int, module_number: c_int) -> c_int {
    let module: &mut Module = get_module();
    GLOBAL_MODULE_NUMBER = module_number;

    ini::register(take(&mut module.ini_entities), module_number);

    for entity in take(&mut module.entities).into_iter() {
        if let Err(err) = entity.register(module_number) {
            crate::output::log(
                crate::output::LogLevel::Error,
                format!("Failed to register: {err:?}"),
            );
            return ZResult::Failure.into();
        }
    }

    if let Some(f) = take(&mut module.module_init) {
        f(ModuleInfo {
            ty: _type,
            number: module_number,
        });
    }

    ZResult::Success.into()
}

pub(super) unsafe extern "C" fn module_shutdown(_type: c_int, module_number: c_int) -> c_int {
    ini::unregister(module_number);

    // let module = get_module();
    //
    // if let Some(f) = take(&mut module.module_shutdown) {
    //     f(ModuleInfo {
    //         ty: _type,
    //         number: module_number,
    //     });
    // }
    //
    // if let Some(ref mut f) = take(&mut module.request_init) {
    //     let _b = Box::from_raw(f);
    // }
    //
    // if let Some(ref mut f) = take(&mut module.request_shutdown) {
    //     let _b = Box::from_raw(f);
    // }
    //
    // drop(Box::from_raw(GLOBAL_MODULE));
    // GLOBAL_MODULE = null_mut();

    ZResult::Success.into()
}

pub(super) unsafe extern "C" fn request_startup(_type: c_int, module_number: c_int) -> c_int {
    let f = get_module().request_init.unwrap_unchecked();

    f(ModuleInfo {
        ty: _type,
        number: module_number,
    });

    ZResult::Success.into()
}

pub(super) unsafe extern "C" fn request_shutdown(_type: c_int, module_number: c_int) -> c_int {
    let f = get_module().request_shutdown.unwrap_unchecked();

    f(ModuleInfo {
        ty: _type,
        number: module_number,
    });

    ZResult::Success.into()
}
