use crate::classes::RawVisibility;
use crate::functions::{Callable, FunctionEntry};
use crate::utils::ensure_end_with_zero;
use phper_sys::{
    zend_internal_arg_info, ZEND_ACC_ABSTRACT, ZEND_ACC_FINAL, ZEND_ACC_PRIVATE,
    ZEND_ACC_PROTECTED, ZEND_ACC_PUBLIC,
};
use std::ffi::CString;
use std::rc::Rc;

/// Builder for registering class method.
pub struct MethodEntity {
    pub(crate) name: CString,
    pub(crate) handler: Option<Rc<dyn Callable>>,
    pub(crate) arguments: &'static [zend_internal_arg_info],
    pub(crate) visibility: RawVisibility,
}

#[derive(Default)]
pub struct MethodEntityBuilder {
    name: CString,
    handler: Option<Rc<dyn Callable>>,
    arguments: &'static [zend_internal_arg_info],
    visibility: RawVisibility,
}
#[allow(clippy::from_over_into)]
impl Into<FunctionEntry> for MethodEntity {
    fn into(self) -> FunctionEntry {
        unsafe { FunctionEntry::from_method_entity(self) }
    }
}

impl MethodEntityBuilder {
    #[inline]
    fn new(name: impl AsRef<str>, arguments: &'static [zend_internal_arg_info]) -> Self {
        Self {
            name: ensure_end_with_zero(name),
            handler: None,
            arguments,
            visibility: ZEND_ACC_PUBLIC,
        }
    }

    #[inline]
    pub fn set_handler(mut self, handler: impl Into<Rc<dyn Callable>>) -> Self {
        self.handler = Some(handler.into());
        self
    }

    #[inline]
    pub fn set_final(mut self) -> Self {
        self.visibility |= ZEND_ACC_FINAL;
        self
    }

    #[inline]
    pub fn set_abstract(mut self) -> Self {
        self.visibility |= ZEND_ACC_ABSTRACT;
        self
    }

    #[inline]
    pub fn set_private(mut self) -> Self {
        self.visibility |= ZEND_ACC_PRIVATE;
        self
    }

    #[inline]
    pub fn set_protected(mut self) -> Self {
        self.visibility |= ZEND_ACC_PROTECTED;
        self
    }

    #[inline]
    pub fn set_public(mut self) -> Self {
        self.visibility |= ZEND_ACC_PUBLIC;
        self
    }

    pub(crate) fn build(self) -> MethodEntity {
        MethodEntity {
            name: self.name,
            handler: self.handler,
            visibility: self.visibility,
            arguments: self.arguments,
        }
    }
}
