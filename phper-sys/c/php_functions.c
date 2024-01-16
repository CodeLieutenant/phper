#include <phper.h>

zend_string *phper_get_function_or_method_name(const zend_function *func) {
    return get_function_or_method_name(func);
}

zend_string *phper_get_function_name(const zend_function *func) {
    return func->common.function_name;
}

bool phper_call_user_function(zval *object, zval *function_name,
                              zval *retval_ptr, const zval *params,
                              uint32_t param_count,
                              const HashTable *named_params) {

    _call_user_function_impl(object, function_name, retval_ptr, param_count,
                             (zval *)params,
                             (HashTable *)named_params) == SUCCESS;
}

const zval *phper_zend_call_var_num(const zend_execute_data *execute_data,
                                    int index) {
    return ZEND_CALL_VAR_NUM(execute_data, index);
}

const zval *phper_zend_call_arg(const zend_execute_data *execute_data,
                                int index) {
    return ZEND_CALL_ARG(execute_data, index);
}

uint32_t phper_zend_num_args(const zend_execute_data *execute_data) {
    return ZEND_NUM_ARGS();
}

bool phper_zend_get_parameters_array_ex(uint32_t param_count,
                                        zval *argument_array) {
    return zend_get_parameters_array_ex(param_count, argument_array) == SUCCESS;
}
