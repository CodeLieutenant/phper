#include <phper.h>

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
    zend_string_release(s);
}

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
