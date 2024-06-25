#include <phper.h>

zend_resource *phper_register_persistent_resource(const zend_string *id,
                                                  const void *ptr, int le_id) {
    return zend_register_persistent_resource_ex((zend_string *)id, (void *)ptr,
                                                le_id);
}

int phper_zend_register_persistent_list_destructors(rsrc_dtor_func_t dtor,
                                                    const char *name,
                                                    int module_number) {
    return zend_register_list_destructors_ex(NULL, dtor, name, module_number);
}

int phper_zend_register_list_destructors(const rsrc_dtor_func_t dtor,
                                         const char *name, int module_number) {
    return zend_register_list_destructors_ex((rsrc_dtor_func_t)dtor, NULL, name,
                                             module_number);
}

int phper_zend_register_list_destructors_ex(const rsrc_dtor_func_t dtor,
                                            const rsrc_dtor_func_t pdtor,
                                            const char *name,
                                            int module_number) {

    return zend_register_list_destructors_ex(
        (rsrc_dtor_func_t)dtor, (rsrc_dtor_func_t)pdtor, name, module_number);
}

int phper_zend_fetch_list_dtor_id(const char *name) {
    return zend_fetch_list_dtor_id(name);
}

const zend_resource *phper_register_persistent_find(const char *hash,
                                                    size_t len) {
    zend_resource *zv = zend_hash_str_find_ptr(&EG(persistent_list), hash, len);

    if (zv == NULL) {
        php_error_docref(0, E_WARNING, "Invalid persistent resource");
        return NULL;
    }

    return zv;
}
