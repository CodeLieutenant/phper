#include <phper.h>

zend_class_entry *
phper_register_interface_entry_ex(zend_class_entry *ce,
                                  const zend_function_entry *functions) {
    ce->info.internal.builtin_functions = functions;
    return zend_register_internal_interface(ce);
}

bool phper_instanceof_function(const zend_class_entry *instance_ce,
                               const zend_class_entry *ce) {
    return instanceof_function(instance_ce, ce) != 0;
}