#include <phper.h>

zend_class_entry *
phper_register_class_entry(zend_class_entry *(*create_ce)(void),
                           const zend_function_entry *functions,
                           zend_object *(*create_object)(zend_class_entry *)) {

    zend_class_entry *ce = create_ce();

    ce->info.internal.builtin_functions = functions;

    zend_register_functions(ce, ce->info.internal.builtin_functions,
                            &ce->function_table, EG(current_module)->type);

    ce->create_object = create_object;

    return ce;
}
