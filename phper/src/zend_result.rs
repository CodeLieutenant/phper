use crate::sys::{ZEND_RESULT_CODE_FAILURE, ZEND_RESULT_CODE_SUCCESS};
use std::ffi::c_int;

#[repr(i32)]
pub enum ZResult {
    Success = ZEND_RESULT_CODE_SUCCESS,
    Failure = ZEND_RESULT_CODE_FAILURE,
}

impl From<ZResult> for c_int {
    fn from(val: ZResult) -> Self {
        val as c_int
    }
}
