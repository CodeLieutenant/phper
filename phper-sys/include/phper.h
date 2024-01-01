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

ZEND_FASTCALL void *phper_emalloc(size_t size);
ZEND_FASTCALL void phper_efree(void *ptr);

// ==================================================
// zval apis:
// ==================================================
ZEND_FASTCALL zend_long *phper_z_lval_p(const zval *zv);
ZEND_FASTCALL double *phper_z_dval_p(const zval *zv);
ZEND_FASTCALL zend_string *phper_z_str_p(const zval *zv);
ZEND_FASTCALL char *phper_z_strval_p(const zval *v);
ZEND_FASTCALL zend_array *phper_z_arr_p(const zval *zv);
ZEND_FASTCALL bool phper_z_refcounted_p(const zval *zval_ptr);
ZEND_FASTCALL int phper_z_res_handle_p(const zval *val);
ZEND_FASTCALL uint32_t phper_z_type_info_p(const zval *zv);
ZEND_FASTCALL int phper_z_type_p(const zval *zv);
ZEND_FASTCALL zend_resource *phper_z_res_p(const zval *zv);
ZEND_FASTCALL zend_reference *phper_z_ref_p(const zval *zv);
ZEND_FASTCALL const zend_object_handlers *phper_z_obj_ht_p(const zval *zv);
ZEND_FASTCALL zend_object *phper_z_obj_p(const zval *zv);
ZEND_FASTCALL uint32_t phper_z_addref_p(zval *zv);
ZEND_FASTCALL zend_function *phper_z_func_p(const zval *zv);
ZEND_FASTCALL void *phper_z_ptr_p(const zval *zv);
ZEND_FASTCALL zend_uchar phper_zval_get_type(const zval *pz);
ZEND_FASTCALL void phper_zval_arr(zval *val, zend_array *arr);
ZEND_FASTCALL void phper_zval_new_arr(zval *val);
ZEND_FASTCALL void phper_zval_stringl(zval *val, const char *s, size_t len);
ZEND_FASTCALL void phper_zval_zval(zval *val, zval *zv, int copy, int dtor);
ZEND_FASTCALL void phper_zval_copy(zval *val, const zval *zv);
ZEND_FASTCALL void phper_zval_copy_value(zval *val, const zval *zv);
ZEND_FASTCALL zend_string *phper_zval_get_string(zval *op);
ZEND_FASTCALL zend_long phper_zval_get_long(const zval *op);
ZEND_FASTCALL void phper_zval_obj(zval *z, zend_object *o);
ZEND_FASTCALL void phper_zval_func(zval *z, zend_function *f);
ZEND_FASTCALL void phper_zval_ptr_dtor(zval *zv);
ZEND_FASTCALL void phper_zval_ptr_dtor_nogc(zval *zval_ptr);
ZEND_FASTCALL void phper_zval_null(zval *zv);
ZEND_FASTCALL void phper_zval_bool(zval *zv, bool value);
ZEND_FASTCALL void phper_zval_long(zval *zv, zend_long l);
ZEND_FASTCALL void phper_zval_double(zval *zv, double d);
ZEND_FASTCALL void phper_zval_str(zval *zv, zend_string *s);
ZEND_FASTCALL void phper_convert_to_long(zval *op);
ZEND_FASTCALL void phper_convert_to_boolean(zval *op);
ZEND_FASTCALL void phper_convert_to_null(zval *op);
ZEND_FASTCALL void phper_convert_to_double(zval *op);
ZEND_FASTCALL void phper_convert_to_array(zval *op);
ZEND_FASTCALL void phper_convert_to_object(zval *op);
ZEND_FASTCALL void phper_convert_to_string(zval *op);
ZEND_FASTCALL void phper_separate_array(zval *zv);
ZEND_FASTCALL void phper_separate_string(zval *zv);
ZEND_FASTCALL void phper_separate_zval(zval *zv);

