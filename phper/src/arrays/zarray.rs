use crate::arrays::{common_fmt, ZArr};
use crate::values::ZVal;
use phper_sys::{phper_z_arr_p, phper_zend_new_array, zend_array, zend_array_destroy};
use std::borrow::Borrow;
use std::fmt;
use std::fmt::Debug;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

/// Wrapper of [zend_array].
#[repr(transparent)]
pub struct ZArray(*mut ZArr);

impl ZArray {
    /// Creates an empty `ZArray`.
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Creates an empty `ZArray` with at least the specified capacity.
    ///
    /// Note that the actual capacity is always a power of two, so if you have
    /// 12 elements in a hashtable the actual table capacity will be 16.
    pub fn with_capacity(n: usize) -> Self {
        unsafe {
            let ptr = phper_zend_new_array(n.try_into().unwrap());
            Self::from_raw(ptr)
        }
    }

    /// Create owned object From raw pointer, usually used in pairs with
    /// `into_raw`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to memory
    /// problems. For example, a double-free may occur if the function is called
    /// twice on the same raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut zend_array) -> Self {
        Self(ZArr::from_mut_ptr(ptr))
    }

    /// Consumes the `ZArray` and transfers ownership to a raw pointer.
    ///
    /// Failure to call [`ZArray::from_raw`] will lead to a memory leak.
    #[inline]
    pub fn into_raw(self) -> *mut zend_array {
        ManuallyDrop::new(self).as_mut_ptr()
    }

    pub(crate) unsafe fn from_zval(value: ZVal) -> Self {
        let value = ManuallyDrop::new(value);
        ZArray::from_raw(phper_z_arr_p(value.as_ptr()) as *mut zend_array)
    }

    pub unsafe fn as_slice(&self) -> &[ZVal] {
        unsafe { &*self.0 }.as_slice()
    }

    pub unsafe fn as_mut_slice(&mut self) -> &mut [ZVal] {
        unsafe { &mut *self.0 }.as_mut_slice()
    }
}

impl Debug for ZArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        common_fmt(self, f, "ZArray")
    }
}

impl Default for ZArray {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for ZArray {
    type Target = ZArr;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref().unwrap() }
    }
}

impl DerefMut for ZArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut().unwrap() }
    }
}

impl Borrow<ZArr> for ZArray {
    fn borrow(&self) -> &ZArr {
        self.deref()
    }
}

impl Clone for ZArray {
    fn clone(&self) -> Self {
        self.deref().to_owned()
    }
}

impl Drop for ZArray {
    fn drop(&mut self) {
        unsafe {
            zend_array_destroy(self.as_mut_ptr());
        }
    }
}
