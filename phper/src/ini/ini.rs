// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [zend_ini_entry_def].

use crate::{c_str, strings::ZString};
use std::{
    ffi::{c_int, c_uchar, c_void, CStr},
    mem::zeroed,
    os::raw::c_char,
    ptr::null_mut,
    str,
};

use phper_sys::*;
use crate::ini::Stage;


/// Configuration changeable policy.
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Policy {
    /// Entry can be set anywhere.
    All = PHP_INI_ALL,
    /// Entry can be set in user scripts (like with `ini_set()`) or in the
    /// Windows registry. Entry can be set in `.user.ini`.
    User = PHP_INI_USER,
    /// Entry can be set in `php.ini`, `.htaccess`, `httpd.conf` or `.user.ini`.
    PerDir = PHP_INI_PERDIR,
    /// Entry can be set in `php.ini` or `httpd.conf`.
    System = PHP_INI_SYSTEM,
}


/// The Type which can transform to an ini value.
pub trait IntoIniValue {
    /// transform to an ini value.
    fn into_ini_value(self) -> String;
}

enum PHPIniFunction<T> {
    Exists(unsafe extern "C" fn(*const c_char, usize, c_int, *mut bool) -> T),

    DefaultValue(unsafe extern "C" fn(*const c_char, usize, c_int) -> T),
}

impl IntoIniValue for bool {
    #[inline]
    fn into_ini_value(self) -> String {
        if self {
            "1".to_owned()
        } else {
            "0".to_owned()
        }
    }
}

impl IntoIniValue for i64 {
    #[inline]
    fn into_ini_value(self) -> String {
        self.to_string()
    }
}

impl IntoIniValue for f64 {
    #[inline]
    fn into_ini_value(self) -> String {
        self.to_string()
    }
}

impl IntoIniValue for String {
    #[inline]
    fn into_ini_value(self) -> String {
        self
    }
}

/// The Type which can transform from an ini key name.
///
/// For php7, the zend_ini_* functions receive ini name as `*mut c_char`, but I
/// think it's immutable.
pub trait FromIniValue {
    /// transform from an ini key name.
    fn from_ini_value(name: &str) -> Self;
}

impl FromIniValue for bool {
    fn from_ini_value(name: &str) -> Self {
        let s = <Option<&CStr>>::from_ini_value(name);
        [
            Some(c_str!("1")),
            Some(c_str!("true")),
            Some(c_str!("on")),
            Some(c_str!("On")),
        ]
        .contains(&s)
    }
}

impl FromIniValue for i64 {
    fn from_ini_value(name: &str) -> Self {
        extract_ini_value(name, PHPIniFunction::DefaultValue(zend_ini_long)).unwrap_or_default()
    }
}

impl FromIniValue for f64 {
    fn from_ini_value(name: &str) -> Self {
        unsafe {
            let name_ptr = name.as_ptr() as *mut c_char;
            zend_ini_double(name_ptr, name.len(), 0)
        }
    }
}

impl FromIniValue for Option<&CStr> {
    fn from_ini_value(name: &str) -> Self {
        let ptr = extract_ini_value(name, PHPIniFunction::Exists(zend_ini_string_ex));
        ptr.map(|ptr| unsafe { CStr::from_ptr(ptr) })
    }
}

fn extract_ini_value<T>(name: &str, func: PHPIniFunction<T>) -> Option<T> {
    let name_ptr = name.as_ptr() as *const c_char;

    match func {
        PHPIniFunction::Exists(f) => {
            let mut exists = false;
            let ptr = unsafe { f(name_ptr, name.len(), 0, &mut exists as *mut bool) };

            if exists {
                Some(ptr)
            } else {
                None
            }
        }
        PHPIniFunction::DefaultValue(f) => Some(unsafe { f(name_ptr, name.len(), 0) }),
    }
}

impl FromIniValue for Option<String> {
    fn from_ini_value(name: &str) -> Self {
        let ptr = extract_ini_value(name, PHPIniFunction::Exists(zend_ini_string_ex));

        ptr.map(|ptr| unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string())
    }
}

impl FromIniValue for String {
    fn from_ini_value(name: &str) -> Self {
        let ptr = extract_ini_value(name, PHPIniFunction::Exists(zend_ini_string_ex));

        ptr.map(|ptr| unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string())
            .unwrap_or_default()
    }
}

impl FromIniValue for Option<&str> {
    fn from_ini_value(name: &str) -> Self {
        let ptr = extract_ini_value(name, PHPIniFunction::Exists(zend_ini_string_ex));
        ptr.map(|ptr| unsafe { CStr::from_ptr(ptr) }.to_str().unwrap()) // Totally OK to crash here
    }
}