// ==================================================
// string apis:
// ==================================================
ZEND_FASTCALL zend_string *phper_zend_new_interned_string(zend_string *str);

ZEND_FASTCALL zend_string *phper_zend_string_init(const char *str, size_t len,
                                                  int persistent);
ZEND_FASTCALL zend_string *phper_zend_string_alloc(size_t len, int persistent);
ZEND_FASTCALL void phper_zend_string_release(zend_string *s);
ZEND_FASTCALL int phper_zstr_len(const zend_string *s);
ZEND_FASTCALL const char *phper_zstr_val(const zend_string *s);
ZEND_FASTCALL void phper_separate_string(zval *zv);
ZEND_FASTCALL zend_string *phper_zend_string_copy(zend_string *s);

// ==================================================
// string builder apis:
// ==================================================
ZEND_FASTCALL void phper_smart_str_alloc(smart_str *str, size_t len,
                                         bool persistent);
ZEND_FASTCALL void phper_smart_str_extend_ex(smart_str *dest, size_t len,
                                             bool persistent);
ZEND_FASTCALL void phper_smart_str_erealloc(smart_str *str, size_t len);
ZEND_FASTCALL void phper_smart_str_realloc(smart_str *str, size_t len);
ZEND_FASTCALL void phper_smart_str_free_ex(smart_str *str, bool persistent);
ZEND_FASTCALL void phper_smart_str_append_escaped(smart_str *str, const char *s,
                                                  size_t l);
ZEND_FASTCALL void phper_smart_str_append_double(smart_str *str, double num,
                                                 int precision,
                                                 bool zero_fraction);
ZEND_FASTCALL void phper_smart_str_append_escaped_truncated(
    smart_str *str, const zend_string *value, size_t length);

ZEND_FASTCALL void phper_smart_str_append_scalar(smart_str *str,
                                                 const zval *value,
                                                 size_t truncate);
ZEND_FASTCALL void phper_smart_str_0(smart_str *str);
ZEND_FASTCALL size_t phper_smart_str_get_len(const smart_str *str);
ZEND_FASTCALL zend_string *phper_smart_str_extract(smart_str *str);
ZEND_FASTCALL void phper_smart_str_appendc_ex(smart_str *dest, char ch,
                                              bool persistent);

ZEND_FASTCALL void phper_smart_str_appendl_ex(smart_str *dest, const char *str,
                                              size_t len, bool persistent);

ZEND_FASTCALL void phper_smart_str_append_ex(smart_str *dest,
                                             const zend_string *src,
                                             bool persistent);
ZEND_FASTCALL void phper_smart_str_append_smart_str_ex(smart_str *dest,
                                                       const smart_str *src,
                                                       bool persistent);
ZEND_FASTCALL void
phper_smart_str_append_long_ex(smart_str *dest, zend_long num, bool persistent);
ZEND_FASTCALL void phper_smart_str_append_unsigned_ex(smart_str *dest,
                                                      zend_ulong num,
                                                      bool persistent);
ZEND_FASTCALL void phper_smart_str_setl(smart_str *dest, const char *src,
                                        size_t len);

// ==================================================
// string builder apis:
// ==================================================
ZEND_FASTCALL void phper_smart_str_alloc(smart_str *str, size_t len,
                                         bool persistent);
ZEND_FASTCALL void phper_smart_str_extend_ex(smart_str *dest, size_t len,
                                             bool persistent);
ZEND_FASTCALL void phper_smart_str_erealloc(smart_str *str, size_t len);
ZEND_FASTCALL void phper_smart_str_realloc(smart_str *str, size_t len);
ZEND_FASTCALL void phper_smart_str_free_ex(smart_str *str, bool persistent);
ZEND_FASTCALL void phper_smart_str_append_escaped(smart_str *str, const char *s,
                                                  size_t l);
ZEND_FASTCALL void phper_smart_str_append_double(smart_str *str, double num,
                                                 int precision,
                                                 bool zero_fraction);
