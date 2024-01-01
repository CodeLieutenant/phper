// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use bindgen::Builder;
use std::{env, ffi::OsStr, fmt::Debug, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-env-changed=PHP_CONFIG");
    println!("cargo:rerun-if-changed=build.rs");
    let current_dir = std::env::current_dir().unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let c_files = std::fs::read_dir(current_dir.join("c"))
        .unwrap()
        .map(|file| file.unwrap())
        .map(|file| file.path().to_string_lossy().to_string())
        .collect::<Vec<_>>();

    c_files
        .iter()
        .for_each(|file| println!("cargo:rerun-if-changed={}", file));
    println!("cargo:rustc-link-search={}", out_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=phpwrapper");

    let php_config = env::var("PHP_CONFIG").unwrap_or_else(|_| "php-config".to_string());

    let includes = execute_command(&[php_config.as_str(), "--includes"]);
    let includes = includes.split(' ').collect::<Vec<_>>();

    let mut builder = cc::Build::new();

    includes.iter().for_each(|include| {
        builder.flag(include);
    });

    builder
        .cpp(false)
        .debug(false)
        .files(&c_files)
        .extra_warnings(true)
        .include("include")
        // .flag("-falign-functions")
        // .flag("-flto=auto")
        // .flag("-std=c2x") // Replace with -std=c23 after CLANG 18
        // .flag("-pedantic")
        // .flag("-Wno-ignored-qualifiers")
        // .force_frame_pointer(false)
        // .opt_level(3)
        .warnings(true)
        // .use_plt(false)
        .static_flag(true)
        .pic(true)
        .compile("phpwrapper");

    let mut builder = Builder::default()
        .header("include/phper.h")
        .allowlist_file("include/phper.h")
        // .allowlist_recursively(true)
        // Block the `zend_ini_parse_quantity` because it's document causes the doc test to fail.
        .blocklist_function("zend_ini_parse_quantity")
        .derive_hash(true)
        .clang_args(&includes)
        .derive_copy(true)
        .derive_eq(true)
        .derive_ord(true)
        .derive_partialeq(true)
        .derive_partialord(true)
        .derive_default(true);

    // iterate over the php include directories, and update the builder
    // to only create bindings from the header files in those directories
    for dir in includes.iter().map(|include| &include[2..]) {
        println!("cargo:include={}", dir);
        let p = PathBuf::from(dir).join(".*\\.h");
        builder = builder.allowlist_file(p.display().to_string());
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    // print!("cargo:warning={}", bindings.to_string());
    bindings
        .write_to_file(out_path.join("php_bindings.rs"))
        .expect("Unable to write output file");
}

fn execute_command<S: AsRef<OsStr> + Debug>(argv: &[S]) -> String {
    let mut command = Command::new(&argv[0]);
    command.args(&argv[1..]);
    let output = command
        .output()
        .unwrap_or_else(|_| panic!("Execute command {:?} failed", &argv))
        .stdout;
    String::from_utf8(output).unwrap().trim().to_owned()
}
