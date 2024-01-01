#include <phper.h>
// ==================================================
// zval apis:
// ==================================================

ZEND_FASTCALL zend_long *phper_z_lval_p(const zval *zv) {
    return &(Z_LVAL_P(zv));
}

ZEND_FASTCALL double *phper_z_dval_p(const zval *zv) {
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

ZEND_FASTCALL bool phper_z_refcounted_p(const zval *zval_ptr) {
    return Z_REFCOUNTED_P(zval_ptr);
}

ZEND_FASTCALL int phper_z_res_handle_p(const zval *val) {
    return Z_RES_HANDLE_P(val);
}

ZEND_FASTCALL uint32_t phper_z_type_info_p(const zval *zv) {
    return Z_TYPE_INFO_P(zv);
}

ZEND_FASTCALL int phper_z_type_p(const zval *zv) {
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
    array_init(val);
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

ZEND_FASTCALL zend_long phper_zval_get_long(const zval *op) {
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

ZEND_FASTCALL void phper_zval_bool(zval *zv, bool value) {
    if (value) {
        ZVAL_TRUE(zv);
    } else {
        ZVAL_FALSE(zv);
    }
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

ZEND_FASTCALL void phper_convert_to_boolean(zval *op) {
    convert_to_boolean(op);
}

ZEND_FASTCALL void phper_convert_to_null(zval *op) {
    convert_to_null(op);
}

ZEND_FASTCALL void phper_convert_to_double(zval *op) {
    convert_to_double(op);
}

ZEND_FASTCALL void phper_convert_to_array(zval *op) {
    convert_to_array(op);
}

ZEND_FASTCALL void phper_convert_to_object(zval *op) {
    convert_to_object(op);
}

ZEND_FASTCALL void phper_convert_to_string(zval *op) {
    convert_to_string(op);
}

ZEND_FASTCALL void phper_separate_array(zval *zv) {
    SEPARATE_ARRAY(zv);
}

ZEND_FASTCALL void phper_separate_string(zval *zv) {
    SEPARATE_STRING(zv);
}

ZEND_FASTCALL void phper_separate_zval(zval *zv) {
    SEPARATE_ZVAL(zv);
}
