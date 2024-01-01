#include <phper.h>
#include <zend_smart_str.h>

 void phper_smart_str_alloc(smart_str *str, size_t len,
                                         bool persistent) {
    smart_str_alloc(str, len, persistent);
}

 void phper_smart_str_extend_ex(smart_str *dest, size_t len,
                                             bool persistent) {
    smart_str_extend_ex(dest, len, persistent);
}

 void phper_smart_str_erealloc(smart_str *str, size_t len) {
    smart_str_erealloc(str, len);
}

 void phper_smart_str_realloc(smart_str *str, size_t len) {
    smart_str_realloc(str, len);
}

 void phper_smart_str_free_ex(smart_str *str, bool persistent) {
    smart_str_free_ex(str, persistent);
}

 void phper_smart_str_append_escaped(smart_str *str, const char *s,
                                                  size_t l) {
    smart_str_append_escaped(str, s, l);
}

 void phper_smart_str_append_double(smart_str *str, double num,
                                                 int precision,
                                                 bool zero_fraction) {
    smart_str_append_double(str, num, precision, zero_fraction);
}

 void phper_smart_str_append_escaped_truncated(
    smart_str *str, const zend_string *value, size_t length) {
    smart_str_append_escaped_truncated(str, value, length);
}

 void phper_smart_str_append_scalar(smart_str *str,
                                                 const zval *value,
                                                 size_t truncate) {
    smart_str_append_scalar(str, value, truncate);
}

 void phper_smart_str_0(smart_str *str) {
    smart_str_0(str);
}

 size_t phper_smart_str_get_len(const smart_str *str) {
    return smart_str_get_len((smart_str *)str);
}

 zend_string *phper_smart_str_extract(smart_str *str) {
    return smart_str_extract(str);
}

 void phper_smart_str_appendc_ex(smart_str *dest, char ch,
                                              bool persistent) {
    smart_str_appendc_ex(dest, ch, persistent);
}

 void phper_smart_str_appendl_ex(smart_str *dest, const char *str,
                                              size_t len, bool persistent) {
    smart_str_appendl_ex(dest, str, len, persistent);
}

 void phper_smart_str_append_ex(smart_str *dest,
                                             const zend_string *src,
                                             bool persistent) {
    smart_str_append_ex(dest, src, persistent);
}

 void phper_smart_str_append_smart_str_ex(smart_str *dest,
                                                       const smart_str *src,
                                                       bool persistent) {
    smart_str_append_smart_str_ex(dest, src, persistent);
}

 void phper_smart_str_append_long_ex(smart_str *dest,
                                                  zend_long num,
                                                  bool persistent) {
    smart_str_append_long_ex(dest, num, persistent);
}

 void phper_smart_str_append_unsigned_ex(smart_str *dest,
                                                      zend_ulong num,
                                                      bool persistent) {
    smart_str_append_unsigned_ex(dest, num, persistent);
}

 void phper_smart_str_setl(smart_str *dest, const char *src,
                                        size_t len) {
    smart_str_setl(dest, src, len);
}