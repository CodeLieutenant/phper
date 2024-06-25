#include <phper.h>

zend_string *phper_zend_new_interned_string(zend_string *str) {
    return zend_new_interned_string(str);
}

zend_string *phper_zend_string_init(const char *str, size_t len,
                                    int persistent) {
    return zend_string_init(str, len, persistent);
}

zend_string *phper_zend_string_alloc(size_t len, int persistent) {
    return zend_string_alloc(len, persistent);
}

void phper_zend_string_release(zend_string *s) {
    zend_string_release(s);
}

int phper_zstr_len(const zend_string *s) {
    return ZSTR_LEN(s);
}

const char *phper_zstr_val(const zend_string *s) {
    return ZSTR_VAL(s);
}

void phper_separate_string(zval *zv) {
    SEPARATE_STRING(zv);
}

zend_string *phper_zend_string_copy(zend_string *s) {
    return zend_string_copy(s);
}
