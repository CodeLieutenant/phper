#include <phper.h>

zend_class_entry *
phper_init_class_entry_ex(const char *class_name, size_t class_name_len,
                          const zend_function_entry *functions,
                          phper_init_class_entry_handler handler,
                          void *argument) {
    zend_class_entry class_ce;
    INIT_CLASS_ENTRY_EX(class_ce, class_name, class_name_len, functions);
    return handler(&class_ce, argument);
}

bool phper_instanceof_function(const zend_class_entry *instance_ce,
                               const zend_class_entry *ce) {
    return instanceof_function(instance_ce, ce) != 0;
}
