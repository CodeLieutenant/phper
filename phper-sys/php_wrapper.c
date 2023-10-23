// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

#include <stdbool.h>

#include <php.h>
#include <php_ini.h>

#include <ext/standard/info.h>
#include <main/SAPI.h>
#include <zend_exceptions.h>
#include <zend_interfaces.h>

#include <zend_smart_str.h>

#if PHP_MAJOR_VERSION >= 8
#include <zend_observer.h>
#endif

typedef ZEND_INI_MH(phper_zend_ini_mh);

typedef zend_class_entry *
phper_init_class_entry_handler(zend_class_entry *class_ce, void *argument);

// ==================================================
// zval apis:
// ==================================================

ZEND_FASTCALL zend_long *phper_z_lval_p(zval *zv) {
    return &(Z_LVAL_P(zv));
}

ZEND_FASTCALL double *phper_z_dval_p(zval *zv) {
    return &(Z_DVAL_P(zv));
}

ZEND_FASTCALL zend_string *phper_z_str_p(const zval *zv) {
    return Z_STR_P(zv);
}

ZEND_FASTCALL char *phper_z_strval_p(const zval *v) {
    return Z_STRVAL_P(v);
}

ZEND_FASTCALL zend_array *phper_z_arr_p(const zval *zv) {
    return Z_ARR_P(zv);
}

ZEND_FASTCALL bool phper_z_refcounted_p(zval *zval_ptr) {
    return Z_REFCOUNTED_P(zval_ptr);
}

ZEND_FASTCALL int phper_z_res_handle_p(const zval *val) {
    return Z_RES_HANDLE_P(val);
}

ZEND_FASTCALL uint32_t phper_z_type_info_p(const zval *zv) {
    return Z_TYPE_INFO_P(zv);
}

ZEND_FASTCALL int phper_z_type_p(zval *zv) {
    return Z_TYPE_P(zv);
}

ZEND_FASTCALL zend_resource *phper_z_res_p(const zval *zv) {
    return Z_RES_P(zv);
}

ZEND_FASTCALL zend_reference *phper_z_ref_p(const zval *zv) {
    return Z_REF_P(zv);
}

ZEND_FASTCALL const zend_object_handlers *phper_z_obj_ht_p(const zval *zv) {
    return Z_OBJ_HT_P(zv);
}

ZEND_FASTCALL zend_object *phper_z_obj_p(const zval *zv) {
    return Z_OBJ_P(zv);
}

ZEND_FASTCALL uint32_t phper_z_addref_p(zval *zv) {
    return Z_ADDREF_P(zv);
}

ZEND_FASTCALL zend_function *phper_z_func_p(const zval *zv) {
    return Z_FUNC_P(zv);
}

ZEND_FASTCALL void *phper_z_ptr_p(const zval *zv) {
    return Z_PTR_P(zv);
}

ZEND_FASTCALL zend_uchar phper_zval_get_type(const zval *pz) {
    return zval_get_type(pz);
}

ZEND_FASTCALL void phper_zval_arr(zval *val, zend_array *arr) {
    ZVAL_ARR(val, arr);
}

ZEND_FASTCALL void phper_zval_new_arr(zval *val) {
#if PHP_VERSION_ID < 80100
    ZVAL_NEW_ARR(val);
#else
    array_init(val);
#endif
}

ZEND_FASTCALL void phper_zval_stringl(zval *val, const char *s, size_t len) {
    ZVAL_STRINGL(val, s, len);
}

ZEND_FASTCALL void phper_zval_zval(zval *val, zval *zv, int copy, int dtor) {
    ZVAL_ZVAL(val, zv, copy, dtor);
}

ZEND_FASTCALL void phper_zval_copy(zval *val, const zval *zv) {
    ZVAL_COPY(val, zv);
}

ZEND_FASTCALL void phper_zval_copy_value(zval *val, const zval *zv) {
    ZVAL_COPY_VALUE(val, zv);
}

ZEND_FASTCALL zend_string *phper_zval_get_string(zval *op) {
    return zval_get_string(op);
}

ZEND_FASTCALL zend_long phper_zval_get_long(zval *op) {
    return zval_get_long(op);
}

ZEND_FASTCALL void phper_zval_obj(zval *z, zend_object *o) {
    ZVAL_OBJ(z, o);
}

ZEND_FASTCALL void phper_zval_func(zval *z, zend_function *f) {
    ZVAL_FUNC(z, f);
}

ZEND_FASTCALL void phper_zval_ptr_dtor(zval *zv) {
    ZVAL_PTR_DTOR(zv);
}

ZEND_FASTCALL void phper_zval_ptr_dtor_nogc(zval *zval_ptr) {
    zval_ptr_dtor_nogc(zval_ptr);
}

ZEND_FASTCALL void phper_zval_null(zval *zv) {
    ZVAL_NULL(zv);
}

ZEND_FASTCALL void phper_zval_true(zval *zv) {
    ZVAL_TRUE(zv);
}

