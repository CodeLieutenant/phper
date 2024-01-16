#pragma once

#include <stdbool.h>
#include <stddef.h>

#include <php.h>
#include <php_ini.h>

#include <ext/standard/info.h>
#include <main/SAPI.h>

#include <zend_exceptions.h>
#include <zend_interfaces.h>
#include <zend_observer.h>
#include <zend_smart_str_public.h>

typedef ZEND_INI_MH(phper_zend_ini_mh);

typedef zend_class_entry *
phper_init_class_entry_handler(zend_class_entry *class_ce, void *argument);

void *phper_emalloc(size_t size);
void phper_efree(void *ptr);

// ==================================================
// zval apis:
// ==================================================
const zend_long *phper_z_lval_p(const zval *zv);
const double *phper_z_dval_p(const zval *zv);
const zend_string *phper_z_str_p(const zval *zv);
const char *phper_z_strval_p(const zval *v);
const zend_array *phper_z_arr_p(const zval *zv);
bool phper_z_refcounted_p(const zval *zval_ptr);
int phper_z_res_handle_p(const zval *val);
uint32_t phper_z_type_info_p(const zval *zv);
int phper_z_type_p(const zval *zv);
const zend_resource *phper_z_res_p(const zval *zv);
const zend_reference *phper_z_ref_p(const zval *zv);
const zend_object_handlers *phper_z_obj_ht_p(const zval *zv);
const zend_object *phper_z_obj_p(const zval *zv);
uint32_t phper_z_addref_p(zval *zv);
zend_function *phper_z_func_p(const zval *zv);
const void *phper_z_ptr_p(const zval *zv);
zend_uchar phper_zval_get_type(const zval *pz);
void phper_zval_arr(zval *val, zend_array *arr);
void phper_zval_new_arr(zval *val);
void phper_zval_stringl(zval *val, const char *s, size_t len);
void phper_zval_zval(zval *val, zval *zv, int copy, int dtor);
void phper_zval_copy(zval *val, const zval *zv);
void phper_zval_copy_value(zval *val, const zval *zv);
zend_string *phper_zval_get_string(zval *op);
zend_long phper_zval_get_long(const zval *op);
void phper_zval_obj(zval *z, zend_object *o);
void phper_zval_func(zval *z, zend_function *f);
void phper_zval_ptr_dtor(zval *zv);
void phper_zval_ptr_dtor_nogc(zval *zval_ptr);
void phper_zval_null(zval *zv);
void phper_zval_bool(zval *zv, bool value);
void phper_zval_long(zval *zv, zend_long l);
void phper_zval_double(zval *zv, double d);
void phper_zval_str(zval *zv, zend_string *s);
void phper_convert_to_long(zval *op);
void phper_convert_to_boolean(zval *op);
void phper_convert_to_null(zval *op);
void phper_convert_to_double(zval *op);
void phper_convert_to_array(zval *op);
void phper_convert_to_object(zval *op);
void phper_convert_to_string(zval *op);
void phper_separate_array(zval *zv);
void phper_separate_string(zval *zv);
void phper_separate_zval(zval *zv);

// ==================================================
// string apis:
// ==================================================
zend_string *phper_zend_new_interned_string(zend_string *str);

zend_string *phper_zend_string_init(const char *str, size_t len,
                                    int persistent);
zend_string *phper_zend_string_alloc(size_t len, int persistent);
void phper_zend_string_release(zend_string *s);
int phper_zstr_len(const zend_string *s);
const char *phper_zstr_val(const zend_string *s);
zend_string *phper_zend_string_copy(zend_string *s);

// ==================================================
// string builder apis:
// ==================================================
void phper_smart_str_alloc(smart_str *str, size_t len, bool persistent);
void phper_smart_str_extend_ex(smart_str *dest, size_t len, bool persistent);
void phper_smart_str_erealloc(smart_str *str, size_t len);
void phper_smart_str_realloc(smart_str *str, size_t len);
void phper_smart_str_free_ex(smart_str *str, bool persistent);
void phper_smart_str_append_escaped(smart_str *str, const char *s, size_t l);
void phper_smart_str_append_double(smart_str *str, double num, int precision,
                                   bool zero_fraction);
