use derive_more::From;
use phper_sys::{
    zend_array, zend_hash_get_current_data_ex, zend_hash_get_current_key_ex,
    zend_hash_internal_pointer_reset_ex, zend_hash_move_forward_ex, zend_string, zend_ulong, zval,
    HashPosition, HASH_KEY_IS_LONG, HASH_KEY_IS_STRING, ZEND_RESULT_CODE_FAILURE,
};
use std::marker::PhantomData;
use std::ptr::null_mut;

use crate::arrays::ZArr;
use crate::strings::ZStr;
use crate::values::ZVal;

/// Iterator key for [`ZArr::iter`] and [`ZArr::iter_mut`].
#[derive(Debug, Clone, PartialEq, From)]
pub enum IterKey<'a> {
    /// Index type iterator key.
    Index(u64),
    /// String type iterator key.
    ZStr(&'a ZStr),
}

/// An iterator over the elements of a `ZArr`.
///
/// This is created by [`iter`].
///
/// [`iter`]: ZArr::iter
pub struct Iter<'a>(RawIter<'a>);

impl<'a> Iter<'a> {
    pub(super) fn new(arr: &'a ZArr) -> Self {
        Self(RawIter::new(arr.as_ptr() as *mut _))
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (IterKey<'a>, &'a ZVal);

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|(key, val)| (key, unsafe { ZVal::from_ptr(val) }))
    }
}

/// An mutable iterator over the elements of a `ZArr`.
///
/// This is created by [`iter_mut`].
///
/// [`iter_mut`]: ZArr::iter_mut
pub struct IterMut<'a>(RawIter<'a>);

impl<'a> IterMut<'a> {
    pub(super) fn new(arr: &'a mut ZArr) -> Self {
        Self(RawIter::new(arr.as_mut_ptr()))
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = (IterKey<'a>, &'a mut ZVal);

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|(key, val)| (key, unsafe { ZVal::from_mut_ptr(val) }))
    }
}

struct RawIter<'a> {
    arr: *mut zend_array,
    pos: HashPosition,
    finished: bool,
    _p: PhantomData<&'a ()>,
}

impl<'a> RawIter<'a> {
    fn new(arr: *mut zend_array) -> Self {
        let mut pos: HashPosition = 0;
        unsafe {
            zend_hash_internal_pointer_reset_ex(arr, &mut pos);
        }

        Self {
            arr,
            pos,
            finished: false,
            _p: PhantomData,
        }
    }
}

impl<'a> Iterator for RawIter<'a> {
    type Item = (IterKey<'a>, *mut zval);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.finished {
                return None;
            }

            let mut str_index: *mut zend_string = null_mut();
            let mut num_index: zend_ulong = 0;

            let result =
                zend_hash_get_current_key_ex(self.arr, &mut str_index, &mut num_index, &self.pos)
                    as u32;

            let iter_key = if result == HASH_KEY_IS_STRING {
                IterKey::ZStr(ZStr::from_mut_ptr(str_index))
            } else if result == HASH_KEY_IS_LONG {
                IterKey::Index(num_index as u64)
            } else {
                self.finished = true;
                return None;
            };

            let val = zend_hash_get_current_data_ex(self.arr, &mut self.pos);
            if val.is_null() {
                self.finished = true;
                return None;
            }

            if zend_hash_move_forward_ex(self.arr, &mut self.pos) == ZEND_RESULT_CODE_FAILURE {
                self.finished = true;
            }

            Some((iter_key, val))
        }
    }
}
