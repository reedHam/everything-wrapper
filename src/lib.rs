#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod EverythingSDK {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::{path::PathBuf, ptr};
use widestring::*;
use EverythingSDK::*;

struct Everything;

impl Everything {
    fn parse_string_ptr(ptr: *const u16) -> String {
        if ptr.is_null() {
            Everything::get_last_error();
        }
        unsafe { U16CStr::from_ptr_str(ptr).to_string_lossy() }
    }

    pub fn version() -> String {
        unsafe {
            let major = Everything_GetMajorVersion();
            let minor = Everything_GetMinorVersion();
            let revision = Everything_GetRevision();
            format!("{}.{}.{}", major, minor, revision)
        }
    }

    pub fn wait_db_loaded() {
        let sleep_duration = 300;
        let max_wait_time = 60 * 1000 * 2;

        unsafe {
            for _ in 0..(max_wait_time / sleep_duration) {
                if Everything_IsDBLoaded() != 0 {
                    return;
                } else {
                    if Everything::get_last_error() == EVERYTHING_OK {
                        // Everything is still loading the database
                        std::thread::sleep(std::time::Duration::from_millis(sleep_duration));
                    }
                }
            }
            panic!("Timeout waiting for database to load");
        }
    }

    pub fn new() -> Self {
        Self::wait_db_loaded();
        Self
    }

    pub fn reset(self: &Self) {
        unsafe {
            Everything_Reset();
        }
    }

    pub fn get_last_error() -> u32 {
        unsafe {
            match Everything_GetLastError() {
                EVERYTHING_OK => 0,
                EVERYTHING_ERROR_CREATETHREAD => panic!("EVERYTHING_ERROR_CREATETHREAD"),
                EVERYTHING_ERROR_CREATEWINDOW => panic!("EVERYTHING_ERROR_CREATEWINDOW"),
                EVERYTHING_ERROR_INVALIDCALL => panic!("EVERYTHING_ERROR_INVALIDCALL"),
                EVERYTHING_ERROR_INVALIDINDEX => panic!("EVERYTHING_ERROR_INVALIDINDEX"),
                EVERYTHING_ERROR_INVALIDPARAMETER => panic!("EVERYTHING_ERROR_INVALIDPARAMETER"),
                EVERYTHING_ERROR_INVALIDREQUEST => panic!("EVERYTHING_ERROR_INVALIDREQUEST"),
                EVERYTHING_ERROR_IPC => panic!("EVERYTHING_ERROR_IPC (Is Everything running?)"),
                EVERYTHING_ERROR_MEMORY => panic!("EVERYTHING_ERROR_MEMORY"),
                EVERYTHING_ERROR_REGISTERCLASSEX => panic!("EVERYTHING_ERROR_REGISTERCLASSEX"),
                err => panic!("Unknown error: {}", err),
            }
        }
    }

    pub fn set_search(self: &Self, search: &str) {
        let search_c_str = match U16CString::from_str(search) {
            Ok(res) => res,
            Err(_) => panic!("Could not convert search string to c string: {}", search),
        };

        unsafe {
            Everything_SetSearchW(search_c_str.as_ptr());
        }
    }

    pub fn get_search(self: &Self) -> String {
        unsafe { Self::parse_string_ptr(Everything_GetSearchW()) }
    }

    pub fn set_request_flags(self: &Self, flag: u32) {
        unsafe {
            Everything_SetRequestFlags(flag);
        }
    }

    pub fn query(self: &Self) -> DWORD {
        unsafe {
            Self::wait_db_loaded();
            Everything_QueryW(true as BOOL);
            self.get_num_results()
        }
    }

    pub fn get_num_results(self: &Self) -> DWORD {
        unsafe { Everything_GetNumResults() }
    }

    pub fn get_result_full_path(self: &Self, index: DWORD) -> String {
        let index = self.clamp_index(index);
        unsafe {
            let file_path = Everything_GetResultPathW(index);
            let file = Everything_GetResultFileNameW(index);

            PathBuf::from(Self::parse_string_ptr(file_path))
                .join(Self::parse_string_ptr(file))
                .to_str()
                .unwrap()
                .to_string()
        }
    }

    pub fn get_result_file_path(self: &Self, index: DWORD) -> String {
        let index = self.clamp_index(index);
        unsafe { Self::parse_string_ptr(Everything_GetResultPathW(index)) }
    }

    pub fn get_result_file_name(self: &Self, index: u32) -> String {
        let index = self.clamp_index(index as DWORD);
        unsafe { Self::parse_string_ptr(Everything_GetResultFileNameW(index)) }
    }

    pub fn cleanup(self: &Self) {
        unsafe {
            Everything_CleanUp();
        }
    }

    fn clamp_index(self: &Self, index: DWORD) -> DWORD {
        let num_results = self.get_num_results();

        if num_results == 0 {
            return 0;
        }
        if index >= num_results {
            num_results - 1
        } else {
            index
        }
    }
}

impl Drop for Everything {
    fn drop(self: &mut Self) {
        self.cleanup();
    }
}

pub mod everything {}

#[cfg(test)]
mod tests {
    use crate::EverythingSDK;

    use super::Everything;
    use serial_test::serial;

    #[test]
    fn reports_version() {
        let version = Everything::version();
        assert_eq!(version, "1.4.1");
    }

    #[test]
    fn waits_for_db_loaded() {
        Everything::wait_db_loaded();
    }

    #[test]
    #[serial]
    fn search() {
        let everything = Everything::new();
        everything.reset();

        everything.set_search("test");
        assert_eq!(everything.get_search(), "test");

        everything.set_search(".mp4");
        assert_eq!(everything.get_search(), ".mp4");

        everything.set_request_flags(
            EverythingSDK::EVERYTHING_REQUEST_FILE_NAME | EverythingSDK::EVERYTHING_REQUEST_PATH,
        );

        let num_results = everything.query();
        assert!(num_results > 0);
        println!("Num results: {}", num_results);
        assert_eq!(everything.get_num_results(), num_results);

        for i in 0..num_results / 20 {
            let file_name = everything.get_result_file_name(i);
            println!("File name: {}", file_name);
            assert!(file_name.len() > 0);

            let file_path = everything.get_result_file_path(i);
            println!("File path: {}", file_path);
            assert!(file_path.len() > 0);

            let full_path = everything.get_result_full_path(i);
            println!("Full path: {}", full_path);
            assert!(full_path.len() > 0);
        }
    }
}
