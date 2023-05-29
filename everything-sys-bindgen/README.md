# everything-sys-bindgen
Rust bindings to the Everything SDK C API using bindgen.

You will get a `(exit code: 0xc0000135, STATUS_DLL_NOT_FOUND)` error if you try to run the program without the Everything SDK DLLs in root directory of the project.  
You can find the dll in the [Everything SDK](https://www.voidtools.com/support/everything/sdk/) download under the dll dir. 
## References
- [Everything SDK](https://www.voidtools.com/support/everything/sdk/)
- [Rust Bindgen](https://rust-lang.github.io/rust-bindgen/)
- [Rust FFI](https://doc.rust-lang.org/nomicon/ffi.html)
