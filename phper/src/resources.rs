// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [zend_resource].

use crate::sys::*;
use std::{
    ffi::c_void,
    fmt::{self, Debug},
    marker::PhantomData,
};

/// Wrapper of [zend_resource].
#[repr(transparent)]
pub struct ZRes<T> {
    inner: zend_resource,
    _data: PhantomData<T>,
}

/// Zend Persistent Resource
/// Usecase: Database Persistent Connections, HTTP Connections etc
#[repr(transparent)]
pub struct ZPersistentResource<T> {
    inner: *const zend_resource,
    _data: PhantomData<T>,
}

impl<T> ZRes<T> {
    /// Wraps a raw pointer.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    ///
    /// # Panics
    ///
    /// Panics if pointer is null.
    pub unsafe fn from_ptr<'a>(ptr: *const zend_resource) -> &'a Self {
        (ptr as *const Self)
            .as_ref()
            .expect("ptr should not be null")
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    pub unsafe fn try_from_ptr<'a>(ptr: *const zend_resource) -> Option<&'a Self> {
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
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_resource) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should not be null")
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    pub unsafe fn try_from_mut_ptr<'a>(ptr: *mut zend_resource) -> Option<&'a mut Self> {
        (ptr as *mut Self).as_mut()
    }

    /// Returns a raw pointer wrapped.
    pub const fn as_ptr(&self) -> *const zend_resource {
        &self.inner
    }

    /// Returns a raw pointer wrapped.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_resource {
        &mut self.inner
    }

    /// Gets the resource handle.
    #[allow(clippy::useless_conversion)]
    pub fn handle(&self) -> i64 {
        self.inner.handle.into()
    }

    /// Casts a zend_resource.ptr to &mut T
    pub fn value_mut(&mut self) -> &mut T {
        unsafe { &mut *(self.inner.ptr as *mut T) }
    }

    /// Casts a zend_resource.ptr to &T
    pub fn value(&self) -> &T {
        unsafe { &*(self.inner.ptr as *const T) }
    }
}

impl<T> Debug for ZRes<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ZRes")
            .field("handle", &self.handle())
            .finish()
    }
}

extern "C" fn clean_persistent_resource<T, Fn>(res: *mut zend_resource)
where
    Fn: FnOnce(&mut T),
{
    let mut x = unsafe { Box::from_raw((*res).ptr as *mut PersistentResourceCleanup<T, Fn>) };

    if let Some(cleanup) = x.cleanup {
        cleanup(&mut x.value)
    }
}

struct PersistentResourceCleanup<T, TFn>
where
    TFn: FnOnce(&mut T),
{
    value: T,
    cleanup: Option<TFn>,
}

impl<T> ZPersistentResource<T> {
    /// Creates new Zend Persistent Resource
    pub fn new(hash: impl Into<crate::strings::ZString>, name: &'static str, value: T) -> Self {
        Self::new_with_cleanup(hash, name, value, Option::<Box<dyn FnOnce(&mut T)>>::None)
    }

    /// Creates new Zend Persistent Resource with Cleanup function
    pub fn new_with_cleanup<Fn: FnOnce(&mut T)>(
        hash: impl Into<crate::strings::ZString>,
        name: &'static str,
        value: T,
        cleanup: Option<Fn>,
    ) -> Self {
        let resource = unsafe {
            let id: i32 = phper_zend_register_persistent_list_destructors(
                Some(clean_persistent_resource::<T, Fn>),
                name.as_ptr() as *const i8,
                crate::modules::GLOBAL_MODULE_NUMBER,
            );
            let hash = hash.into().into_raw();
            let boxed = Box::into_raw(Box::new(PersistentResourceCleanup { value, cleanup }));
            phper_register_persistent_resource(hash, boxed as *mut c_void, id)
                as *const zend_resource
        };

        Self {
            inner: resource,
            _data: Default::default(),
        }
    }

    /// Find zend resource by resource name
    pub fn find<'a>(hash: impl AsRef<&'a str>) -> Option<Self> {
        // TODO: Should be nice if we checked the ID for the TYPE
        let resource = unsafe {
            let hash: &'a str = hash.as_ref();
            phper_register_persistent_find(hash.as_ptr() as *const i8, hash.len())
        };

        if !resource.is_null() {
            Some(Self {
                inner: resource,
                _data: Default::default(),
            })
        } else {
            None
        }
    }

    /// Extract value from the Resource
    pub fn value(&self) -> &T {
        unsafe {
            let value = &*((*self.inner).ptr
                as *const PersistentResourceCleanup<T, Box<dyn FnOnce(&mut T)>>);

            &value.value
        }
    }
}
