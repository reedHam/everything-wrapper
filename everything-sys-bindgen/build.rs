fn main() {
    println!("cargo:rustc-link-search=Everything-SDK\\lib");
    println!("cargo:rustc-link-lib=dylib=Everything64");
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

    bindings
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");
}
