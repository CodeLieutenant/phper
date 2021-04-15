use std::{
    borrow::Cow,
    cell::Cell,
    ffi::{c_void, CStr},
    mem::zeroed,
    os::raw::{c_char, c_int},
    ptr::null_mut,
    slice, str,
};
use crate::{
    c_str_ptr,
    classes::{This, Visibility},
    errors::Throwable,
    sys::*,
};
use std::{slice::from_raw_parts, sync::atomic::Ordering};
use crate::arrays::Array;

#[repr(transparent)]
pub struct ExecuteData {
    inner: zend_execute_data,
}

impl ExecuteData {
    #[inline]
    pub const fn new(inner: zend_execute_data) -> Self {
        Self { inner }
    }

    pub unsafe fn from_mut<'a>(ptr: *mut zend_execute_data) -> &'a mut Self {
        &mut *(ptr as *mut Self)
    }

    pub fn as_mut(&mut self) -> *mut zend_execute_data {
        &mut self.inner
    }

    #[inline]
    pub unsafe fn common_num_args(&self) -> u32 {
        (*self.inner.func).common.num_args
    }

    #[inline]
    pub unsafe fn common_required_num_args(&self) -> u32 {
        (*self.inner.func).common.required_num_args
    }

    #[inline]
    pub unsafe fn common_arg_info(&self) -> *mut zend_arg_info {
        (*self.inner.func).common.arg_info
    }

    #[inline]
    pub unsafe fn num_args(&self) -> u32 {
        self.inner.This.u2.num_args
    }

    #[inline]
    pub unsafe fn get_this(&mut self) -> &mut This {
        This::from_mut(phper_get_this(&mut self.inner))
    }

    pub unsafe fn get_parameters_array(&mut self) -> Vec<Val> {
        let num_args = self.num_args();
        let mut arguments = vec![zeroed::<zval>(); num_args as usize];
        _zend_get_parameters_array_ex(num_args as c_int, arguments.as_mut_ptr());
        arguments.into_iter().map(Val::new).collect()
    }
}

#[repr(transparent)]
pub struct Val {
    inner: zval,
}

impl Val {
    #[inline]
    pub const fn new(inner: zval) -> Self {
        Self { inner }
    }

    pub fn null() -> Self {
        let mut val = Self {
            inner: unsafe { zeroed::<zval>() },
        };
        ().set_val(&mut val);
        val
    }

    #[inline]
    pub unsafe fn from_mut<'a>(ptr: *mut zval) -> &'a mut Self {
        assert!(!ptr.is_null(), "ptr should not be null");
        &mut *(ptr as *mut Self)
    }

    pub fn as_mut(&mut self) -> *mut zval {
        &mut self.inner
    }

    unsafe fn type_info(&mut self) -> &mut u32 {
        &mut self.inner.u1.type_info
    }

    pub fn as_string(&mut self) -> String {
        unsafe {
            let s = phper_zval_get_string(&mut self.inner);
            let buf = from_raw_parts(&(*s).val as *const i8 as *const u8, (*s).len);
            phper_zend_string_release(s);
            str::from_utf8(buf).unwrap().to_string()
        }
    }

    pub fn as_i64(&mut self) -> i64 {
        unsafe { phper_zval_get_long(&mut self.inner) }
    }
}

pub trait SetVal {
    fn set_val(self, val: &mut Val);
}

impl SetVal for () {
    fn set_val(self, val: &mut Val) {
        unsafe {
            *val.type_info() = IS_NULL;
        }
    }
}

impl SetVal for bool {
    fn set_val(self, val: &mut Val) {
        unsafe {
            *val.type_info() = if self { IS_TRUE } else { IS_FALSE };
        }
    }
}

impl SetVal for i32 {
    fn set_val(self, val: &mut Val) {
        (self as i64).set_val(val)
    }
}

impl SetVal for u32 {
    fn set_val(self, val: &mut Val) {
        (self as i64).set_val(val)
    }
}

impl SetVal for i64 {
    fn set_val(self, val: &mut Val) {
        unsafe {
            (*val.as_mut()).value.lval = self;
            (*val.as_mut()).u1.type_info = IS_LONG;
        }
    }
}

impl SetVal for f64 {
    fn set_val(self, val: &mut Val) {
        unsafe {
            (*val.as_mut()).value.dval = self;
            (*val.as_mut()).u1.type_info = IS_DOUBLE;
        }
    }
}

impl SetVal for &str {
    fn set_val(self, val: &mut Val) {
        unsafe {
            phper_zval_stringl(val.as_mut(), self.as_ptr().cast(), self.len());
        }
    }
}

impl SetVal for String {
    fn set_val(self, val: &mut Val) {
        unsafe {
            phper_zval_stringl(val.as_mut(), self.as_ptr().cast(), self.len());
        }
    }
}

impl SetVal for Array {
    fn set_val(mut self, val: &mut Val) {
        unsafe {
            phper_zval_arr(val.as_mut(), self.as_mut());
        }
    }
}

impl<T: SetVal> SetVal for Option<T> {
    fn set_val(self, val: &mut Val) {
        match self {
            Some(t) => t.set_val(val),
            None => ().set_val(val),
        }
    }
}

impl<T: SetVal, E: Throwable> SetVal for Result<T, E> {
    fn set_val(self, val: &mut Val) {
        match self {
            Ok(t) => t.set_val(val),
            Err(e) => unsafe {
                let class = e
                    .class_entity()
                    .as_ref()
                    .expect("class entry is null pointer");
                let mut message = e.to_string();
                message.push('\0');
                zend_throw_exception(
                    class.entry.load(Ordering::SeqCst).cast(),
                    message.as_ptr().cast(),
                    e.code() as i64,
                );
            },
        }
    }
}

impl SetVal for Val {
    fn set_val(mut self, val: &mut Val) {
        unsafe {
            phper_zval_zval(val.as_mut(), self.as_mut(), 1, 0);
        }
    }
}
