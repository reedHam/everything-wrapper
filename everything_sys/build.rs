use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // Code modified from https://rust-lang.github.io/rust-bindgen/tutorial-3.html
    let everything_sdk_path = fs::canonicalize(Path::new(r#"..\Everything-SDK\lib"#)).unwrap();
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
    let target_feature = env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default();
    let is_release = target_feature.contains("crt-static");

    let profile = if is_release { "release" } else { "debug" };

    let dll_path = Path::new(r"C:\github\everything-wrapper\Everything-SDK\dll\Everything64.dll");

    let target_path = Path::new(&target_dir)
        .join(profile)
        .join("deps")
        .join("Everything64.dll");

    if !target_path.exists() {
        fs::copy(&dll_path, &target_path).expect(&format!(
            "Failed to copy Everything.dll from {:?} to {:?}",
            dll_path, target_path
        ));
    }
}
