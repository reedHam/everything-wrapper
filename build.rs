extern crate bindgen;

use std::env;
use std::fs::canonicalize;
use std::path::*;

fn main() {
    // Code modified from https://rust-lang.github.io/rust-bindgen/tutorial-3.html
    // Tell cargo to look for shared libraries in the specified directory
    let everything_sdk_path = canonicalize(Path::new(r#"Everything-SDK\lib"#)).unwrap();
    println!("cargo:rustc-link-search={}", everything_sdk_path.display());

    // shared library.
    println!("cargo:rustc-link-lib=Everything64");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // Only generate bindings for the everything API
    let everything_regex_filter = "Everything.*|EVERYTHING.*";

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .allowlist_function(everything_regex_filter)
        .allowlist_type(everything_regex_filter)
        .allowlist_var(everything_regex_filter)
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
