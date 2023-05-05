extern crate bindgen;

use std::env;
use std::fs::canonicalize;
use std::path::*;

fn main() {
    let everything_sdk_path = canonicalize(Path::new(r#"Everything-SDK\lib"#)).unwrap();
    println!("cargo:rustc-link-search={}", everything_sdk_path.display());
    println!("cargo:rustc-link-lib=Everything64");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
