#include <phper.h>

zend_constant phper_create_constant(const char *name, size_t name_len, zval val,
                                    int flags) {
    zend_constant c = {
        .name =
            zend_string_init_interned(name, name_len, flags & CONST_PERSISTENT),
        .value = val,
    };

    return c;
}

zend_result phper_register_constant(zend_constant *constant, int flags,
                                    int module_number) {
    ZEND_CONSTANT_SET_FLAGS(constant, flags, module_number);
    return zend_register_constant(constant);
}