#include <phper.h>

zval *phper_get_this(const zend_execute_data *execute_data) {
    return getThis();
}

size_t phper_zend_object_properties_size(const zend_class_entry *ce) {
    return zend_object_properties_size((zend_class_entry *)ce);
}

void *phper_zend_object_alloc(size_t obj_size, const zend_class_entry *ce) {
    return zend_object_alloc(obj_size, (zend_class_entry *)ce);
}

bool phper_object_init_ex(zval *arg, const zend_class_entry *class_type) {
    return object_init_ex(arg, (zend_class_entry *)class_type) == SUCCESS;
}

void phper_zend_object_release(zend_object *obj) {
    zend_object_release(obj);
}

uint32_t phper_zend_object_gc_refcount(const zend_object *obj) {
    return GC_REFCOUNT(obj);
}