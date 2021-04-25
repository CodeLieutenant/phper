use phper_test::test_php_scripts_with_condition;
use std::{env, path::Path, str};

#[test]
fn test_php() {
    test_php_scripts_with_condition(
        env!("CARGO_BIN_EXE_log"),
        &[Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("php")
            .join("test_php_say.php")],
        |output| {
            let stdout = str::from_utf8(&output.stdout).unwrap();
            stdout == "Hello, world!" && output.status.success()
        },
    );

    test_php_scripts_with_condition(
        env!("CARGO_BIN_EXE_log"),
        &[Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("php")
            .join("test_php_notice.php")],
        |output| {
            let stdout = str::from_utf8(&output.stdout).unwrap();
            stdout.contains("Notice:")
                && stdout.contains("Something happened: just for test")
                && output.status.success()
        },
    );

    test_php_scripts_with_condition(
        env!("CARGO_BIN_EXE_log"),
        &[Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("php")
            .join("test_php_warning.php")],
        |output| {
            let stdout = str::from_utf8(&output.stdout).unwrap();
            stdout.contains("Warning:")
                && stdout.contains("Something warning: just for test")
                && output.status.success()
        },
    );

    test_php_scripts_with_condition(
        env!("CARGO_BIN_EXE_log"),
        &[Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("php")
            .join("test_php_error.php")],
        |output| {
            let stdout = str::from_utf8(&output.stdout).unwrap();
            stdout.contains("Fatal error:")
                && stdout.contains("Something gone failed: just for test")
        },
    );

    test_php_scripts_with_condition(
        env!("CARGO_BIN_EXE_log"),
        &[Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("php")
            .join("test_php_deprecated.php")],
        |output| {
            let stdout = str::from_utf8(&output.stdout).unwrap();
            stdout.contains("Deprecated:")
                && stdout.contains("Something deprecated: just for test")
                && output.status.success()
        },
    );
}
