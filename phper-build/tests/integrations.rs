use std::env::current_dir;

use phper_build::generate_php_function_args;

struct Files<'a>(&'a std::path::PathBuf);

impl<'a> Drop for Files<'a> {
    fn drop(&mut self) {
        std::fs::remove_file(self.0.join("php_args_bindings.c")).unwrap();
        std::fs::remove_file(self.0.join("php_args_bindings.h")).unwrap();
        std::fs::remove_file(self.0.join("php_args_bindings.rs")).unwrap();
    }
}

#[test]
pub fn test_generate_function_args() {
    let current_dir = current_dir().unwrap();
    // let _files = Files(&current_dir);

    let stubs_dir = current_dir.join("tests").join("stubs");

    assert!(generate_php_function_args(&current_dir, &[&stubs_dir], None).is_ok());
    // assert!(current_dir.join("php_args_bindings.h").exists());
    // assert!(current_dir.join("php_args_bindings.c").exists());
    // assert!(current_dir.join("php_args_bindings.rs").exists());
    // assert!(current_dir.join("php_args_bindings.a").exists());
}
