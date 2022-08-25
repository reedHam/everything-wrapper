#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use widestring::U16CString;

#[cfg(test)]
mod tests {
    use widestring::u16cstr;

    use super::*;

    #[test]
    fn version() {
        unsafe {
            let major = Everything_GetMajorVersion();
            let minor = Everything_GetMinorVersion();
            let revision = Everything_GetRevision();
            println!("Everything major version: {}", major);
            println!("Everything minor version: {}", minor);
            println!("Everything revision: {}", revision);
            assert_eq!(major, 1);
            assert_eq!(minor, 4);
            assert_eq!(revision, 1);
        }
    }

    #[test]
    fn query() {
        unsafe {
            Everything_SetSearchW(U16CString::from(u16cstr!("")).as_ptr());
            Everything_SetRequestFlags(EVERYTHING_REQUEST_FILE_NAME | EVERYTHING_REQUEST_PATH);
            Everything_QueryW(true as BOOL);
            let num_results = Everything_GetNumResults();
            println!("Everything num results: {}", num_results);
            assert!(num_results > 0);
        }
    }
}