impl FromIniValue for &str {
    fn from_ini_value(name: &str) -> Self {
        let ptr = extract_ini_value(name, PHPIniFunction::Exists(zend_ini_string_ex));
        ptr.map(|ptr| unsafe { CStr::from_ptr(ptr) }.to_str().unwrap())
            .unwrap_or_default() // Totally OK to crash here
    }
}

/// Zend INI Entry
pub struct Entry {
    /// Has Entry been modified
    pub modified: bool,
    /// Name of the INI Entry
    pub name: ZString,
    /// Current value before change
    pub value: ZString,
}

impl From<&_zend_ini_entry> for Entry {
    fn from(value: &_zend_ini_entry) -> Self {
        unsafe {
            Self {
                modified: value.modified > 0,
                name: ZString::from_raw(value.name),
                value: ZString::from_raw(value.value),
            }
        }
    }
}

unsafe extern "C" fn on_modify<T: OnModify>(
    entry: *mut _zend_ini_entry,
    new_value: *mut _zend_string,
    arg1: *mut c_void,
    _arg2: *mut c_void,
    _arg3: *mut c_void,
    stage: i32,
) -> i32 {
    let stage = match Stage::try_from(stage) {
        Ok(val) => val,
        Err(_) => return ZEND_RESULT_CODE_FAILURE,
    };

    let on_modify_item = &mut *(arg1 as *mut OnModifyCarry<T>);

    let modify = &mut on_modify_item.on_modify;

    let result = modify
        .on_modify(
            Entry::from(&*(entry as *const _zend_ini_entry)),
            ZString::from_raw(new_value),
            stage,
        )
        .map(|_| ZEND_RESULT_CODE_SUCCESS)
        .map_err(|_| ZEND_RESULT_CODE_FAILURE)
        .unwrap();

    // Prevent memory leaks
    if stage == Stage::Shutdown || stage == Stage::Deactivate {
        let _item = Box::from_raw(on_modify_item);
    }

    result
}

/// On INI Change Trait
pub trait OnModify {
    /// Called whenever INI has changed
    fn on_modify(
        &mut self,
        entry: Entry,
        new_value: ZString,
        stage: Stage,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

impl OnModify for () {
    fn on_modify(
        &mut self,
        _entry: Entry,
        _new_value: ZString,
        _stage: Stage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

struct OnModifyCarry<T>
where
    T: OnModify,
{
    on_modify: T,
}

type ZendOnModify = unsafe extern "C" fn(
    entry: *mut zend_ini_entry,
    new_value: *mut zend_string,
    mh_arg1: *mut c_void,
    mh_arg2: *mut c_void,
    mh_arg3: *mut c_void,
    stage: c_int,
) -> c_int;

pub(crate) fn create_ini_entry_ex<T>(
    name: impl AsRef<str>,
    default_value: impl AsRef<str>,
    modifiable: u32,
    on_modify_impl: Option<T>,
) -> zend_ini_entry_def
where
    T: OnModify,
{
    let name = name.as_ref();
    let default_value = default_value.as_ref();
    let (modifiable, name_length) = (modifiable as c_uchar, name.len() as u16);

    let (callback, arg): (Option<ZendOnModify>, *mut OnModifyCarry<T>) = match on_modify_impl {
        Some(callback) => (
            Some(on_modify::<T> as ZendOnModify),
            Box::into_raw(Box::new(OnModifyCarry {
                on_modify: callback,
            })),
        ),
        None => (None, null_mut()),
    };

    zend_ini_entry_def {
        name: name.as_ptr().cast(),
        name_length,
        on_modify: callback,
        mh_arg1: arg as *mut c_void,
        mh_arg2: null_mut(),
        mh_arg3: null_mut(),
        value: default_value.as_ptr().cast(),
        value_length: default_value.len() as u32,
        displayer: None,
        modifiable,
    }
}

unsafe fn entries(mut ini_entries: Vec<zend_ini_entry_def>) -> *const zend_ini_entry_def {
    ini_entries.push(zeroed::<zend_ini_entry_def>());

    Box::into_raw(ini_entries.into_boxed_slice()).cast()
}

pub(crate) fn register(ini_entries: Vec<zend_ini_entry_def>, module_number: c_int) {
    unsafe {
        zend_register_ini_entries(entries(ini_entries), module_number);
    }
}

pub(crate) fn unregister(module_number: c_int) {
    unsafe {
        zend_unregister_ini_entries(module_number);
    }
}
