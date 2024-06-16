// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

#![warn(rust_2018_idioms)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![doc = include_str!("../README.md")]
// #![feature(unboxed_closures)]

#[macro_use]
mod macros;

pub mod arrays;
pub mod classes;
<<<<<<< HEAD
pub mod arguments;
pub(crate) mod constants;
=======
pub mod constants;
>>>>>>> 217b34537f1e0f2d5fd7803bc1be241a5a1ac763
pub mod errors;
pub mod functions;
pub mod ini;
pub mod modules;
pub mod objects;
pub mod output;
pub mod references;
pub mod resources;
pub mod smart_str;
pub mod strings;
pub mod types;
mod utils;
pub mod values;

pub use crate::errors::{ok, Error, Result};
pub use phper_alloc as alloc;
pub use phper_macros::*;
pub use phper_sys as sys;
