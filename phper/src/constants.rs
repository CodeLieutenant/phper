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

use crate::modules::Registerer;
use crate::sys::*;
use crate::values::ZVal;
use bitflags::bitflags;

pub struct Constant {
    constant: zend_constant,
    flags: i32,
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
    pub fn new(name: impl AsRef<str>, value: impl Into<ZVal>, flags: Option<Flags>) -> Self {
        let name = name.as_ref();
        let length = name.len();
        let ptr = name.as_bytes().as_ptr() as *const i8;
        let flags = flags.unwrap_or(Flags::Cs | Flags::Persistent).bits() as i32;
        Self {
            constant: unsafe { phper_create_constant(ptr, length, value.into().inner, flags) },
            flags,
        }
    }
}

impl Registerer for Constant {
    fn register(&mut self, module_number: i32) -> Result<(), Box<dyn std::error::Error>> {
        let result =
            unsafe { phper_register_constant(&mut self.constant, self.flags, module_number) };

        if result == ZEND_RESULT_CODE_SUCCESS {
            Ok(())
        } else {
            Err("Failed to register new ZEND constant".into())
        }
    }
}
