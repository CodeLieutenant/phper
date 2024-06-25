use crate::arrays::entry::{Entry, OccupiedEntry, VacantEntry};
use crate::arrays::zarray::ZArray;
use crate::arrays::{common_fmt, InsertKey, Iter, IterMut, Key};
use crate::values::ZVal;
use phper_alloc::ToRefOwned;
use phper_sys::{
    phper_z_addref_p, phper_zend_array_dup, phper_zend_hash_index_del,
    phper_zend_hash_index_exists, phper_zend_hash_index_find, phper_zend_hash_index_update,
    phper_zend_hash_next_index_insert, phper_zend_str_del, phper_zend_str_exists,
    phper_zend_str_find, phper_zend_str_update, phper_zval_arr, zend_array, zend_array_count,
};
use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;

/// Wrapper of [zend_array].
#[repr(transparent)]
pub struct ZArr {
    inner: zend_array,
    _p: PhantomData<*mut ()>,
}

impl ZArr {
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
    pub unsafe fn from_ptr<'a>(ptr: *const zend_array) -> &'a Self {
        (ptr as *const Self)
            .as_ref()
            .expect("ptr shouldn't be null")
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_ptr<'a>(ptr: *const zend_array) -> Option<&'a Self> {
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
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_array) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should't be null")
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_mut_ptr<'a>(ptr: *mut zend_array) -> Option<&'a mut Self> {
        (ptr as *mut Self).as_mut()
    }

    /// Returns a raw pointer wrapped.
    #[inline]
    pub const fn as_ptr(&self) -> *const zend_array {
        &self.inner
    }

    /// Returns a raw pointer wrapped.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_array {
        &mut self.inner
    }