void phper_smart_str_append_escaped_truncated(smart_str *str,
                                              const zend_string *value,
                                              size_t length);

void phper_smart_str_append_scalar(smart_str *str, const zval *value,
                                   size_t truncate);
void phper_smart_str_0(smart_str *str);
size_t phper_smart_str_get_len(const smart_str *str);
zend_string *phper_smart_str_extract(smart_str *str);
void phper_smart_str_appendc_ex(smart_str *dest, char ch, bool persistent);

void phper_smart_str_appendl_ex(smart_str *dest, const char *str, size_t len,
                                bool persistent);

void phper_smart_str_append_ex(smart_str *dest, const zend_string *src,
                               bool persistent);
void phper_smart_str_append_smart_str_ex(smart_str *dest, const smart_str *src,
                                         bool persistent);
void phper_smart_str_append_long_ex(smart_str *dest, zend_long num,
                                    bool persistent);
void phper_smart_str_append_unsigned_ex(smart_str *dest, zend_ulong num,
                                        bool persistent);
void phper_smart_str_setl(smart_str *dest, const char *src, size_t len);

// ==================================================
// string builder apis:
// ==================================================
void phper_smart_str_alloc(smart_str *str, size_t len, bool persistent);
void phper_smart_str_extend_ex(smart_str *dest, size_t len, bool persistent);
void phper_smart_str_erealloc(smart_str *str, size_t len);
void phper_smart_str_realloc(smart_str *str, size_t len);
void phper_smart_str_free_ex(smart_str *str, bool persistent);
void phper_smart_str_append_escaped(smart_str *str, const char *s, size_t l);
void phper_smart_str_append_double(smart_str *str, double num, int precision,
                                   bool zero_fraction);
void phper_smart_str_append_escaped_truncated(smart_str *str,
                                              const zend_string *value,
                                              size_t length);

void phper_smart_str_append_scalar(smart_str *str, const zval *value,
                                   size_t truncate);
void phper_smart_str_0(smart_str *str);
size_t phper_smart_str_get_len(const smart_str *str);
zend_string *phper_smart_str_extract(smart_str *str);
void phper_smart_str_appendc_ex(smart_str *dest, char ch, bool persistent);

void phper_smart_str_appendl_ex(smart_str *dest, const char *str, size_t len,
                                bool persistent);

void phper_smart_str_append_ex(smart_str *dest, const zend_string *src,
                               bool persistent);
void phper_smart_str_append_smart_str_ex(smart_str *dest, const smart_str *src,
                                         bool persistent);
void phper_smart_str_append_long_ex(smart_str *dest, zend_long num,
                                    bool persistent);
void phper_smart_str_append_unsigned_ex(smart_str *dest, zend_ulong num,
                                        bool persistent);
void phper_smart_str_setl(smart_str *dest, const char *src, size_t len);

// ==================================================
// array apis:
// ==================================================

zval *phper_zend_hash_str_update(HashTable *ht, const char *key, size_t len,
                                 zval *pData);

zval *phper_zend_hash_index_update(HashTable *ht, zend_ulong h, zval *pData);

zval *phper_zend_hash_next_index_insert(HashTable *ht, zval *pData);

void phper_array_init(zval *arg);
void *phper_zend_hash_str_find_ptr(const HashTable *ht, const char *str,
                                   size_t len);
bool phper_zend_hash_str_exists(const HashTable *ht, const char *str,
                                size_t len);
bool phper_zend_hash_index_exists(const HashTable *ht, zend_ulong h);
zend_array *phper_zend_new_array(uint32_t size);
zend_array *phper_zend_array_dup(zend_array *source);
zval *phper_zend_hash_index_find(const HashTable *ht, zend_ulong h);
bool phper_zend_hash_index_del(HashTable *ht, zend_ulong h);
zval *phper_zend_symtable_str_update(HashTable *ht, const char *str, size_t len,
                                     zval *pData);

