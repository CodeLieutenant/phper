use phper_sys::{
    zend_ce_aggregate, zend_ce_arrayaccess, zend_ce_countable, zend_ce_iterator,
    zend_ce_serializable, zend_ce_stringable, zend_ce_traversable,
};

use super::entry::ClassEntry;

/// Predefined interface `Iterator`.
#[inline]
pub fn iterator_interface<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_iterator) }
}

/// Predefined interface `ArrayAccess`.
#[inline]
pub fn array_access_interface<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_arrayaccess) }
}

/// Predefined interface `Traversable`.
#[inline]
pub fn traversable_interface<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_traversable) }
}

/// Predefined interface `Aggregate`.
#[inline]
pub fn aggregate_interface<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_aggregate) }
}

/// Predefined interface `Serializable`.
#[inline]
pub fn serializable_interface<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_serializable) }
}

/// Predefined interface `Countable`.
#[inline]
pub fn countable_interface<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_countable) }
}

/// Predefined interface `Stringable`.
#[inline]
pub fn stringable_interface<'a>() -> &'a ClassEntry {
    unsafe { ClassEntry::from_ptr(zend_ce_stringable) }
}
