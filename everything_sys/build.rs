extern crate bindgen;

use std::fs::canonicalize;
use std::path::*;
use std::{env, fs};

fn main() {
    // Code modified from https://rust-lang.github.io/rust-bindgen/tutorial-3.html
    let everything_sdk_path = canonicalize(Path::new(r#"..\Everything-SDK\lib"#)).unwrap();
    println!("cargo:rustc-link-search={}", everything_sdk_path.display());

    println!("cargo:rustc-link-lib=Everything64");

    println!("cargo:rerun-if-changed=wrapper.h");

    let everything_regex_filter = "Everything.*|EVERYTHING.*";

    let bindings = bindgen::Builder::default()
        .allowlist_function(everything_regex_filter)
        .allowlist_type(everything_regex_filter)
        .allowlist_var(everything_regex_filter)
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let profile = env::var("PROFILE").expect("PROFILE environment variable not found");
    let dll_path = Path::new("../Everything-SDK/dll/Everything64.dll");

    let target_path = Path::new(&target_dir)
        .join(profile)
        .join("deps")
        .join("Everything64.dll");

    if !target_path.exists() {
        fs::copy(dll_path, target_path).expect("Failed to copy Everything.dll");
    }
}
