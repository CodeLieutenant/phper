//! Apis relate to [zend_string](crate::sys::smart_str).

use std::{ffi::c_char, fmt::Debug, mem::MaybeUninit};

use crate::{
    strings::{ZStr, ZString},
    sys::*,
};

/// PHP String Builder for [zend_string](crate::sys::smart_str).
#[repr(transparent)]
pub struct ZSmartStr {
    inner: smart_str,
}

impl Default for ZSmartStr {
    fn default() -> Self {
        Self {
            inner: smart_str {
                s: std::ptr::null_mut(),
                a: 0,
            },
        }
    }
}

impl ZSmartStr {
    /// Creates a new smart str zero initialized.

    #[inline]
    pub fn new() -> Self {
        unsafe {
            let a = MaybeUninit::<ZSmartStr>::uninit();
            a.assume_init()
        }
    }

    /// Wraps a raw pointer.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    ///
    /// # Panics
    ///
    /// Panics if pointer is null.
    #[inline]
    pub unsafe fn from_ptr<'a>(ptr: *const smart_str) -> &'a Self {
        (ptr as *const Self).as_ref().expect("ptr should't be null")
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_ptr<'a>(ptr: *const smart_str) -> Option<&'a Self> {
        (ptr as *const Self).as_ref()
    }

    /// Wraps a raw pointer.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    ///
    /// # Panics
    ///
    /// Panics if pointer is null.
    #[inline]
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut smart_str) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should't be null")
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_mut_ptr<'a>(ptr: *mut smart_str) -> Option<&'a mut Self> {
        (ptr as *mut Self).as_mut()
    }

    /// Returns a raw pointer wrapped.
    pub const fn as_ptr(&self) -> *const smart_str {
        &self.inner
    }

