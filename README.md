# Everything SDK Wrapper

## Requirements
- Everything SDK 1.4.1 
- Windows 64-bit
- Running Everything Process
- Rust

## Overview
- Uses rust bindgen to generate bindings to the Everything SDK C API

### Rust modules
- `everything-sys-bindgen` - Rust bindings to the Everything SDK C API
- `everything-rs` - A more ergonomic wrapper around the everything-sys-bindgen

## Documentation
- [Everything SDK](https://www.voidtools.com/support/everything/sdk/)
- [Rust Bindgen](https://rust-lang.github.io/rust-bindgen/)
- [Rust FFI](https://doc.rust-lang.org/nomicon/ffi.html)

### Usage
```rust
use everything::{Everything, EverythingSort, EverythingRequestFlags};

let everything = Everything::new();
everything.set_search("test");

everything.set_max_results(10);

everything.set_result_offset(10);

everything.set_request_flags(
    EverythingRequestFlags::FullPathAndFileName
    | EverythingRequestFlags::DateCreated
    | EverythingRequestFlags::DateModified
    | EverythingRequestFlags::Size
    | EverythingRequestFlags::Extension
);

everything.set_sort(EverythingSort::DateCreatedDescending);

everything.query().unwrap();

let num_results = everything.get_result_count();

for path in everything.full_path_iter().flatten() {
    println!("{}", path);
}

```

For more examples see the test modules in the everything-sys-bindgen and everything crates.
- https://github.com/reedHam/everything-wrapper/blob/27c3f93f6f30bce0a68e61816c428a0d77fb8348/everything-sys-bindgen/src/lib.rs#L8
- https://github.com/reedHam/everything-wrapper/blob/27c3f93f6f30bce0a68e61816c428a0d77fb8348/everything/src/lib.rs#L492


## Building
After creating the target directory, you must copy the dll file to the target directory. 
The dll file can be found in the Everything SDK installation directory.  
The dll file is named `Everything64.dll`  

## node-everything
This is a simple nodejs ffi wrapper around the Everything SDK.
This is a work in progress and is not ready for use and should only be used as an example.
Currently it dose not parse file paths very efficiently.

## TODO
- [x] Add more tests
- [x] Add more documentation
- [x] Add more examples
- [ ] Async queries
- [ ] Automated build process
- [ ] Publish to crates.io
