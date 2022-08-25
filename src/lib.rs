#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod EverythingSDK {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod everything {
    use super::EverythingSDK::*;
    use widestring::*;

    pub fn version() -> String {
        unsafe {
            let major = Everything_GetMajorVersion();
            let minor = Everything_GetMinorVersion();
            let revision = Everything_GetRevision();
            format!("{}.{}.{}", major, minor, revision)
        }
    }

    pub fn set_search(search: &str) {
        let search_c_str = match U16CString::from_str(search) {
            Ok(res) => res,
            Err(_) => panic!("Could not convert query to U16CString"),
        };

        unsafe {
            Everything_SetSearchW(search_c_str.as_ptr());
        }
    }

    pub fn query(query: &str) -> DWORD {
        unsafe {
            Everything_QueryW(true as BOOL);
            Everything_GetNumResults()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::everything::*;

    #[test]
    fn reports_version() {
        let version = version();
        assert_eq!(version, "1.4.1");
    }

    #[test]
    fn queries() {
        let num_results = query("");
        println!("Everything num results: {}", num_results);
        assert!(num_results > 0);
    }
}
