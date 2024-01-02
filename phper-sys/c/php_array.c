#include <phper.h>

zval *phper_zend_hash_str_update(HashTable *ht, const char *key, size_t len,
                                 zval *pData) {
    return zend_hash_str_update(ht, key, len, pData);
}

zval *phper_zend_hash_index_update(HashTable *ht, zend_ulong h, zval *pData) {
    return zend_hash_index_update(ht, h, pData);
}

zval *phper_zend_hash_next_index_insert(HashTable *ht, zval *pData) {
    return zend_hash_next_index_insert(ht, pData);
}

void phper_array_init(zval *arg) {
    array_init(arg);
}

void *phper_zend_hash_str_find_ptr(const HashTable *ht, const char *str,
                                   size_t len) {
    return zend_hash_str_find_ptr(ht, str, len);
}

bool phper_zend_hash_str_exists(const HashTable *ht, const char *str,
                                size_t len) {
    return zend_hash_str_exists(ht, str, len) != 0;
}

bool phper_zend_hash_index_exists(const HashTable *ht, zend_ulong h) {
    return zend_hash_index_exists(ht, h) != 0;
}

zend_array *phper_zend_new_array(uint32_t size) {
    return zend_new_array(size);
}

zend_array *phper_zend_array_dup(zend_array *source) {
    return zend_array_dup(source);
}

zval *phper_zend_hash_index_find(const HashTable *ht, zend_ulong h) {
    return zend_hash_index_find(ht, h);
}

bool phper_zend_hash_index_del(HashTable *ht, zend_ulong h) {
    return zend_hash_index_del(ht, h) == SUCCESS;
}

zval *phper_zend_symtable_str_update(HashTable *ht, const char *str, size_t len,
                                     zval *pData) {
    return zend_symtable_str_update(ht, str, len, pData);
}

bool phper_zend_symtable_str_del(HashTable *ht, const char *str, size_t len) {
    return zend_symtable_str_del(ht, str, len) == SUCCESS;
}

zval *phper_zend_symtable_str_find(HashTable *ht, const char *str, size_t len) {
    return zend_symtable_str_find(ht, str, len);
}

bool phper_zend_symtable_str_exists(HashTable *ht, const char *str,
                                    size_t len) {
    return zend_symtable_str_exists(ht, str, len) != 0;
}

zval *phper_zend_str_update(HashTable *ht, const char *str, size_t len,
                            zval *pData) {
    return zend_hash_str_update(ht, str, len, pData);
}

bool phper_zend_str_del(HashTable *ht, const char *str, size_t len) {
    return zend_hash_str_del(ht, str, len) == SUCCESS;
}

zval *phper_zend_str_find(HashTable *ht, const char *str, size_t len) {
    return zend_hash_str_find(ht, str, len);
}

bool phper_zend_str_exists(HashTable *ht, const char *str, size_t len) {
    return zend_hash_str_exists(ht, str, len) != 0;
}
