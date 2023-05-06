#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use widestring::U16CString;

    // Utility function for wide string conversion
    fn to_wide(s: &str) -> U16CString {
        U16CString::from_str(s).unwrap()
    }

    #[test]
    fn test_everything_version() {
        let version: DWORD;

        unsafe {
            version = Everything_GetMajorVersion();
        }

        assert!(version > 0);
    }

    #[test]
    fn test_everything_set_search_w() {
        let search_string = "test";
        let wide_search_string = to_wide(search_string);

        unsafe {
            Everything_SetSearchW(wide_search_string.as_ptr());
        }
    }

    fn test_search() {
        let search_string = "test";
        let wide_search_string = to_wide(search_string);

        unsafe {
            Everything_SetSearchW(wide_search_string.as_ptr());
            let result = Everything_QueryW(1);

            assert_eq!(result, 1);
        }
    }

    #[test]
    fn test_everything_query_w() {
        test_search();
    }

    #[test]
    fn test_everything_get_num_results() {
        let num_results: DWORD;

        test_search();
        unsafe {
            num_results = Everything_GetNumResults();
        }

        println!("num_results: {}", num_results);

        assert!(num_results > 0);
    }

    #[test]
    fn test_everything_get_result_path_w() {
        let index: DWORD = 0;
        let result;
        test_search();
        unsafe {
            result = Everything_GetResultPathW(index);
        }

        assert!(!result.is_null());
    }

    #[test]
    fn test_everything_get_result_file_name_w() {
        let index: DWORD = 0;
        let result;
        test_search();
        unsafe {
            result = Everything_GetResultFileNameW(index);
        }

        assert!(!result.is_null());
    }

    #[test]
    fn test_everything_get_result_full_path_and_file_name_w() {
        let index: DWORD = 0;
        let buf: [WCHAR; 260] = [0; 260];
        let size: DWORD;
        test_search();
        unsafe {
            size = Everything_GetResultFullPathNameW(
                index,
                buf.as_ptr() as *mut _,
                buf.len() as DWORD,
            );
        }

        assert!(size > 0);
    }

    #[test]
    fn test_everything_reset() {
        unsafe {
            Everything_Reset();
        }
    }
}
