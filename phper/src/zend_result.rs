use phper_sys::{ZEND_RESULT_CODE, ZEND_RESULT_CODE_FAILURE, ZEND_RESULT_CODE_SUCCESS};
use std::ffi::c_int;

#[repr(i32)]
pub enum ZResult {
    Success = ZEND_RESULT_CODE_SUCCESS,
    Failure = ZEND_RESULT_CODE_FAILURE,
}

impl Into<c_int> for ZResult {
    fn into(self) -> ZEND_RESULT_CODE {
        self as c_int
    }
}
