use crate::classes::RawVisibility;
use crate::types::Scalar;
use phper_sys::{
    zend_class_entry, zend_declare_property_bool, zend_declare_property_double,
    zend_declare_property_long, zend_declare_property_null, zend_declare_property_stringl,
    zend_long, ZEND_ACC_PRIVATE, ZEND_ACC_PROTECTED, ZEND_ACC_PUBLIC, ZEND_ACC_STATIC,
};

/// Builder for declare class property.
struct PropertyEntity {
    name: String,
    visibility: RawVisibility,
    value: Scalar,
}

impl PropertyEntity {
    fn new(name: impl Into<String>, visibility: Visibility, value: impl Into<Scalar>) -> Self {
        Self {
            name: name.into(),
            visibility: visibility as RawVisibility,
            value: value.into(),
        }
    }

    #[inline]
    pub(crate) fn set_vis_static(&mut self) -> &mut Self {
        self.visibility |= ZEND_ACC_STATIC;
        self
    }

    pub(crate) fn declare(&self, ce: *mut zend_class_entry) {
        let name = self.name.as_ptr().cast();
        let name_length = self.name.len();
        let access_type = self.visibility as i32;

        unsafe {
            match &self.value {
                Scalar::Null => {
                    zend_declare_property_null(ce, name, name_length, access_type);
                }
                Scalar::Bool(b) => {
                    zend_declare_property_bool(ce, name, name_length, *b as zend_long, access_type);
                }
                Scalar::I64(i) => {
                    zend_declare_property_long(ce, name, name_length, *i, access_type);
                }
                Scalar::F64(f) => {
                    zend_declare_property_double(ce, name, name_length, *f, access_type);
                }
                Scalar::String(s) => {
                    // If the `ce` is `ZEND_INTERNAL_CLASS`, then the `zend_string` is allocated
                    // as persistent.
                    zend_declare_property_stringl(
                        ce,
                        name,
                        name_length,
                        s.as_ptr().cast(),
                        s.len(),
                        access_type,
                    );
                }
                Scalar::Bytes(b) => {
                    zend_declare_property_stringl(
                        ce,
                        name,
                        name_length,
                        b.as_ptr().cast(),
                        b.len(),
                        access_type,
                    );
                }
            }
        }
    }
}

/// Visibility of class properties and methods.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum Visibility {
    /// Public.
    #[default]
    Public = ZEND_ACC_PUBLIC,

    /// Protected.
    Protected = ZEND_ACC_PROTECTED,

    /// Private.
    Private = ZEND_ACC_PRIVATE,
}
