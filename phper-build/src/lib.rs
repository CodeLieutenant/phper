// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

#![warn(rust_2018_idioms, missing_docs)]
#![warn(clippy::dbg_macro)]
#![doc = include_str!("../README.md")]

use bindgen::Builder;
use cc::Build;
use phper_sys::*;
use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

const GEN_STUB_PHP: &str = include_str!("../gen_stub.php");

/// Register all php build relative configure parameters, used in `build.rs`.
pub fn register_all() {
    register_link_args();
    register_configures();
}

/// Register useful rust cfg for project using phper.
pub fn register_configures() {
    // versions
    println!(
        "cargo:rustc-cfg=phper_major_version=\"{}\"",
        PHP_MAJOR_VERSION
    );
    println!(
        "cargo:rustc-cfg=phper_minor_version=\"{}\"",
        PHP_MINOR_VERSION
    );
    println!(
        "cargo:rustc-cfg=phper_release_version=\"{}\"",
        PHP_RELEASE_VERSION
    );

    if PHP_DEBUG != 0 {
        println!("cargo:rustc-cfg=phper_debug");
    }

    if USING_ZTS != 0 {
        println!("cargo:rustc-cfg=phper_zts");
    }
}

/// Register link arguments for os-specified situation.
pub fn register_link_args() {
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-arg=-undefined");
        println!("cargo:rustc-link-arg=dynamic_lookup");
    }
}

fn execute_command<S: AsRef<OsStr>>(argv: &[S]) -> Result<String, Box<dyn std::error::Error>> {
    let mut command = Command::new(&argv[0]);
    command.args(&argv[1..]);
    let output = command.output()?.stdout;
    Ok(String::from_utf8(output)?.trim().to_owned())
}

fn create_builder() -> Result<(Build, Builder), Box<dyn std::error::Error>> {
    let php_config = env::var("PHP_CONFIG").unwrap_or_else(|_| "php-config".to_string());

    let includes = execute_command(&[php_config.as_str(), "--includes"])?;
    let includes = includes.split(' ').collect::<Vec<_>>();

    let builder = Builder::default()
        .derive_debug(true)
        .clang_args(&includes)
        .generate_inline_functions(true)
        .generate_block(true)
        .generate_comments(true)
        .allowlist_type("zend_internal_arg_info")
        .allowlist_type("zend_type")
        .allowlist_var("arginfo_*")
        .wrap_unsafe_ops(true)
        .array_pointers_in_arguments(true)
        .generate_cstr(true);

    let mut cc = cc::Build::new();

    for dir in includes.iter() {
        cc.flag(dir);
    }

    cc.cpp(false)
        .debug(false)
        .extra_warnings(false)
        .warnings(false)
        .flag("-std=c2x") // Replace with -std=c23 after CLANG 18
        .force_frame_pointer(false)
        .opt_level(3)
        .use_plt(false)
        .static_flag(true)
        .pic(true);

    Ok((cc, builder))
}