    /// Returns a raw pointer wrapped.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut smart_str {
        &mut self.inner
    }

    /// Creates a new smart str with supplied allocated size.

    #[inline]
    pub fn with_capacity(n: usize) -> Self {
        let mut a = MaybeUninit::<ZSmartStr>::uninit();

        unsafe {
            phper_smart_str_alloc(a.as_mut_ptr().cast(), n, false);

            a.assume_init()
        }
    }

    /// Extends current smart_str to the new allocated size.
    #[inline]
    pub fn extend(&mut self, n: usize) -> &mut Self {
        unsafe {
            phper_smart_str_extend_ex(&mut self.inner, n, false);
        }
        self
    }

    /// Returns current size of the string (not allocated bytes).
    #[inline]
    pub fn len(&self) -> usize {
        if self.inner.s.is_null() {
            0
        } else {
            unsafe { (*self.inner.s).len }
        }
    }

    /// Checks if the smart_str is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Retuns number of bytes allocated for the smart str.
    #[inline]
    pub fn allocated_size(&self) -> usize {
        self.inner.a
    }

    /// Appends i64 into the smart str.
    #[inline]
    pub fn append_long(&mut self, num: impl Into<i64>) -> &mut Self {
        unsafe {
            phper_smart_str_append_long_ex(&mut self.inner, num.into(), false);
        }

        self
    }

    /// Appends u64 into the smart str.

    #[inline]
    pub fn append_unsigned(&mut self, num: impl Into<u64>) -> &mut Self {
        unsafe {
            phper_smart_str_append_unsigned_ex(&mut self.inner, num.into(), false);
        }

        self
    }

    /// Appends zend_string to the smart str.
    #[inline]
    pub fn append_string(&mut self, str: impl Into<*const zend_string>) -> &mut Self {
        unsafe {
            phper_smart_str_append_ex(&mut self.inner, str.into(), false);
        }

        self
    }

    /// Appends CString and escapes it.

    #[inline]
    pub fn append_string_escaped(
        &mut self,
        str: impl Into<*const c_char>,
        len: usize,
    ) -> &mut Self {
        unsafe {
            phper_smart_str_append_escaped(&mut self.inner, str.into(), len);
        }

        self
    }

    /// Appends zend_string up to the supplied length and escapes it.
    #[inline]
    pub fn append_string_escaped_truncated(
        &mut self,
        str: impl Into<*mut zend_string>,
        len: usize,
    ) -> &mut Self {
        unsafe {
            phper_smart_str_append_escaped_truncated(&mut self.inner, str.into(), len);
        }

        self
    }

    /// Appends C style char to the smart str.
    #[inline]
    pub fn append_byte(&mut self, ch: i8) -> &mut Self {
        unsafe {
            phper_smart_str_appendc_ex(&mut self.inner, ch, false);
        }

        self
    }

    /// Appends UTF-8 char to the smart str.
    #[inline]
    pub fn append_char(&mut self, ch: char) -> &mut Self {
        let mut bytes = [0u8; 4];
        ch.encode_utf8(&mut bytes);

        unsafe {
            phper_smart_str_appendl_ex(
                &mut self.inner,
                bytes.as_ptr() as *const i8,
                ch.len_utf8(),
                false,
            );
        }

        self
    }

    /// Concatanates two smart str buffers.
    #[inline]
    pub fn append_smart_str(&mut self, str: impl Into<*const smart_str>) -> &mut Self {
        unsafe {
            phper_smart_str_append_smart_str_ex(&mut self.inner, str.into(), false);
        }

        self
    }

    /// Appends float to the smart str.
    #[inline]
    pub fn append_float(
        &mut self,
        num: impl Into<f32>,
        precision: i32,
        zero_fraction: bool,
    ) -> &mut Self {
        unsafe {
            phper_smart_str_append_double(
                &mut self.inner,
                num.into() as f64,
                precision,
                zero_fraction,
            )
        }

        self
    }

    /// Adds NULL byte at the end of the buffer, making it a valid C String.
    #[inline]
    pub fn append_null_byte(&mut self) -> &mut Self {
        unsafe {
            phper_smart_str_0(&mut self.inner);
        }

        self
    }

    /// Appends double to the smart str.

    #[inline]
    pub fn append_double(
        &mut self,
        num: impl Into<f64>,
        precision: i32,
        zero_fraction: bool,
    ) -> &mut Self {
        unsafe {
            phper_smart_str_append_double(&mut self.inner, num.into(), precision, zero_fraction)
        }

        self
    }

    /// Appends any ZVAL to the smart str.

    #[inline]
    pub fn append_scalar(&mut self, num: impl Into<*const zval>, truncate: usize) -> &mut Self {
        unsafe {
            phper_smart_str_append_scalar(&mut self.inner, num.into(), truncate);
        }

        self
    }

    /// Extracts the ZendString from the smart_str builder and drops the
    /// allocated memory. Don't forget to call `append_null_byte` before
    /// build.
    #[inline]
    pub fn build(self) -> ZString {
        unsafe {
            let ptr =
                phper_zend_string_init((*self.inner.s).val.as_ptr(), self.len(), false.into());

            ZString::from_raw(ptr)
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<*const smart_str> for &ZSmartStr {
    fn into(self) -> *const smart_str {
        &self.inner
    }
}

#[allow(clippy::from_over_into)]
impl Into<*mut smart_str> for &mut ZSmartStr {
    fn into(self) -> *mut smart_str {
        &mut self.inner
    }
}

impl Clone for ZSmartStr {
    fn clone(&self) -> Self {
        let mut a = MaybeUninit::<ZSmartStr>::uninit();

        unsafe {
            phper_smart_str_append_smart_str_ex(a.as_mut_ptr().cast(), &self.inner, false);

            a.assume_init()
        }
    }
}

impl Debug for ZSmartStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let zstr = unsafe { ZStr::from_ptr(self.inner.s.cast_const()) };
        let mut d = f.debug_struct("ZSmartStr");
        d.field("str", zstr);
        d.finish()
    }
}

impl Drop for ZSmartStr {
    fn drop(&mut self) {
        unsafe {
            phper_smart_str_free_ex(&mut self.inner, false);
        }
    }
}