ZEND_FASTCALL void phper_zval_false(zval *zv) {
    ZVAL_FALSE(zv);
}

ZEND_FASTCALL void phper_zval_long(zval *zv, zend_long l) {
    ZVAL_LONG(zv, l);
}

ZEND_FASTCALL void phper_zval_double(zval *zv, double d) {
    ZVAL_DOUBLE(zv, d);
}

ZEND_FASTCALL void phper_zval_str(zval *zv, zend_string *s) {
    ZVAL_STR(zv, s);
}

ZEND_FASTCALL void phper_convert_to_long(zval *op) {
    convert_to_long(op);
}

ZEND_FASTCALL void phper_convert_to_string(zval *op) {
    convert_to_string(op);
}

ZEND_FASTCALL void phper_separate_array(zval *zv) {
    SEPARATE_ARRAY(zv);
}

// ==================================================
// string apis:
// ==================================================

ZEND_FASTCALL zend_string *phper_zend_new_interned_string(zend_string *str) {
    return zend_new_interned_string(str);
}

ZEND_FASTCALL zend_string *phper_zend_string_init(const char *str, size_t len,
                                                  int persistent) {
    return zend_string_init(str, len, persistent);
}

ZEND_FASTCALL zend_string *phper_zend_string_alloc(size_t len, int persistent) {
    return zend_string_alloc(len, persistent);
}

ZEND_FASTCALL void phper_zend_string_release(zend_string *s) {
    return zend_string_release(s);
}

#if PHP_VERSION_ID < 80000
static zend_string *phper_zend_string_concat3(const char *str1, size_t str1_len,
                                              const char *str2, size_t str2_len,
                                              const char *str3,
                                              size_t str3_len) {
    size_t len = str1_len + str2_len + str3_len;
    zend_string *res = zend_string_alloc(len, 0);

    memcpy(ZSTR_VAL(res), str1, str1_len);
    memcpy(ZSTR_VAL(res) + str1_len, str2, str2_len);
    memcpy(ZSTR_VAL(res) + str1_len + str2_len, str3, str3_len);
    ZSTR_VAL(res)
    [len] = '\0';

    return res;
}
#endif

ZEND_FASTCALL int phper_zstr_len(const zend_string *s) {
    return ZSTR_LEN(s);
}

ZEND_FASTCALL const char *phper_zstr_val(const zend_string *s) {
    return ZSTR_VAL(s);
}

ZEND_FASTCALL void phper_separate_string(zval *zv) {
    SEPARATE_STRING(zv);
}

ZEND_FASTCALL zend_string *phper_zend_string_copy(zend_string *s) {
    return zend_string_copy(s);
}

// ==================================================
// string builder apis:
// ==================================================

ZEND_FASTCALL void phper_smart_str_alloc(smart_str *str, size_t len,
                                         bool persistent) {
    smart_str_alloc(str, len, persistent);
}

ZEND_FASTCALL void phper_smart_str_extend_ex(smart_str *dest, size_t len,
                                             bool persistent) {
    smart_str_extend_ex(dest, len, persistent);
}

ZEND_FASTCALL void phper_smart_str_erealloc(smart_str *str, size_t len) {
    smart_str_erealloc(str, len);
}

ZEND_FASTCALL void phper_smart_str_realloc(smart_str *str, size_t len) {
    smart_str_realloc(str, len);
}

ZEND_FASTCALL void phper_smart_str_free_ex(smart_str *str, bool persistent) {
    smart_str_free_ex(str, persistent);
}

ZEND_FASTCALL void phper_smart_str_append_escaped(smart_str *str, const char *s,
                                                  size_t l) {
    smart_str_append_escaped(str, s, l);
}

ZEND_FASTCALL void phper_smart_str_append_double(smart_str *str, double num,
                                                 int precision,
                                                 bool zero_fraction) {
    smart_str_append_double(str, num, precision, zero_fraction);
}

ZEND_FASTCALL void phper_smart_str_append_escaped_truncated(
    smart_str *str, const zend_string *value, size_t length) {
    smart_str_append_escaped_truncated(str, value, length);
}

ZEND_FASTCALL void phper_smart_str_append_scalar(smart_str *str,
                                                 const zval *value,
                                                 size_t truncate) {
    smart_str_append_scalar(str, value, truncate);
}

ZEND_FASTCALL void phper_smart_str_0(smart_str *str) {
    smart_str_0(str);
}

ZEND_FASTCALL size_t phper_smart_str_get_len(const smart_str *str) {
    return smart_str_get_len(str);
}

ZEND_FASTCALL zend_string *phper_smart_str_extract(smart_str *str) {
    return smart_str_extract(str);
}

ZEND_FASTCALL void phper_smart_str_appendc_ex(smart_str *dest, char ch,
                                              bool persistent) {
    smart_str_appendc_ex(dest, ch, persistent);
}

