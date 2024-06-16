// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

#include <phper.h>

// ==================================================
// module apis:
// ==================================================

const char *phper_get_zend_module_build_id() {
    return ZEND_MODULE_BUILD_ID;
}

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

// ==================================================
// Argument API:
// ==================================================

zend_internal_arg_info
phper_zend_begin_arg_info_ex(bool return_reference,
                             uintptr_t required_num_args) {
#define static
#define const
    ZEND_BEGIN_ARG_INFO_EX(info, 0, return_reference, required_num_args)
    ZEND_END_ARG_INFO()
    return info[0];
#undef static
#undef const
}

zend_internal_arg_info phper_zend_arg_info(bool pass_by_ref, const char *name) {
    zend_internal_arg_info info[] = {ZEND_ARG_INFO(pass_by_ref, )};
    info[0].name = name;
    return info[0];
}