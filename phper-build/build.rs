use std::env;
use std::env::current_dir;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=gen_stub.php");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    std::fs::copy(
        current_dir().unwrap().join("gen_stub.php"),
        out_path.join("gen_stub.php"),
    ).unwrap();
}