    /// Returns true if the array has a length of 0.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get array items length.
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { zend_array_count(self.as_ptr() as *mut _) as usize }
    }

    /// Add or update item by key.
    pub fn insert<'a>(&mut self, key: impl Into<InsertKey<'a>>, value: impl Into<ZVal>) {
        let key = key.into();
        let mut value = ManuallyDrop::new(value.into());
        let val = value.as_mut_ptr();

        unsafe {
            match key {
                InsertKey::NextIndex => {
                    phper_zend_hash_next_index_insert(self.as_mut_ptr(), val);
                }
                InsertKey::Index(i) => {
                    phper_zend_hash_index_update(self.as_mut_ptr(), i, val);
                }
                InsertKey::Str(s) => {
                    phper_zend_str_update(
                        self.as_mut_ptr(),
                        s.as_ptr().cast(),
                        s.len(),
                        val,
                    );
                }
                InsertKey::Bytes(b) => {
                    phper_zend_str_update(
                        self.as_mut_ptr(),
                        b.as_ptr().cast(),
                        b.len(),
                        val,
                    );
                }
                InsertKey::ZStr(s) => {
                    phper_zend_str_update(
                        self.as_mut_ptr(),
                        s.as_c_str_ptr().cast(),
                        s.len(),
                        val,
                    );
                }
            }
        }
    }

    /// Get item by key.
    #[inline]
    pub fn get<'a>(&self, key: impl Into<Key<'a>>) -> Option<&'a ZVal> {
        self.inner_get(key).map(|v| &*v)
    }

    /// Get item by key.
    #[inline]
    pub fn get_mut<'a>(&mut self, key: impl Into<Key<'a>>) -> Option<&'a mut ZVal> {
        self.inner_get(key)
    }

    fn inner_get<'a>(&self, key: impl Into<Key<'a>>) -> Option<&'a mut ZVal> {
        let key = key.into();
        let ptr = self.as_ptr() as *mut _;
        unsafe {
            let value = match key {
                Key::Index(i) => phper_zend_hash_index_find(ptr, i),
                Key::Str(s) => {
                    phper_zend_str_find(ptr, s.as_ptr().cast(), s.len())
                }
                Key::Bytes(b) => {
                    phper_zend_str_find(ptr, b.as_ptr().cast(), b.len())
                }
                Key::ZStr(s) => {
                    phper_zend_str_find(ptr, s.as_c_str_ptr(), s.len())
                }
            };
            if value.is_null() {
                None
            } else {
                Some(ZVal::from_mut_ptr(value))
            }
        }
    }

    /// Check if the key exists.
    pub fn exists<'a>(&self, key: impl Into<Key<'a>>) -> bool {
        let key = key.into();
        let ptr = self.as_ptr() as *mut _;
        unsafe {
            match key {
                Key::Index(i) => phper_zend_hash_index_exists(ptr, i),
                Key::Str(s) => {
                    phper_zend_str_exists(ptr, s.as_ptr().cast(), s.len())
                }
                Key::Bytes(b) => {
                    phper_zend_str_exists(ptr, b.as_ptr().cast(), b.len())
                }
                Key::ZStr(s) => phper_zend_str_exists(
                    ptr,
                    s.to_bytes().as_ptr().cast(),
                    s.len(),
                ),
            }
        }
    }

    /// Remove the item under the key
    pub fn remove<'a>(&mut self, key: impl Into<Key<'a>>) -> bool {
        let key = key.into();
        unsafe {
            match key {
                Key::Index(i) => phper_zend_hash_index_del(&mut self.inner, i),
                Key::Str(s) => phper_zend_str_del(
                    &mut self.inner,
                    s.as_ptr().cast(),
                    s.len(),
                ),
                Key::Bytes(b) => phper_zend_str_del(
                    &mut self.inner,
                    b.as_ptr().cast(),
                    b.len(),
                ),
                Key::ZStr(s) => phper_zend_str_del(
                    &mut self.inner,
                    s.as_c_str_ptr().cast(),
                    s.len(),
                ),
            }
        }
    }

    /// Gets the given key’s corresponding entry in the array for in-place
    /// manipulation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use phper::arrays::ZArray;
    ///
    /// let mut arr = ZArray::new();
    ///
    /// // count the number of occurrences of letters in the vec
    /// for x in ["a", "b", "a", "c", "a", "b"] {
    ///     arr.entry(x)
    ///         .and_modify(|cur| *cur.as_mut_long().unwrap() += 1)
    ///         .or_insert(1);
    /// }
    /// ```
    #[inline]
    pub fn entry<'a>(&'a mut self, key: impl Into<Key<'a>>) -> Entry<'a> {
        let key = key.into();
        match self.get_mut(key.clone()) {
            Some(val) => Entry::Occupied(OccupiedEntry(val)),
            None => Entry::Vacant(VacantEntry { arr: self, key }),
        }
    }

    /// Provides a forward iterator.
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self)
    }

    /// Provides a forward iterator with mutable references.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut::new(self)
    }

    /// Gets Rust slice from PHP Packed Array
    ///
    /// # Safety:
    ///     Called must be sure that array is PACKED
    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn as_slice(&self) -> &[ZVal] {
        std::slice::from_raw_parts(
            self.inner.__bindgen_anon_1.arPacked as *const ZVal,
            self.len(),
        )
    }

    /// Gets Mutable Rust slice from PHP Packed Array
    ///
    /// # Safety:
    ///     Called must be sure that array is PACKED
    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn as_mut_slice(&mut self) -> &mut [ZVal] {
        std::slice::from_raw_parts_mut(
            self.inner.__bindgen_anon_1.arPacked as *mut ZVal,
            self.len(),
        )
    }
}

impl Debug for ZArr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        common_fmt(self, f, "ZArr")
    }
}

impl ToOwned for ZArr {
    type Owned = ZArray;

    fn to_owned(&self) -> Self::Owned {
        unsafe {
            let dest = phper_zend_array_dup(self.as_ptr() as *mut _);
            ZArray::from_raw(dest)
        }
    }
}

impl ToRefOwned for ZArr {
    type Owned = ZArray;

    fn to_ref_owned(&mut self) -> Self::Owned {
        let mut val = ManuallyDrop::new(ZVal::default());
        unsafe {
            phper_zval_arr(val.as_mut_ptr(), self.as_mut_ptr());
            phper_z_addref_p(val.as_mut_ptr());
            ZArray::from_raw(val.as_mut_z_arr().unwrap().as_mut_ptr())
        }
    }
}
