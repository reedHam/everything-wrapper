#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(test)]
use everything_sys::*;
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr::null_mut;

pub struct Everything {
    query: CString,
}

impl Everything {
    pub fn new(query: &str) -> Result<Self, std::ffi::NulError> {
        let query = CString::new(query)?;
        Ok(Self { query })
    }

    pub fn search(&self) -> Result<Vec<String>, &'static str> {
        let query_ptr = self.query.as_ptr() as *mut i8;

        unsafe {
            Everything_SetSearch(query_ptr);
            Everything_SetRequestFlags(EVERYTHING_REQUEST_FULL_PATH_AND_FILE_NAME);
            Everything_SetMatchPath(1);
            Everything_SetSort(1);
        }

        if unsafe { Everything_Query(true as i32) } == 0 {
            let num_results = unsafe { Everything_GetNumResults() };
            let mut results: Vec<String> = Vec::with_capacity(num_results as usize);

            for i in 0..num_results {
                let result_size =
                    unsafe { Everything_GetResultFullPathNameW(i, null_mut(), 0) } + 1;
                let mut result_buffer: Vec<u16> = Vec::with_capacity(result_size as usize);
                result_buffer.resize(result_size as usize, 0);

                unsafe {
                    Everything_GetResultFullPathNameW(i, result_buffer.as_mut_ptr(), result_size);
                }

                if let Ok(result_string) = String::from_utf16(&result_buffer) {
                    results.push(result_string);
                }
            }

            Ok(results)
        } else {
            Err("Error executing the search query.")
        }
    }
}

impl Drop for Everything {
    fn drop(&mut self) {
        unsafe {
            Everything_Reset();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let everything = Everything::new("*.txt").unwrap();
        let results = everything.search().unwrap();
        assert!(!results.is_empty());
    }
}
