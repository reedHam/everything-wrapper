#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use everything_sys::*;
use widestring::{U16CStr, U16CString};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EverythingError {
    // no error detected
    Ok = EVERYTHING_OK,
    // out of memory.
    Memory = EVERYTHING_ERROR_MEMORY,
    // Everything search client is not running
    Ipc = EVERYTHING_ERROR_IPC,
    // unable to register window class.
    RegisterClassEx = EVERYTHING_ERROR_REGISTERCLASSEX,
    // unable to create listening window
    CreateWindow = EVERYTHING_ERROR_CREATEWINDOW,
    // unable to create listening thread
    CreateThread = EVERYTHING_ERROR_CREATETHREAD,
    // invalid index
    InvalidIndex = EVERYTHING_ERROR_INVALIDINDEX,
    // invalid call
    InvalidCall = EVERYTHING_ERROR_INVALIDCALL,
    // invalid request data, request data first.
    InvalidRequest = EVERYTHING_ERROR_INVALIDREQUEST,
    // bad parameter.
    InvalidParameter = EVERYTHING_ERROR_INVALIDPARAMETER,
}

impl From<EverythingError> for String {
    fn from(error: EverythingError) -> Self {
        format!("{:?}", error)
    }
}

impl From<u32> for EverythingError {
    fn from(error: u32) -> Self {
        match error {
            EVERYTHING_OK => EverythingError::Ok,
            EVERYTHING_ERROR_MEMORY => EverythingError::Memory,
            EVERYTHING_ERROR_IPC => EverythingError::Ipc,
            EVERYTHING_ERROR_REGISTERCLASSEX => EverythingError::RegisterClassEx,
            EVERYTHING_ERROR_CREATEWINDOW => EverythingError::CreateWindow,
            EVERYTHING_ERROR_CREATETHREAD => EverythingError::CreateThread,
            EVERYTHING_ERROR_INVALIDINDEX => EverythingError::InvalidIndex,
            EVERYTHING_ERROR_INVALIDCALL => EverythingError::InvalidCall,
            EVERYTHING_ERROR_INVALIDREQUEST => EverythingError::InvalidRequest,
            EVERYTHING_ERROR_INVALIDPARAMETER => EverythingError::InvalidParameter,
            _ => panic!("Unknown error code: {}", error),
        }
    }
}

pub struct Everything;

impl Everything {
    fn parse_string_ptr(ptr: *const u16) -> String {
        if ptr.is_null() {
            let error_code = Everything::get_last_error();
            panic!("Error code: {:?}", error_code);
        }
        unsafe { U16CStr::from_ptr_str(ptr).to_string_lossy() }
    }

    pub fn get_last_error() -> EverythingError {
        let error_code = unsafe { Everything_GetLastError() };
        error_code.into()
    }

    pub fn wait_db_loaded() {
        let sleep_duration = 300;
        let max_wait_time = 60 * 1000 * 2;
        let mut wait_time = 0;

        unsafe {
            while Everything_IsDBLoaded() == 0 {
                match Everything::get_last_error() {
                    EverythingError::Ok => {
                        std::thread::sleep(std::time::Duration::from_millis(sleep_duration));
                        wait_time += sleep_duration;
                    }
                    _ => panic!(
                        "Error waiting for database to load (code: {:?})",
                        Everything::get_last_error()
                    ),
                }

                if wait_time < max_wait_time {
                    panic!("Timeout waiting for everything to load");
                }
            }
        }
    }

    pub fn set_search(&self, search: &str) {
        let wide_search = U16CString::from_str(search).expect("Failed to convert search string");
        unsafe {
            Everything_SetSearchW(wide_search.as_ptr());
        }
    }

    pub fn query(&self, wait: bool) -> Result<(), EverythingError> {
        let result = unsafe { Everything_QueryW(wait as BOOL) };
        if result == 0 {
            Err(Everything::get_last_error())
        } else {
            Ok(())
        }
    }

    pub fn get_result_count(&self) -> u32 {
        unsafe { Everything_GetNumResults() }
    }

    pub fn get_result_path(&self, index: u32) -> String {
        let result_ptr =
            unsafe { Everything_GetResultFullPathNameW(index, std::ptr::null_mut(), 0) };
        if result_ptr == 0 {
            let error_code = Everything::get_last_error();
            panic!("Error code: {:?}", error_code);
        }

        let mut path_buffer = vec![0u16; result_ptr as usize];
        unsafe {
            Everything_GetResultFullPathNameW(index, path_buffer.as_mut_ptr(), result_ptr);
        }

        Everything::parse_string_ptr(path_buffer.as_ptr())
    }

    pub fn get_result_file_name(&self, index: u32) -> String {
        let result_ptr = unsafe { Everything_GetResultFileNameW(index) };
        Everything::parse_string_ptr(result_ptr)
    }

    pub fn new() -> Everything {
        Everything::wait_db_loaded();
        Everything
    }
}

impl Drop for Everything {
    fn drop(&mut self) {
        unsafe {
            Everything_CleanUp();
        }
    }
}

impl Default for Everything {
    fn default() -> Self {
        Everything::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_everything_instance() {
        let _everything = Everything::new();
    }

    #[test]
    fn test_set_search_and_query() {
        let everything = Everything::new();
        everything.set_search("test");
        let result = everything.query(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_result_count() {
        let everything = Everything::new();
        everything.set_search("test");
        everything.query(true).unwrap();
        let count = everything.get_result_count();
        println!("Result count: {}", count);
    }

    #[test]
    fn test_get_result_paths_and_file_names() {
        let everything = Everything::new();
        everything.set_search("test");
        everything.query(true).unwrap();
        let count = everything.get_result_count();

        for i in 0..count {
            let path = everything.get_result_path(i);
            let file_name = everything.get_result_file_name(i);
            println!("Path: {}, File name: {}", path, file_name);
        }
    }
}