bool phper_zend_symtable_str_del(HashTable *ht, const char *str, size_t len);
zval *phper_zend_symtable_str_find(HashTable *ht, const char *str, size_t len);

bool phper_zend_symtable_str_exists(HashTable *ht, const char *str, size_t len);

zval *phper_zend_str_update(HashTable *ht, const char *str, size_t len,
                            zval *pData);
bool phper_zend_str_del(HashTable *ht, const char *str, size_t len);
zval *phper_zend_str_find(HashTable *ht, const char *str, size_t len);
bool phper_zend_str_exists(HashTable *ht, const char *str, size_t len);

// ==================================================
// object apis:
// ==================================================

zval *phper_get_this(const zend_execute_data *execute_data);
size_t phper_zend_object_properties_size(const zend_class_entry *ce);
void *phper_zend_object_alloc(size_t obj_size, const zend_class_entry *ce);
bool phper_object_init_ex(zval *arg, const zend_class_entry *class_type);
void phper_zend_object_release(zend_object *obj);
uint32_t phper_zend_object_gc_refcount(const zend_object *obj);

// ==================================================
// class apis:
// ==================================================
zend_class_entry phper_init_class_entry_ex(const char *class_name,
                                           size_t class_name_len);

zend_class_entry *
phper_register_class_entry_ex(zend_class_entry *ce, zend_class_entry *parent,
                              const zend_function_entry *functions);

zend_class_entry phper_init_interface_entry_ex(const char *class_name,
                                               size_t class_name_len);

bool phper_instanceof_function(const zend_class_entry *instance_ce,
                               const zend_class_entry *ce);

zend_class_entry *
phper_register_interface_entry_ex(zend_class_entry *ce,
                                  const zend_function_entry *functions);

zend_string *phper_get_function_or_method_name(const zend_function *func);
zend_string *phper_get_function_name(const zend_function *func);
bool phper_call_user_function(zval *object, zval *function_name,
                              zval *retval_ptr, const zval *params,
                              uint32_t param_count,
                              const HashTable *named_params);
const zval *phper_zend_call_var_num(const zend_execute_data *execute_data,
                                    int index);
const zval *phper_zend_call_arg(const zend_execute_data *execute_data,
                                int index);
uint32_t phper_zend_num_args(const zend_execute_data *execute_data);
bool phper_zend_get_parameters_array_ex(uint32_t param_count,
                                        zval *argument_array);

// ==================================================
// module apis:
// ==================================================
const char *phper_get_zend_module_build_id();
zend_resource *phper_register_persistent_resource(const zend_string *id,
                                                  const void *ptr, int le_id);
int phper_zend_register_persistent_list_destructors(rsrc_dtor_func_t dtor,
                                                    const char *name,
                                                    int module_number);
int phper_zend_register_list_destructors(const rsrc_dtor_func_t dtor,
                                         const char *name, int module_number);
int phper_zend_register_list_destructors_ex(const rsrc_dtor_func_t dtor,
                                            const rsrc_dtor_func_t pdtor,
                                            const char *name,
                                            int module_number);
int phper_zend_fetch_list_dtor_id(const char *name);
const zend_resource *phper_register_persistent_find(const char *hash,
                                                    size_t len);

// ==================================================
// Argument API:
// ==================================================

zend_internal_arg_info
phper_zend_begin_arg_info_ex(bool return_reference,
                             uintptr_t required_num_args);
zend_internal_arg_info phper_zend_arg_info(bool pass_by_ref, const char *name);

// ==================================================
// Constants API:
// ==================================================
zend_constant phper_create_constant(const char *name, size_t name_len, zval val,
                                    int flags);

zend_result phper_register_constant(zend_constant *constant, int flags,
                                    int module_number);