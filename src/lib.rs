#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use widestring::*;

pub fn version() -> String {
    unsafe {
        let major = Everything_GetMajorVersion();
        let minor = Everything_GetMinorVersion();
        let revision = Everything_GetRevision();
        format!("{}.{}.{}", major, minor, revision)
    }
}

pub fn query_sync(query: &str) -> DWORD {
    let query_c_str = match U16CString::from_str(query) {
        Ok(res) => res,
        Err(_) => return 0,
    };
    unsafe {
        Everything_SetSearchW(query_c_str.as_ptr());
        Everything_SetRequestFlags(EVERYTHING_REQUEST_FILE_NAME | EVERYTHING_REQUEST_PATH);
        Everything_QueryW(true as BOOL);
        Everything_GetNumResults()
    }
}

#[cfg(test)]
mod tests {
    use widestring::u16cstr;

    use super::*;

    #[test]
    fn reports_version() {
        unsafe {
            let version = version();
            assert_eq!(version, "1.4.1");
        }
    }

    #[test]
    fn gets_results() {
        let num_results = query_sync("");
        println!("Everything num results: {}", num_results);
        assert!(num_results > 0);
    }
}
