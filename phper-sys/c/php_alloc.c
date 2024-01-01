#include <phper.h>

ZEND_FASTCALL void *phper_emalloc(size_t size) {
    return emalloc(size);
}

ZEND_FASTCALL void phper_efree(void *ptr) {
    efree(ptr);
}
