# Everything SDK Rust Wrapper

## Requirements
- Everything SDK 1.4.1 
- Windows 64-bit

## Development Requirements
- clang 14.0.5 x86_64-pc-windows-msvc
- Rust 1.62.1 stable-x86_64-pc-windows-msvc
- Visual Studio 2019

## Overview
- Uses rust bindgen to generate bindings to the Everything SDK C API

## Documentation
- [Everything SDK](https://www.voidtools.com/support/everything/sdk/)
- [Rust Bindgen](https://rust-lang.github.io/rust-bindgen/)
- [Rust FFI](https://doc.rust-lang.org/nomicon/ffi.html)

## Building
After creating the target directory, you must copy the dll file to the target directory.  
The dll file can be found in the Everything SDK installation directory.  
The dll file is named `Everything64.dll`  

## Todo 
- [ ] Research advantages of building from source instead of using the dll.
- [ ] Research building from source with rust bindgen. 
- [ ] Add more tests to ensure that the wrapper is working as expected.
- [ ] Add util functions for converting between rust types and c types.
- [ ] Add documentation to the wrapper.
- [ ] Add script for moving the dll file to the target directory.

