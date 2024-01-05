// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [zend_constant](crate::sys::zend_constant).

use crate::strings::ZString;
use crate::values::ZVal;
use crate::{sys::*};
use bitflags::bitflags;
use std::ffi::{c_char, c_int};

pub struct Constant {
    name: String,
    value: ZVal,
    flags: Flags,
}

// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Flags(u32);

impl Default for Flags {
    fn default() -> Self {
        Self::Cs | Self::Persistent
    }
}

bitflags! {
    impl Flags: u32 {
        const Cs = CONST_CS;
        const Persistent = CONST_PERSISTENT;
        const NoFileCache = CONST_NO_FILE_CACHE;
        const Deprecated = CONST_DEPRECATED;
        const Owned = CONST_OWNED;
    }
}

impl Constant {
    pub fn new(name: impl Into<String>, value: impl Into<ZVal>, flags: Option<Flags>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            flags: flags.unwrap_or_default(),
        }
    }

    pub(crate) fn register(&self, module_number: c_int) {
        let flags = self.flags.bits() as c_int;

        unsafe {
            let mut name = ZString::new(self.name.clone());
            let mut constant = _zend_constant{
                value: self.value.clone().into_inner(),
                name: name.as_mut_ptr(),
            };

            zend_register_constant(&mut constant);

            // match  {
            //     Scalar::Null => {
            //         zend_register_null_constant(name_ptr, name_len, flags, module_number)
            //     }
            //     Scalar::Bool(b) => zend_register_bool_constant(
            //         name_ptr,
            //         name_len,
            //         *b as zend_bool,
            //         flags,
            //         module_number,
            //     ),
            //     Scalar::I64(i) => zend_register_long_constant(
            //         name_ptr,
            //         name_len,
            //         *i as zend_long,
            //         flags,
            //         module_number,
            //     ),
            //     Scalar::F64(f) => {
            //         zend_register_double_constant(name_ptr, name_len, *f, flags, module_number)
            //     }
            //     Scalar::String(s) => {
            //         let s_ptr = s.as_ptr() as *mut u8;
            //         zend_register_stringl_constant(
            //             name_ptr,
            //             name_len,
            //             s_ptr.cast(),
            //             s.len(),
            //             flags,
            //             module_number,
            //         )
            //     }
            //     Scalar::Bytes(s) => {
            //         let s_ptr = s.as_ptr() as *mut u8;
            //         zend_register_stringl_constant(
            //             name_ptr,
            //             name_len,
            //             s_ptr.cast(),
            //             s.len(),
            //             flags,
            //             module_number,
            //         )
            //     }
            // }
        }
    }
}
