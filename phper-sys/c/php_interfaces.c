#include <phper.h>

zend_class_entry phper_init_interface_entry(const char *class_name,
                                            size_t class_name_len) {
    zend_class_entry class_ce = {0};
    INIT_CLASS_ENTRY_EX(class_ce, class_name, class_name_len, NULL);
    return class_ce;
}

zend_class_entry *
phper_register_interface_entry(zend_class_entry *ce,
                               const zend_function_entry *functions) {
    ce->info.internal.builtin_functions = functions;
    return zend_register_internal_interface(ce);
}

bool phper_instanceof_function(const zend_class_entry *instance_ce,
                               const zend_class_entry *ce) {
    return instanceof_function(instance_ce, ce) != 0;
}