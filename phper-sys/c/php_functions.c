#include <phper.h>

zend_string *phper_get_function_or_method_name(const zend_function *func) {
    return get_function_or_method_name(func);
}

zend_string *phper_get_function_name(const zend_function *func) {
    return func->common.function_name;
}

bool phper_call_user_function(HashTable *function_table, zval *object,
                              zval *function_name, zval *retval_ptr,
                              uint32_t param_count, zval params[]) {
    (void)function_table; // suppress "unused parameter" warnings.
    return call_user_function(function_table, object, function_name, retval_ptr,
                              param_count, params) == SUCCESS;
}

zval *phper_zend_call_var_num(zend_execute_data *execute_data, int index) {
    return ZEND_CALL_VAR_NUM(execute_data, index);
}

zval *phper_zend_call_arg(zend_execute_data *execute_data, int index) {
    return ZEND_CALL_ARG(execute_data, index);
}

uint32_t phper_zend_num_args(const zend_execute_data *execute_data) {
    return ZEND_NUM_ARGS();
}

bool phper_zend_get_parameters_array_ex(uint32_t param_count,
                                        zval *argument_array) {
    return zend_get_parameters_array_ex(param_count, argument_array) != 0;
}
