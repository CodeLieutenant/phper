use phper_sys::zend_function_entry;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

pub(super) struct FEntry(pub(super) Vec<zend_function_entry>);

unsafe impl Send for FEntry {}
unsafe impl Sync for FEntry {}

impl FEntry {
    pub(super) const fn new_const() -> Self {
        Self(Vec::new())
    }

    pub(super) unsafe fn from_zend_functions(fns: *mut zend_function_entry, size: usize) -> Self {
        FEntry(Vec::from_raw_parts(fns, size, size))
    }
}

impl Deref for FEntry {
    type Target = Vec<zend_function_entry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Safety: This is used as a global variable, initialization is always
/// guaranteed by PHP to be from one thread in ZTS, and on NTS its always one thread
pub(super) static mut PHP_FUNCTIONS: ManuallyDrop<FEntry> = ManuallyDrop::new(FEntry::new_const());
