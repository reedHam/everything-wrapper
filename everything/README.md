# Everything  
This crate provides a safe wrapper around the `everything_sys`.  
`everything_sys` is a rust binding to the [Everything SDK](https://www.voidtools.com/support/everything/sdk/) that allow IPC communication to the everything service.  
The Everything service indexes files on windows and provides a expressive query syntax to search for files.  
See the [Everything SDK documentation](https://www.voidtools.com/support/everything/sdk/) for more information.  

# Example
```rust
use everything::{Everything, EverythingRequestFlags, EverythingSort};

let mut everything = Everything::new();

everything.set_search("test");

everything.set_request_flags(
    EverythingRequestFlags::FullPathAndFileName
    | EverythingRequestFlags::Size
    | EverythingRequestFlags::DateCreated
);

everything.set_sort(EverythingSort::DateCreatedDescending);

everything.query().unwrap();

let num_results = everything.get_num_results();

assert!(num_results > 0);

for (i, path) in everything.full_path_iter().flatten().enumerate() {
    let size = everything.get_result_size(i as u32).unwrap();
    let date_created = everything.get_result_created_date(i as u32).unwrap();
    println!("{}: {} {} {}", i, path, size, date_created);
}
```
