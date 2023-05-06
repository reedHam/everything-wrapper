# Everything SDK Rust Wrapper

## Requirements
- Everything SDK 1.4.1 
- Windows 64-bit
- Running Everything Process
- Rust

## Overview
- Uses rust bindgen to generate bindings to the Everything SDK C API

### Rust modules
- `everything-sys` - Rust bindings to the Everything SDK C API
- `everything` - A more ergonomic wrapper around the everything-sys

## Documentation
- [Everything SDK](https://www.voidtools.com/support/everything/sdk/)
- [Rust Bindgen](https://rust-lang.github.io/rust-bindgen/)
- [Rust FFI](https://doc.rust-lang.org/nomicon/ffi.html)

## Building
After creating the target directory, you must copy the dll file to the target directory. 
The dll file can be found in the Everything SDK installation directory.  
The dll file is named `Everything64.dll`  

Currently the build scripts have a hardcoded path to the dll file that works on my machine any PRs to fix this are welcome.

## node-everything
This is a simple nodejs ffi wrapper around the Everything SDK.
This is a work in progress and is not ready for use and should only be used as an example.
Currently it dose not parse file paths very efficiently.