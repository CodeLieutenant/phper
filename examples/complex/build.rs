// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use std::env;
use std::path::PathBuf;
use bindgen::Builder;
use walkdir::WalkDir;

fn main() {
    phper_build::register_configures();

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-arg=-undefined");
        println!("cargo:rustc-link-arg=dynamic_lookup");
    }

    let current_dir = env::current_dir().unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let stubs = current_dir.join("stubs");


    for entry in WalkDir::new(stubs) {
        println!("{}", entry.unwrap().path().display());
    }


    let mut builder = Builder::default();

    // for dir in PathBuf::from(stubs).iter().map(|include| &include[2..]) {
    //     println!("cargo:include={}", dir);
    //     let p = PathBuf::from(dir).join(".*\\.h");
    //     builder = builder.allowlist_file(p.display().to_string());
    // }

}
