use crate::errors::{throw, ArgumentCountError};
use crate::values::{ExecuteData, ZVal};
use phper_sys::{zend_execute_data, zval};

/// The entry for all registered PHP functions.
pub(super) unsafe extern "C" fn call_function_handler(
    execute_data: *mut zend_execute_data,
    return_value: *mut zval,
) {
    let execute_data = ExecuteData::from_mut_ptr(execute_data);
    let return_value = ZVal::from_mut_ptr(return_value);

    // Check arguments count.
    let num_args = execute_data.num_args();
    let required_num_args = execute_data.common_required_num_args();
    if num_args < required_num_args {
        let func_name = execute_data.func().get_function_or_method_name();
        let err: crate::Error = match func_name.to_str() {
            Ok(func_name) => {
                ArgumentCountError::new(func_name.to_owned(), required_num_args, num_args).into()
            }
            Err(e) => e.into(),
        };
        throw(err);
        *return_value = ().into();
        return;
    }

    let handler = execute_data.get_handler();
    if let Some(mut params) = execute_data.get_parameters_array() {
        handler.call(execute_data, params.as_mut_slice(), return_value);
    } else {
        let func_name = execute_data.func().get_function_or_method_name();
        let err: crate::Error = match func_name.to_str() {
            Ok(func_name) => {
                ArgumentCountError::new(func_name.to_owned(), required_num_args, num_args).into()
            }
            Err(e) => e.into(),
        };
        throw(err);
    }
}