ZEND_FASTCALL void phper_smart_str_append_escaped_truncated(
    smart_str *str, const zend_string *value, size_t length);

ZEND_FASTCALL void phper_smart_str_append_scalar(smart_str *str,
                                                 const zval *value,
                                                 size_t truncate);
ZEND_FASTCALL void phper_smart_str_0(smart_str *str);
ZEND_FASTCALL size_t phper_smart_str_get_len(const smart_str *str);
ZEND_FASTCALL zend_string *phper_smart_str_extract(smart_str *str);
ZEND_FASTCALL void phper_smart_str_appendc_ex(smart_str *dest, char ch,
                                              bool persistent);

ZEND_FASTCALL void phper_smart_str_appendl_ex(smart_str *dest, const char *str,
                                              size_t len, bool persistent);

ZEND_FASTCALL void phper_smart_str_append_ex(smart_str *dest,
                                             const zend_string *src,
                                             bool persistent);
ZEND_FASTCALL void phper_smart_str_append_smart_str_ex(smart_str *dest,
                                                       const smart_str *src,
                                                       bool persistent);
ZEND_FASTCALL void
phper_smart_str_append_long_ex(smart_str *dest, zend_long num, bool persistent);
ZEND_FASTCALL void phper_smart_str_append_unsigned_ex(smart_str *dest,
                                                      zend_ulong num,
                                                      bool persistent);
ZEND_FASTCALL void phper_smart_str_setl(smart_str *dest, const char *src,
                                        size_t len);

// ==================================================
// array apis:
// ==================================================

ZEND_FASTCALL zval *phper_zend_hash_str_update(HashTable *ht, const char *key,
                                               size_t len, zval *pData);

ZEND_FASTCALL zval *phper_zend_hash_index_update(HashTable *ht, zend_ulong h,
                                                 zval *pData);

ZEND_FASTCALL zval *phper_zend_hash_next_index_insert(HashTable *ht,
                                                      zval *pData);

ZEND_FASTCALL void phper_array_init(zval *arg);
ZEND_FASTCALL void *phper_zend_hash_str_find_ptr(const HashTable *ht,
                                                 const char *str, size_t len);
ZEND_FASTCALL bool phper_zend_hash_str_exists(const HashTable *ht,
                                              const char *str, size_t len);
ZEND_FASTCALL bool phper_zend_hash_index_exists(const HashTable *ht,
                                                zend_ulong h);
ZEND_FASTCALL zend_array *phper_zend_new_array(uint32_t size);
ZEND_FASTCALL zend_array *phper_zend_array_dup(zend_array *source);
ZEND_FASTCALL zval *phper_zend_hash_index_find(const HashTable *ht,
                                               zend_ulong h);
ZEND_FASTCALL bool phper_zend_hash_index_del(HashTable *ht, zend_ulong h);
ZEND_FASTCALL zval *phper_zend_symtable_str_update(HashTable *ht,
                                                   const char *str, size_t len,
                                                   zval *pData);

ZEND_FASTCALL bool phper_zend_symtable_str_del(HashTable *ht, const char *str,
                                               size_t len);
ZEND_FASTCALL zval *phper_zend_symtable_str_find(HashTable *ht, const char *str,
                                                 size_t len);

ZEND_FASTCALL bool phper_zend_symtable_str_exists(HashTable *ht,
                                                  const char *str, size_t len);

ZEND_FASTCALL zval *phper_zend_str_update(HashTable *ht, const char *str,
                                          size_t len, zval *pData);
ZEND_FASTCALL bool phper_zend_str_del(HashTable *ht, const char *str,
                                      size_t len);
ZEND_FASTCALL zval *phper_zend_str_find(HashTable *ht, const char *str,
                                        size_t len);
ZEND_FASTCALL bool phper_zend_str_exists(HashTable *ht, const char *str,
                                         size_t len);