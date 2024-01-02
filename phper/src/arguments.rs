// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [zend_arg_info].

use std::ffi::CString;

use crate::utils::ensure_end_with_zero;

/// Function or method argument info.
pub struct Argument {
    pub(crate) name: CString,
    pub(crate) pass_by_ref: bool,
    pub(crate) required: bool,
}

impl Argument {
    /// Indicate the argument is pass by value.
    pub fn by_val(name: impl AsRef<str>) -> Self {
        let name = unsafe {
            let name = name.as_ref().as_bytes().to_vec();
            CString::from_vec_unchecked(name)
        };

        Self {
            name,
            pass_by_ref: false,
            required: true,
        }
    }

    /// Indicate the argument is pass by reference.
    pub fn by_ref(name: impl Into<String>) -> Self {
        let name = ensure_end_with_zero(name);
        Self {
            name,
            pass_by_ref: true,
            required: true,
        }
    }

    /// Indicate the argument is pass by value and is optional.
    pub fn by_val_optional(name: impl Into<String>) -> Self {
        let name = ensure_end_with_zero(name);
        Self {
            name,
            pass_by_ref: false,
            required: false,
        }
    }

    /// Indicate the argument is pass by reference nad is optional.
    pub fn by_ref_optional(name: impl Into<String>) -> Self {
        let name = ensure_end_with_zero(name);
        Self {
            name,
            pass_by_ref: true,
            required: false,
        }
    }
}