/// Includes php bindings for function/method arguments
pub fn generate_php_function_args<P: AsRef<Path>, Q: AsRef<Path>>(
    output_dir: P,
    dirs: &[Q],
    php_exec: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = output_dir.as_ref();
    let gen_stub_php = output_dir.join("gen_stub.php");
    std::fs::write(&gen_stub_php, GEN_STUB_PHP)?;

    let gen_stub_php = gen_stub_php.as_os_str().to_str().unwrap();

    for dir in dirs {
        let dir_name =  dir.as_ref().to_str().unwrap();

        std::process::Command::new(php_exec.unwrap_or("php"))
            .args([gen_stub_php, dir_name])
            .output()?;
    }

    let mut header = String::with_capacity(64 * 1024);
    let mut c_file = String::with_capacity(64 * 1024);

    header.push_str("#pragma once\n\n#include <php.h>\n\n",);
    c_file.push_str("#include <php.h>\n\nBEGIN_EXTERN_C()\n\n");

    for dir in dirs {
        let dir = dir.as_ref();

        let files = WalkDir::new(dir).follow_links(false);

        for file in files {
            let file = file?;
            let path = file.path();

            if path.is_file() && path.extension() == Some(OsStr::new("php")) {
                println!("cargo:rerun-if-changed={}", path.display());
            }

            if file.file_type().is_dir() || path.extension() != Some(OsStr::new("h")) {
                continue;
            }

            let contents = std::fs::read_to_string(path)?;

            match extract_arginfo_size_and_name(&contents) {
                Some(vals) => write_header(&mut header, &vals),
                None => continue,
            }

            c_file.push_str(&contents);
            std::fs::remove_file(path)?;
        }
    }

    c_file.push_str("END_EXTERN_C()\n\n");

    let php_args_binding_h_path = output_dir.join("php_args_bindings.h");
    std::fs::write(&php_args_binding_h_path, &header)?;

    let php_args_binding_c_path = output_dir.join("php_args_bindings.c");
    std::fs::write(&php_args_binding_c_path, format!("\n\n{}\n\n", c_file))?;

    let (mut cc, builder) = create_builder()?;

    cc.file(&php_args_binding_c_path)
        .include(output_dir)
        .compile("php_args_bindings");

    builder
        .header(php_args_binding_h_path.to_str().unwrap())
        .allowlist_file(php_args_binding_h_path.to_str().unwrap())
        .generate()?
        .write_to_file(output_dir.join("php_args_bindings.rs"))?;

    Ok(())
}

fn write_header(header: &mut String, content: &[(&str, usize)]) {
    content.iter().for_each(|(name, count)| {
        header.push_str("extern zend_internal_arg_info ");
        header.push_str(name);
        header.push('[');
        header.push_str(count.to_string().as_str());
        header.push_str("];\n\n");
    })
}

fn extract_arginfo_size_and_name(input: &str) -> Option<Vec<(&str, usize)>> {
    let mut result = Vec::new();

    let mut name = "";
    let mut counter = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.starts_with("ZEND_BEGIN_") {
            let start = line.find("arginfo_")?;
            let end = line.find(',')?;

            name = &line[start..end];
        }

        if line.starts_with("ZEND_ARG_") {
            counter += 1;
        }

        if line.starts_with("ZEND_END_ARG_INFO") {
            result.push((name, counter + 1));
            counter = 0;
            name = "";
        }
    }

    Some(result)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_write_header() {
        let mut header = String::new();
        let content = vec![
            ("arginfo_Complex_say_hello", 1),
            ("arginfo_Complex_say_hello2", 5),
        ];

        write_header(&mut header, &content);

        assert_eq!(
            header,
            r#"extern zend_internal_arg_info arginfo_Complex_say_hello[1];

extern zend_internal_arg_info arginfo_Complex_say_hello2[5];

"#
        );
    }

    #[test]
    fn test_extract_arginfo_size_and_name() {
        let input = r#"
        #pragma once

        #include <php.h>

        ZEND_BEGIN_ARG_WITH_RETURN_TYPE_INFO_EX(arginfo_Complex_say_hello, 0, 1,
                                                IS_STRING, 0)
        ZEND_ARG_TYPE_INFO(0, name, IS_STRING, 0)
        ZEND_END_ARG_INFO()


        ZEND_BEGIN_ARG_WITH_RETURN_TYPE_INFO_EX(arginfo_Complex_say_hello2, 0, 1,
                                                IS_STRING, 0)
        ZEND_ARG_TYPE_INFO(0, name, IS_STRING, 0)
        ZEND_ARG_TYPE_INFO(0, name, IS_STRING, 0)
        ZEND_ARG_TYPE_INFO(0, name, IS_STRING, 0)
        ZEND_ARG_TYPE_INFO(0, name, IS_STRING, 0)
        ZEND_END_ARG_INFO()
        "#;

        let result = extract_arginfo_size_and_name(input);

        assert!(result.is_some());

        let result = result.unwrap();

        assert_eq!(result.len(), 2);

        assert_eq!("arginfo_Complex_say_hello", result[0].0);
        assert_eq!(2, result[0].1);

        assert_eq!("arginfo_Complex_say_hello2", result[1].0);
        assert_eq!(5, result[1].1)
    }
}