ZEND_FASTCALL void phper_smart_str_appendl_ex(smart_str *dest, const char *str,
                                              size_t len, bool persistent) {
    smart_str_appendl_ex(dest, str, len, persistent);
}

ZEND_FASTCALL void phper_smart_str_append_ex(smart_str *dest,
                                             const zend_string *src,
                                             bool persistent) {
    smart_str_append_ex(dest, src, persistent);
}

ZEND_FASTCALL void phper_smart_str_append_smart_str_ex(smart_str *dest,
                                                       const smart_str *src,
                                                       bool persistent) {
    smart_str_append_smart_str_ex(dest, src, persistent);
}

ZEND_FASTCALL void phper_smart_str_append_long_ex(smart_str *dest,
                                                  zend_long num,
                                                  bool persistent) {
    smart_str_append_long_ex(dest, num, persistent);
}

ZEND_FASTCALL void phper_smart_str_append_unsigned_ex(smart_str *dest,
                                                      zend_ulong num,
                                                      bool persistent) {
    smart_str_append_unsigned_ex(dest, num, persistent);
}

ZEND_FASTCALL void phper_smart_str_setl(smart_str *dest, const char *src,
                                        size_t len) {
    smart_str_setl(dest, src, len);
}

// ==================================================
// array apis:
// ==================================================

zval *phper_zend_hash_str_update(HashTable *ht, const char *key, size_t len,
                                 zval *pData) {
    return zend_hash_str_update(ht, key, len, pData);
}

zval *phper_zend_hash_index_update(HashTable *ht, zend_ulong h, zval *pData) {
    return zend_hash_index_update(ht, h, pData);
}

zval *phper_zend_hash_next_index_insert(HashTable *ht, zval *pData) {
    return zend_hash_next_index_insert(ht, pData);
}

void phper_array_init(zval *arg) {
    array_init(arg);
}

void *phper_zend_hash_str_find_ptr(const HashTable *ht, const char *str,
                                   size_t len) {
    return zend_hash_str_find_ptr(ht, str, len);
}

bool phper_zend_hash_str_exists(const HashTable *ht, const char *str,
                                size_t len) {
    return zend_hash_str_exists(ht, str, len) != 0;
}

bool phper_zend_hash_index_exists(const HashTable *ht, zend_ulong h) {
    return zend_hash_index_exists(ht, h) != 0;
}

zend_array *phper_zend_new_array(uint32_t size) {
#if PHP_VERSION_ID >= 70300
    return zend_new_array(size);
#else
    HashTable *ht = emalloc(sizeof(HashTable));
    zend_hash_init(ht, size, NULL, ZVAL_PTR_DTOR, 0);
    return ht;
#endif
}

zend_array *phper_zend_array_dup(zend_array *source) {
    return zend_array_dup(source);
}

zval *phper_zend_hash_index_find(const HashTable *ht, zend_ulong h) {
    return zend_hash_index_find(ht, h);
}

bool phper_zend_hash_index_del(HashTable *ht, zend_ulong h) {
    return zend_hash_index_del(ht, h) == SUCCESS;
}

zval *phper_zend_symtable_str_update(HashTable *ht, const char *str, size_t len,
                                     zval *pData) {
    return zend_symtable_str_update(ht, str, len, pData);
}

bool phper_zend_symtable_str_del(HashTable *ht, const char *str, size_t len) {
    return zend_symtable_str_del(ht, str, len) == SUCCESS;
}

zval *phper_zend_symtable_str_find(HashTable *ht, const char *str, size_t len) {
    return zend_symtable_str_find(ht, str, len);
}

bool phper_zend_symtable_str_exists(HashTable *ht, const char *str,
                                    size_t len) {
    return zend_symtable_str_exists(ht, str, len) != 0;
}

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
#if PHP_VERSION_ID >= 70300
    return zend_object_alloc(obj_size, ce);
#else
    void *obj = emalloc(obj_size + zend_object_properties_size(ce));
    memset(obj, 0, obj_size - sizeof(zval));
    return obj;
#endif
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
#if PHP_VERSION_ID >= 80000
    return get_function_or_method_name(func);
#else
    if (func->common.scope) {
        return phper_zend_string_concat3(ZSTR_VAL(func->common.scope->name),
                                         ZSTR_LEN(func->common.scope->name),
                                         "::", sizeof("::") - 1,
                                         ZSTR_VAL(func->common.function_name),
                                         ZSTR_LEN(func->common.function_name));
    }
    return func->common.function_name
               ? zend_string_copy(func->common.function_name)
               : zend_string_init("main", sizeof("main") - 1, 0);
#endif
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
// memory apis:
// ==================================================

void *phper_emalloc(size_t size) {
    return emalloc(size);
}

void phper_efree(void *ptr) {
    return efree(ptr);
}

// ==================================================
// module apis:
// ==================================================

const char *phper_get_zend_module_build_id() {
    return ZEND_MODULE_BUILD_ID;
}

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
