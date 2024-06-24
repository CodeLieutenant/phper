// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [zend_array].

mod entry;
mod iterators;
mod keys;
mod zarr;
mod zarray;

use std::{
    fmt::{self, Debug},
};


pub use iterators::{Iter, IterKey, IterMut};
pub use keys::{InsertKey, Key};
pub use zarr::ZArr;
pub use zarray::ZArray;

fn common_fmt(this: &ZArr, f: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result {
    struct Debugger<'a>(&'a ZArr);

    impl Debug for Debugger<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_map().entries(self.0.iter()).finish()
        }
    }

    let zd = Debugger(this);

    f.debug_tuple(name).field(&zd).finish()
}
