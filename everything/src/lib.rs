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

impl TryFrom<u32> for EverythingError {
    type Error = &'static str;

    fn try_from(error: u32) -> Result<Self, Self::Error> {
        let err = match error {
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
            _ => Err("Unknown error code")?,
        };
        Ok(err)
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EverythingSort {
    NameAscending = EVERYTHING_SORT_NAME_ASCENDING,
    NameDescending = EVERYTHING_SORT_NAME_DESCENDING,
    PathAscending = EVERYTHING_SORT_PATH_ASCENDING,
    PathDescending = EVERYTHING_SORT_PATH_DESCENDING,
    SizeAscending = EVERYTHING_SORT_SIZE_ASCENDING,
    SizeDescending = EVERYTHING_SORT_SIZE_DESCENDING,
    ExtensionAscending = EVERYTHING_SORT_EXTENSION_ASCENDING,
    ExtensionDescending = EVERYTHING_SORT_EXTENSION_DESCENDING,
    TypeNameAscending = EVERYTHING_SORT_TYPE_NAME_ASCENDING,
    TypeNameDescending = EVERYTHING_SORT_TYPE_NAME_DESCENDING,
    DateCreatedAscending = EVERYTHING_SORT_DATE_CREATED_ASCENDING,
    DateCreatedDescending = EVERYTHING_SORT_DATE_CREATED_DESCENDING,
    DateModifiedAscending = EVERYTHING_SORT_DATE_MODIFIED_ASCENDING,
    DateModifiedDescending = EVERYTHING_SORT_DATE_MODIFIED_DESCENDING,
    AttributesAscending = EVERYTHING_SORT_ATTRIBUTES_ASCENDING,
    AttributesDescending = EVERYTHING_SORT_ATTRIBUTES_DESCENDING,
    FileListFilenameAscending = EVERYTHING_SORT_FILE_LIST_FILENAME_ASCENDING,
    FileListFilenameDescending = EVERYTHING_SORT_FILE_LIST_FILENAME_DESCENDING,
    RunCountAscending = EVERYTHING_SORT_RUN_COUNT_ASCENDING,
    RunCountDescending = EVERYTHING_SORT_RUN_COUNT_DESCENDING,
    DateRecentlyChangedAscending = EVERYTHING_SORT_DATE_RECENTLY_CHANGED_ASCENDING,
    DateRecentlyChangedDescending = EVERYTHING_SORT_DATE_RECENTLY_CHANGED_DESCENDING,
    DateAccessedAscending = EVERYTHING_SORT_DATE_ACCESSED_ASCENDING,
    DateAccessedDescending = EVERYTHING_SORT_DATE_ACCESSED_DESCENDING,
    DateRunAscending = EVERYTHING_SORT_DATE_RUN_ASCENDING,
    DateRunDescending = EVERYTHING_SORT_DATE_RUN_DESCENDING,
}

impl From<EverythingSort> for u32 {
    fn from(sort: EverythingSort) -> Self {
        match sort {
            EverythingSort::NameAscending => EVERYTHING_SORT_NAME_ASCENDING,
            EverythingSort::NameDescending => EVERYTHING_SORT_NAME_DESCENDING,
            EverythingSort::PathAscending => EVERYTHING_SORT_PATH_ASCENDING,
            EverythingSort::PathDescending => EVERYTHING_SORT_PATH_DESCENDING,
            EverythingSort::SizeAscending => EVERYTHING_SORT_SIZE_ASCENDING,
            EverythingSort::SizeDescending => EVERYTHING_SORT_SIZE_DESCENDING,
            EverythingSort::ExtensionAscending => EVERYTHING_SORT_EXTENSION_ASCENDING,
            EverythingSort::ExtensionDescending => EVERYTHING_SORT_EXTENSION_DESCENDING,
            EverythingSort::TypeNameAscending => EVERYTHING_SORT_TYPE_NAME_ASCENDING,
            EverythingSort::TypeNameDescending => EVERYTHING_SORT_TYPE_NAME_DESCENDING,
            EverythingSort::DateCreatedAscending => EVERYTHING_SORT_DATE_CREATED_ASCENDING,
            EverythingSort::DateCreatedDescending => EVERYTHING_SORT_DATE_CREATED_DESCENDING,
            EverythingSort::DateModifiedAscending => EVERYTHING_SORT_DATE_MODIFIED_ASCENDING,
            EverythingSort::DateModifiedDescending => EVERYTHING_SORT_DATE_MODIFIED_DESCENDING,
            EverythingSort::AttributesAscending => EVERYTHING_SORT_ATTRIBUTES_ASCENDING,
            EverythingSort::AttributesDescending => EVERYTHING_SORT_ATTRIBUTES_DESCENDING,
            EverythingSort::FileListFilenameAscending => {
                EVERYTHING_SORT_FILE_LIST_FILENAME_ASCENDING
            }
            EverythingSort::FileListFilenameDescending => {
                EVERYTHING_SORT_FILE_LIST_FILENAME_DESCENDING
            }
            EverythingSort::RunCountAscending => EVERYTHING_SORT_RUN_COUNT_ASCENDING,
            EverythingSort::RunCountDescending => EVERYTHING_SORT_RUN_COUNT_DESCENDING,
            EverythingSort::DateRecentlyChangedAscending => {
                EVERYTHING_SORT_DATE_RECENTLY_CHANGED_ASCENDING
            }
            EverythingSort::DateRecentlyChangedDescending => {
                EVERYTHING_SORT_DATE_RECENTLY_CHANGED_DESCENDING
            }
            EverythingSort::DateAccessedAscending => EVERYTHING_SORT_DATE_ACCESSED_ASCENDING,
            EverythingSort::DateAccessedDescending => EVERYTHING_SORT_DATE_ACCESSED_DESCENDING,
            EverythingSort::DateRunAscending => EVERYTHING_SORT_DATE_RUN_ASCENDING,
            EverythingSort::DateRunDescending => EVERYTHING_SORT_DATE_RUN_DESCENDING,
        }
    }
}

impl TryFrom<u32> for EverythingSort {
    type Error = &'static str;

    fn try_from(sort: u32) -> Result<Self, Self::Error> {
        let sort = match sort {
            EVERYTHING_SORT_NAME_ASCENDING => EverythingSort::NameAscending,
            EVERYTHING_SORT_NAME_DESCENDING => EverythingSort::NameDescending,
            EVERYTHING_SORT_PATH_ASCENDING => EverythingSort::PathAscending,
            EVERYTHING_SORT_PATH_DESCENDING => EverythingSort::PathDescending,
            EVERYTHING_SORT_SIZE_ASCENDING => EverythingSort::SizeAscending,
            EVERYTHING_SORT_SIZE_DESCENDING => EverythingSort::SizeDescending,
            EVERYTHING_SORT_EXTENSION_ASCENDING => EverythingSort::ExtensionAscending,
            EVERYTHING_SORT_EXTENSION_DESCENDING => EverythingSort::ExtensionDescending,
            EVERYTHING_SORT_TYPE_NAME_ASCENDING => EverythingSort::TypeNameAscending,
            EVERYTHING_SORT_TYPE_NAME_DESCENDING => EverythingSort::TypeNameDescending,
            EVERYTHING_SORT_DATE_CREATED_ASCENDING => EverythingSort::DateCreatedAscending,
            EVERYTHING_SORT_DATE_CREATED_DESCENDING => EverythingSort::DateCreatedDescending,
            EVERYTHING_SORT_DATE_MODIFIED_ASCENDING => EverythingSort::DateModifiedAscending,
            EVERYTHING_SORT_DATE_MODIFIED_DESCENDING => EverythingSort::DateModifiedDescending,
            EVERYTHING_SORT_ATTRIBUTES_ASCENDING => EverythingSort::AttributesAscending,
            EVERYTHING_SORT_ATTRIBUTES_DESCENDING => EverythingSort::AttributesDescending,
            EVERYTHING_SORT_FILE_LIST_FILENAME_ASCENDING => {
                EverythingSort::FileListFilenameAscending
            }
            EVERYTHING_SORT_FILE_LIST_FILENAME_DESCENDING => {
                EverythingSort::FileListFilenameDescending
            }
            EVERYTHING_SORT_RUN_COUNT_ASCENDING => EverythingSort::RunCountAscending,
            EVERYTHING_SORT_RUN_COUNT_DESCENDING => EverythingSort::RunCountDescending,
            EVERYTHING_SORT_DATE_RECENTLY_CHANGED_ASCENDING => {
                EverythingSort::DateRecentlyChangedAscending
            }
            EVERYTHING_SORT_DATE_RECENTLY_CHANGED_DESCENDING => {
                EverythingSort::DateRecentlyChangedDescending
            }
            EVERYTHING_SORT_DATE_ACCESSED_ASCENDING => EverythingSort::DateAccessedAscending,
            EVERYTHING_SORT_DATE_ACCESSED_DESCENDING => EverythingSort::DateAccessedDescending,
            EVERYTHING_SORT_DATE_RUN_ASCENDING => EverythingSort::DateRunAscending,
            EVERYTHING_SORT_DATE_RUN_DESCENDING => EverythingSort::DateRunDescending,
            _ => Err("Unknown sort code")?,
        };
        Ok(sort)
    }
}

pub struct Everything;

impl Everything {
    fn parse_string_ptr(ptr: *const u16) -> String {
        if ptr.is_null() {
            let error_code = Everything::get_last_error();
            panic!("Error code: {:?}", error_code);
        }
        unsafe {
            if let Ok(string) = U16CStr::from_ptr_str(ptr).to_string() {
                string
            } else {
                let lossy_string = U16CStr::from_ptr_str(ptr).to_string_lossy();
                println!("Failed to convert path: {:?}", lossy_string);
                lossy_string
            }
        }
    }

    pub fn get_last_error() -> EverythingError {
        let error_code = unsafe { Everything_GetLastError() };
        error_code.try_into().unwrap()
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

    pub fn get_search(&self) -> String {
        let search_ptr = unsafe { Everything_GetSearchW() };
        Everything::parse_string_ptr(search_ptr)
    }

    pub fn set_sort(&self, sort: EverythingSort) {
        unsafe {
            Everything_SetSort(sort.into());
        }
    }

    pub fn get_sort(&self) -> Option<EverythingSort> {
        let sort = unsafe { Everything_GetSort() };
        if let Ok(eve_sort) = sort.try_into() {
            Some(eve_sort)
        } else {
            None
        }
    }

    pub fn is_fast_sort(&self, sort: EverythingSort) -> bool {
        unsafe { Everything_IsFastSort(sort.into()) != 0 }
    }

    pub fn is_result_file(&self, index: DWORD) -> bool {
        unsafe { Everything_IsFileResult(index) != 0 }
    }

    pub fn is_result_folder(&self, index: DWORD) -> bool {
        unsafe { Everything_IsFolderResult(index) != 0 }
    }

    pub fn is_result_volume(&self, index: DWORD) -> bool {
        unsafe { Everything_IsVolumeResult(index) != 0 }
    }

    pub fn get_num_results(&self) -> DWORD {
        unsafe { Everything_GetNumResults() }
    }

    pub fn get_total_results(&self) -> DWORD {
        unsafe { Everything_GetTotResults() }
    }

    pub fn set_max_results(&self, max_results: DWORD) {
        unsafe {
            Everything_SetMax(max_results);
        }
    }

    pub fn get_max_results(&self) -> DWORD {
        unsafe { Everything_GetMax() }
    }

    pub fn set_result_offset(&self, offset_results: DWORD) {
        unsafe {
            Everything_SetOffset(offset_results);
        }
    }

    pub fn get_result_offset(&self) -> DWORD {
        unsafe { Everything_GetOffset() }
    }

    pub fn query(&self, wait: bool) -> Result<(), EverythingError> {
        let result = unsafe { Everything_QueryW(wait as BOOL) };
        if result == 0 {
            Err(Everything::get_last_error())
        } else {
            Ok(())
        }
    }

    pub fn reset(&self) {
        unsafe {
            Everything_Reset();
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

    pub fn path_iter(&self) -> impl Iterator<Item = String> + '_ {
        let num_results = self.get_result_count();
        let offset = self.get_result_offset();

        (offset..num_results).map(|index| self.get_result_path(index))
    }

    pub fn get_result_file_name(&self, index: u32) -> String {
        let result_ptr = unsafe { Everything_GetResultFileNameW(index) };
        Everything::parse_string_ptr(result_ptr)
    }

    pub fn name_iter(&self) -> impl Iterator<Item = String> + '_ {
        let num_results = self.get_result_count();
        let offset = self.get_result_offset();

        (offset..num_results).map(|index| self.get_result_file_name(index))
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
    fn it_functions() {
        let everything = Everything::new();

        everything.set_search("test");
        let search = everything.get_search();
        assert_eq!(search, "test");

        everything.set_sort(EverythingSort::DateCreatedDescending);
        let sort = everything.get_sort().unwrap();
        assert_eq!(sort, EverythingSort::DateCreatedDescending);

        let is_fast_sort = everything.is_fast_sort(sort);
        assert!(is_fast_sort);

        let is_not_fast_sort = everything.is_fast_sort(EverythingSort::TypeNameAscending);
        assert!(!is_not_fast_sort);

        everything.query(true).unwrap();
        let result_count = everything.get_result_count();
        assert!(result_count > 0);

        let mut results: Vec<String> = vec![];
        for i in 0..10 {
            let path = everything.get_result_path(i);
            let file_name = everything.get_result_file_name(i);
            results.push(path.clone());
            println!("Path: {}, File name: {}", path, file_name);
        }

        let path_results = everything.path_iter().take(10);

        for (path, path_result) in results.iter().zip(path_results) {
            assert_eq!(path, &path_result);
        }

        everything.reset();

        let result_count = everything.get_result_count();
        assert_eq!(result_count, 0);

        everything.set_search("test");
        everything.set_result_offset(100);
        everything.query(true).unwrap();

        let result_count = everything.get_result_count();
        assert!(result_count > 0);

        let offset = everything.get_result_offset();

        let mut results: Vec<String> = vec![];
        for i in offset..offset + 10 {
            let path = everything.get_result_path(i);
            let file_name = everything.get_result_file_name(i);
            results.push(path.clone());
            println!("Path: {}, File name: {}", path, file_name);
        }

        let path_results = everything.path_iter().take(10);

        for (path, path_result) in results.iter().zip(path_results) {
            assert_eq!(path, &path_result);
        }

        everything.set_result_offset(100_000_000);
        everything.query(true).unwrap();

        let result_count = everything.get_result_count();
        assert_eq!(result_count, 0);

        let results = everything.path_iter().take(10);
        assert_eq!(results.count(), 0);
    }
}
