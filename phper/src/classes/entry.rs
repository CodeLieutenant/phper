//! Apis relate to [zend_class_entry].

use crate::{
    arrays::ZArr,
    errors::{ClassNotFoundError, InitializeObjectError},
    objects::ZObject,
    strings::ZStr,
    sys::*,
    values::ZVal,
};
use std::{
    borrow::ToOwned,
    fmt::Debug,
    marker::PhantomData,
    mem::{replace, ManuallyDrop},
};

use super::find_global_class_entry_ptr;

/// Wrapper of [zend_class_entry].
#[repr(transparent)]
pub struct ClassEntry {
    inner: zend_class_entry,
    _p: PhantomData<*mut zend_class_entry>,
}

impl ClassEntry {
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
    pub unsafe fn from_ptr<'a>(ptr: *const zend_class_entry) -> &'a Self {
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
    pub unsafe fn try_from_ptr<'a>(ptr: *const zend_class_entry) -> Option<&'a Self> {
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
    pub unsafe fn from_mut_ptr<'a>(ptr: *mut zend_class_entry) -> &'a mut Self {
        (ptr as *mut Self).as_mut().expect("ptr should't be null")
    }

    /// Wraps a raw pointer, return None if pointer is null.
    ///
    /// # Safety
    ///
    /// Create from raw pointer.
    #[inline]
    pub unsafe fn try_from_mut_ptr<'a>(ptr: *mut zend_class_entry) -> Option<&'a mut Self> {
        (ptr as *mut Self).as_mut()
    }

    /// Returns a raw pointer wrapped.
    #[inline]
    pub const fn as_ptr(&self) -> *const zend_class_entry {
        &self.inner
    }

    /// Returns a raw pointer wrapped.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut zend_class_entry {
        &mut self.inner
    }

    /// Create reference from global class name.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use phper::classes::ClassEntry;
    ///
    /// let std_class = ClassEntry::from_globals("stdClass").unwrap();
    /// let _obj = std_class.new_object([]).unwrap();
    /// ```
    #[inline]
    pub fn from_globals(class_name: impl AsRef<str>) -> crate::Result<&'static Self> {
        let name = class_name.as_ref();
        let ptr: *mut Self = find_global_class_entry_ptr(name).cast();
        unsafe {
            ptr.as_ref().ok_or_else(|| {
                crate::Error::ClassNotFound(ClassNotFoundError::new(name.to_string()))
            })
        }
    }

    /// Create the object from class and call `__construct` with arguments.
    ///
    /// If the `__construct` is private, or protected and the called scope isn't
    /// parent class, it will throw PHP Error.
    #[inline]
    pub fn new_object(&self, arguments: impl AsMut<[ZVal]>) -> crate::Result<ZObject> {
        let mut object = self.init_object()?;
        object.call_construct(arguments)?;
        Ok(object)
    }

    /// Create the object from class, without calling `__construct`.
    ///
    /// **Be careful when `__construct` is necessary.**
    #[inline]
    pub fn init_object(&self) -> crate::Result<ZObject> {
        unsafe {
            let ptr = self.as_ptr() as *mut _;
            let mut val = ZVal::default();
            if !phper_object_init_ex(val.as_mut_ptr(), ptr) {
                Err(InitializeObjectError::new(self.get_name().to_str()?.to_owned()).into())
            } else {
                // Can't drop val here! Otherwise, the object will be dropped too (wasting me a
                // day of debugging time here).
                let mut val = ManuallyDrop::new(val);
                let ptr = phper_z_obj_p(val.as_mut_ptr());
                Ok(ZObject::from_raw(ptr.cast_mut()))
            }
        }
    }

    /// Get the class name.
    #[inline]
    pub fn get_name(&self) -> &ZStr {
        unsafe { ZStr::from_ptr(self.inner.name) }
    }

    #[inline]
    pub fn get_name_str(&self) -> &str {
        unsafe { ZStr::from_ptr(self.inner.name).as_str() }
    }

    /// Detect if the method is exists in class.
    #[inline]
    pub fn has_method(&self, method_name: &str) -> bool {
        unsafe {
            let function_table = ZArr::from_ptr(&self.inner.function_table);
            function_table.exists(method_name)
        }
    }

    /// Detect if the class is instance of parent class.
    #[inline]
    pub fn is_instance_of(&self, parent: &ClassEntry) -> bool {
        unsafe { phper_instanceof_function(self.as_ptr(), parent.as_ptr()) }
    }

    /// Get the static property by name of class.
    ///
    /// Return None when static property hasn't register by
    /// [ClassEntity::add_static_property].
    #[inline]
    pub fn get_static_property(&self, name: impl AsRef<str>) -> Option<&ZVal> {
        let ptr = self.as_ptr() as *mut _;
        let prop = Self::inner_get_static_property(ptr, name);
        unsafe { ZVal::try_from_ptr(prop) }
    }

    /// Set the static property by name of class.
    ///
    /// Return `Some(x)` where `x` is the previous value of static property, or
    /// return `None` when static property hasn't register by
    /// [ClassEntity::add_static_property].
    #[inline]
    pub fn set_static_property(&self, name: impl AsRef<str>, val: impl Into<ZVal>) -> Option<ZVal> {
        let ptr = self.as_ptr() as *mut _;
        let prop = Self::inner_get_static_property(ptr, name);
        let prop = unsafe { ZVal::try_from_mut_ptr(prop) };
        prop.map(|prop| replace(prop, val.into()))
    }

    #[inline]
    fn inner_get_static_property(scope: *mut zend_class_entry, name: impl AsRef<str>) -> *mut zval {
        let name = name.as_ref();

        unsafe { zend_read_static_property(scope, name.as_ptr().cast(), name.len(), true.into()) }
    }
}

impl Debug for ClassEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ClassEntry")
            .field(&self.get_name().to_c_str())
            .finish()
    }
}
