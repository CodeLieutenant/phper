#include <phper.h>

 void *phper_emalloc(size_t size) {
    return emalloc(size);
}

 void phper_efree(void *ptr) {
    efree(ptr);
}
