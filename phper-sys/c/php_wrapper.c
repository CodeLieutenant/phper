// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

#include <phper.h>

// ==================================================
// object apis:
// ==================================================

zval *phper_get_this(zend_execute_data *execute_data) {
    return getThis();
}

size_t phper_zend_object_properties_size(zend_class_entry *ce) {
    return zend_object_properties_size(ce);
}

void *phper_zend_object_alloc(size_t obj_size, zend_class_entry *ce) {
    return zend_object_alloc(obj_size, ce);
}

zend_object *(**phper_get_create_object(zend_class_entry *ce))(
    zend_class_entry *class_type) {
    return &ce->create_object;
}

bool phper_object_init_ex(zval *arg, zend_class_entry *class_type) {
    return object_init_ex(arg, class_type) == SUCCESS;
}

void phper_zend_object_release(zend_object *obj) {
    zend_object_release(obj);
}

uint32_t phper_zend_object_gc_refcount(const zend_object *obj) {
    return GC_REFCOUNT(obj);
}

// ==================================================
// class apis:
// ==================================================

zend_class_entry *
phper_init_class_entry_ex(const char *class_name, size_t class_name_len,
                          const zend_function_entry *functions,
                          phper_init_class_entry_handler handler,
                          void *argument) {
    zend_class_entry class_ce;
    INIT_CLASS_ENTRY_EX(class_ce, class_name, class_name_len, functions);
    return handler(&class_ce, argument);
}

bool phper_instanceof_function(const zend_class_entry *instance_ce,
                               const zend_class_entry *ce) {
    return instanceof_function(instance_ce, ce) != 0;
}

// ==================================================
// function apis:
// ==================================================

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

// ==================================================
// module apis:
// ==================================================

const char *phper_get_zend_module_build_id() {
    return ZEND_MODULE_BUILD_ID;
}

zend_resource *phper_register_persistent_resource(const zend_string *id,
                                                  const void *ptr, int le_id) {
    return zend_register_persistent_resource_ex((zend_string *)id, (void *)ptr,
                                                le_id);
}

int phper_zend_register_persistent_list_destructors(rsrc_dtor_func_t dtor,
                                                    const char *name,
                                                    int module_number) {
    return zend_register_list_destructors_ex(NULL, dtor, name, module_number);
}

int phper_zend_register_list_destructors(const rsrc_dtor_func_t dtor,
                                         const char *name, int module_number) {
    return zend_register_list_destructors_ex((rsrc_dtor_func_t)dtor, NULL, name,
                                             module_number);
}

int phper_zend_register_list_destructors_ex(const rsrc_dtor_func_t dtor,
                                            const rsrc_dtor_func_t pdtor,
                                            const char *name,
                                            int module_number) {

    return zend_register_list_destructors_ex(
        (rsrc_dtor_func_t)dtor, (rsrc_dtor_func_t)pdtor, name, module_number);
}

int phper_zend_fetch_list_dtor_id(const char *name) {
    return zend_fetch_list_dtor_id(name);
}

const zend_resource *phper_register_persistent_find(const char *hash,
                                                    size_t len) {
    zend_resource *zv = zend_hash_str_find_ptr(&EG(persistent_list), hash, len);

    if (zv == NULL) {
        php_error_docref(0, E_WARNING, "Invalid persistent resource");
        return NULL;
    }

    return zv;
}

// ==================================================
// Argument API:
// ==================================================

zend_internal_arg_info
phper_zend_begin_arg_info_ex(bool return_reference,
                             uintptr_t required_num_args) {
#define static
#define const
    ZEND_BEGIN_ARG_INFO_EX(info, 0, return_reference, required_num_args)
    ZEND_END_ARG_INFO()
    return info[0];
#undef static
#undef const
}

zend_internal_arg_info phper_zend_arg_info(bool pass_by_ref, const char *name) {
    zend_internal_arg_info info[] = {ZEND_ARG_INFO(pass_by_ref, )};
    info[0].name = name;
    return info[0];
}

//  zend_internal_arg_info phper_zend_arg_info(
//     const char *name, int32_t type, bool return_reference, bool is_variadic,
//     bool is_tentative, uintptr_t required_num_args) {
//     zend_internal_arg_info info = {
//         .name = name,
//         .type = ZEND_TYPE_INIT_MASK(
//             type |
//             _ZEND_ARG_INFO_FLAGS(return_reference, is_variadic,
//             is_tentative)),
//     };

//     return info;
// }
