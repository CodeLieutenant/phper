#include <phper.h>

zend_class_entry phper_init_class_entry(const char *class_name,
                                        size_t class_name_len) {

    zend_class_entry class_ce = {0};
    INIT_CLASS_ENTRY_EX(class_ce, class_name, class_name_len, NULL);
    return class_ce;
}

zend_class_entry *
phper_register_class_entry(zend_class_entry *ce, zend_class_entry *parent,
                           const zend_function_entry *functions) {
    ce->info.internal.builtin_functions = functions;

    if (parent == NULL) {
        return zend_register_internal_class(ce);
    }

    return zend_register_internal_class_ex(ce, parent);
}
