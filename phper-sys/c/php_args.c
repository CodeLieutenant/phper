#include <php.h>

ZEND_FASTCALL zend_internal_arg_info phper_zend_arg_info(
    const char *name, int32_t type, bool return_reference, bool is_variadic,
    bool is_tentative, uintptr_t required_num_args) {
    zend_internal_arg_info info = {
        .name = name,
        .type = ZEND_TYPE_INIT_MASK(
            type |
            _ZEND_ARG_INFO_FLAGS(return_reference, is_variadic, is_tentative)),
    };

    return info;
}
