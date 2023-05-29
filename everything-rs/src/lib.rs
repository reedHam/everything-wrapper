//! # Everything  
//! This crate provides a safe wrapper around the `everything-sys-bindgen`.  
//! `everything-sys-bindgen` is a rust binding to the [Everything SDK](https://www.voidtools.com/support/everything/sdk/) that allow IPC communication to the everything service.  
//! The Everything service indexes files on windows and provides a expressive query syntax to search for files.  
//! See the [Everything SDK documentation](https://www.voidtools.com/support/everything/sdk/) for more information.  
//!
//! # Example
//! ```rust
//! use everything_rs::{Everything, EverythingRequestFlags, EverythingSort, EverythingError};
//!
//! fn main() -> Result<(), EverythingError> {
//!    let mut everything = Everything::new();
//!
//!    everything.set_search("test");
//!
//!    everything.set_request_flags(
//!        EverythingRequestFlags::FullPathAndFileName
//!        | EverythingRequestFlags::Size
//!        | EverythingRequestFlags::DateCreated
//!    );
//!
//!    everything.set_sort(EverythingSort::DateCreatedDescending);
//!
//!    everything.query()?;
//!
//!    let num_results = everything.get_num_results();
//!
//!    assert!(num_results > 0);
//!
//!    for (i, path) in everything.full_path_iter().flatten().enumerate() {
//!        let size = everything.get_result_size(i as u32)?;
//!        let date_created = everything.get_result_created_date(i as u32)?;
//!        println!("{}: {} {} {}", i, path, size, date_created);
//!    }
//!
//!    Ok(())
//! }
//! ```

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(target_os = "windows")]
extern crate everything_sys_bindgen;

use bitflags::bitflags;
use everything_sys_bindgen::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use widestring::{U16CStr, U16CString};

/// Represents errors that can occur when using the Everything SDK and that will be returned by Everything.get_last_error().   
/// See <https://www.voidtools.com/support/everything/sdk/everything_getlasterror/>   
/// Note that there is no documentation for EVERYTHING_ERROR_INVALIDREQUEST or EVERYTHING_ERROR_INVALIDPARAMETER   
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, TS)]
pub enum EverythingError {
    /// no error detected
    Ok = EVERYTHING_OK,
    /// out of memory.
    Memory = EVERYTHING_ERROR_MEMORY,
    /// Everything search client is not running
    Ipc = EVERYTHING_ERROR_IPC,
    /// unable to register window class.
    RegisterClassEx = EVERYTHING_ERROR_REGISTERCLASSEX,
    /// unable to create listening window
    CreateWindow = EVERYTHING_ERROR_CREATEWINDOW,
    /// unable to create listening thread
    CreateThread = EVERYTHING_ERROR_CREATETHREAD,
    /// invalid index
    InvalidIndex = EVERYTHING_ERROR_INVALIDINDEX,
    /// invalid call
    InvalidCall = EVERYTHING_ERROR_INVALIDCALL,
    /// Occurs when attempting to read a result attribute without setting the request flags for it.  
    /// You must set request flags before trying to read a result.  
    InvalidRequest = EVERYTHING_ERROR_INVALIDREQUEST,
    /// bad parameter.
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

/// Types of sorting that everything can supports.  
/// See <https://www.voidtools.com/support/everything/sdk/everything_setsort/>  
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
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

bitflags! {
    /// Input to the Everything.set_request_flags() function.
    /// Defines what file information is loaded into memory by everything when query is called.
    /// See <https://www.voidtools.com/support/everything/sdk/everything_getrequestflags/> for more information.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EverythingRequestFlags: u32 {
        const FileName = EVERYTHING_REQUEST_FILE_NAME;
        const Path = EVERYTHING_REQUEST_PATH;
        const FullPathAndFileName = EVERYTHING_REQUEST_FULL_PATH_AND_FILE_NAME;
        const Extension = EVERYTHING_REQUEST_EXTENSION;
        const Size = EVERYTHING_REQUEST_SIZE;
        const DateCreated = EVERYTHING_REQUEST_DATE_CREATED;
        const DateModified = EVERYTHING_REQUEST_DATE_MODIFIED;
        const DateAccessed = EVERYTHING_REQUEST_DATE_ACCESSED;
        const Attributes = EVERYTHING_REQUEST_ATTRIBUTES;
        const FileListFileName = EVERYTHING_REQUEST_FILE_LIST_FILE_NAME;
        const RunCount = EVERYTHING_REQUEST_RUN_COUNT;
        const DateRun = EVERYTHING_REQUEST_DATE_RUN;
        const DateRecentlyChanged = EVERYTHING_REQUEST_DATE_RECENTLY_CHANGED;
        const HighlightedFileName = EVERYTHING_REQUEST_HIGHLIGHTED_FILE_NAME;
        const HighlightedPath = EVERYTHING_REQUEST_HIGHLIGHTED_PATH;
        const HighlightedFullPathAndFileName = EVERYTHING_REQUEST_HIGHLIGHTED_FULL_PATH_AND_FILE_NAME;
    }
}

trait U64Able {
    fn as_u64(&self) -> u64;
}

impl U64Able for FILETIME {
    fn as_u64(&self) -> u64 {
        ((self.dwHighDateTime as u64) << 32) | (self.dwLowDateTime as u64)
    }
}

/// Checks for a null pointer and gets the last everything error if there is one.   
/// Otherwise, iterate until null is reached and return a string.  
/// # Arguments  
///
/// * `ptr` - A pointer to a u16 string returned by the Everything API.  
fn parse_string_ptr(ptr: *const u16) -> Result<String, EverythingError> {
    if ptr.is_null() {
        let error_code = Everything::get_last_error();
        panic!("Error code: {:?}", error_code);
    }

    Ok(unsafe { U16CStr::from_ptr_str(ptr).to_string_lossy() })
}

/// A wrapper around the Everything API.  
/// Calls cleanup on drop.  
#[derive(Debug)]
pub struct Everything;

impl Everything {
    /// See <https://www.voidtools.com/support/everything/sdk/everything_getlasterror/>  
    pub fn get_last_error() -> Result<(), EverythingError> {
        let error_code = unsafe { Everything_GetLastError() };
        match error_code.try_into().unwrap() {
            EverythingError::Ok => Ok(()),
            err => Err(err),
        }
    }

    /// Sleep the current thread until the Everything database is loaded.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_isdbloaded/>  
    pub fn wait_db_loaded() -> Result<(), EverythingError> {
        let sleep_duration = 300;
        let max_wait_time = 60 * 1000 * 2;
        let mut wait_time = 0;

        unsafe {
            while Everything_IsDBLoaded() == 0 {
                Everything::get_last_error()?;
                if wait_time < max_wait_time {
                    panic!("Timeout waiting for everything to load");
                }
                std::thread::sleep(std::time::Duration::from_millis(sleep_duration));
                wait_time += sleep_duration;
            }
        }

        Ok(())
    }

    /// Set the query to be used by the next call to query.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_setsearch/>  
    pub fn set_search(&self, search: &str) {
        let wide_search = U16CString::from_str(search).expect("Failed to convert search string");
        unsafe {
            Everything_SetSearchW(wide_search.as_ptr());
        }
    }

    /// Get the current query.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_getsearch/>  
    ///
    /// Search Syntax reference can be seen here   
    /// <https://www.voidtools.com/support/everything/searching/>   
    /// More search functions can be found on the forums.   
    /// <https://www.voidtools.com/forum/viewtopic.php?t=10091>  
    /// <https://www.voidtools.com/forum/viewtopic.php?t=10176>  
    /// <https://www.voidtools.com/forum/viewtopic.php?t=10099>  
    /// <https://www.voidtools.com/forum/viewtopic.php?t=10860>  
    pub fn get_search(&self) -> Result<String, EverythingError> {
        let search_ptr = unsafe { Everything_GetSearchW() };
        parse_string_ptr(search_ptr)
    }

    /// Set the sorting to be used by the next call to query.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_setsort/>  
    pub fn set_sort(&self, sort: EverythingSort) {
        unsafe {
            Everything_SetSort(sort.into());
        }
    }

    /// Get the current sorting.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_getsort/>  
    pub fn get_sort(&self) -> Option<EverythingSort> {
        let sort = unsafe { Everything_GetSort() };
        if let Ok(eve_sort) = sort.try_into() {
            Some(eve_sort)
        } else {
            None
        }
    }

    /// Check if the sort type is indexed.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_isfastsort/>  
    pub fn is_fast_sort(&self, sort: EverythingSort) -> bool {
        unsafe { Everything_IsFastSort(sort.into()) != 0 }
    }

    /// See <https://www.voidtools.com/support/everything/sdk/everything_isfileresult/>  
    pub fn is_result_file(&self, index: DWORD) -> bool {
        unsafe { Everything_IsFileResult(index) != 0 }
    }

    /// See <https://www.voidtools.com/support/everything/sdk/everything_isfolderresult/>  
    pub fn is_result_folder(&self, index: DWORD) -> bool {
        unsafe { Everything_IsFolderResult(index) != 0 }
    }

    /// See <https://www.voidtools.com/support/everything/sdk/everything_isvolumeresult/>  
    pub fn is_result_volume(&self, index: DWORD) -> bool {
        unsafe { Everything_IsVolumeResult(index) != 0 }
    }

    /// Returns the number of visible results in everything's memory.  
    /// Will be 0 until the first call to query.  
    /// Then will equal the number of max results specified by set_max_results.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_getnumresults/>  
    pub fn get_num_results(&self) -> DWORD {
        unsafe { Everything_GetNumResults() }
    }

    /// Returns the number of results returned by the query.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_gettotresults/>  
    pub fn get_total_results(&self) -> DWORD {
        unsafe { Everything_GetTotResults() }
    }

    /// Limit's the number of results returned by the everything.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_setmax/>  
    pub fn set_max_results(&self, max_results: DWORD) {
        unsafe {
            Everything_SetMax(max_results);
        }
    }

    /// Returns the current maximum number of results.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_getmax/>  
    pub fn get_max_results(&self) -> DWORD {
        unsafe { Everything_GetMax() }
    }

    /// Set the index offset that everything will start its result window from.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_setoffset/>  
    pub fn set_result_offset(&self, offset_results: DWORD) {
        unsafe {
            Everything_SetOffset(offset_results);
        }
    }

    /// Returns the current result offset.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_getoffset/>  
    pub fn get_result_offset(&self) -> DWORD {
        unsafe { Everything_GetOffset() }
    }

    /// Set the type of data that the everything service will load.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_setreplywindow/>  
    pub fn set_request_flags(&self, request_flags: EverythingRequestFlags) {
        unsafe {
            Everything_SetRequestFlags(request_flags.bits());
        }
    }

    /// Returns the current request flags.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_getrequestflags/>  
    pub fn get_request_flags(&self) -> EverythingRequestFlags {
        let request_flags = unsafe { Everything_GetRequestFlags() };
        EverythingRequestFlags::from_bits_truncate(request_flags)
    }

    pub fn query(&self) -> Result<(), EverythingError> {
        let result = unsafe { Everything_QueryW(1) };
        if result == 0 {
            Everything::get_last_error()
        } else {
            Ok(())
        }
    }

    /// Reset the query state.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_reset/>  
    pub fn reset(&self) {
        unsafe {
            Everything_Reset();
        }
    }

    /// Returns the total number of indexes in the everything result window.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_getnumfileresults/>  
    pub fn get_result_count(&self) -> u32 {
        unsafe { Everything_GetNumResults() }
    }

    /// Creates a owned string from full path slice for a result at an index.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_getresultfullpathname/>  
    pub fn get_result_full_path(&self, index: u32) -> Result<String, EverythingError> {
        let path_length =
            unsafe { Everything_GetResultFullPathNameW(index, std::ptr::null_mut(), 0) };
        if path_length == 0 {
            Everything::get_last_error()?;
        }

        // Length does not include null terminator
        let mut path_buffer = Vec::with_capacity(path_length as usize + 1);
        unsafe {
            let count_copied = Everything_GetResultFullPathNameW(
                index,
                path_buffer.as_mut_ptr(),
                path_buffer.len() as u32,
            );
            Ok(
                U16CStr::from_ptr(path_buffer.as_ptr(), count_copied as usize)
                    .unwrap()
                    .to_string_lossy(),
            )
        }
    }

    /// Returns an iterator over the full paths of the results.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_getresultfullpathname/>  
    pub fn full_path_iter(&self) -> impl Iterator<Item = Result<String, EverythingError>> + '_ {
        let num_results = self.get_result_count();
        (0..num_results).map(|index| self.get_result_full_path(index))
    }

    /// Iterates from the pointer to find a null terminator returning an owned string.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_getresultfilename/>  
    pub fn get_result_file_name(&self, index: u32) -> Result<String, EverythingError> {
        let result_ptr = unsafe { Everything_GetResultFileNameW(index) };

        if result_ptr.is_null() {
            Everything::get_last_error()?;
        }

        parse_string_ptr(result_ptr)
    }

    /// Returns an iterator over the file names of the results.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_getresultfilename/>  
    pub fn name_iter(&self) -> impl Iterator<Item = Result<String, EverythingError>> + '_ {
        let num_results = self.get_result_count();
        (0..num_results).map(|index| self.get_result_file_name(index))
    }

    /// Returns the created date of the result at the index.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_getresultdatecreated/>  
    pub fn get_result_created_date(&self, index: u32) -> Result<u64, EverythingError> {
        let mut file_time: FILETIME = FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        };

        let success = unsafe { Everything_GetResultDateCreated(index, &mut file_time) };

        if success == 0 {
            Everything::get_last_error()?;
        }

        Ok(file_time.as_u64())
    }

    /// Returns the modified date of the result at the index.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_getresultdatemodified/>  
    pub fn get_result_count_modified_date(&self, index: u32) -> Result<u64, EverythingError> {
        let mut file_time: FILETIME = FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        };

        let success = unsafe { Everything_GetResultDateModified(index, &mut file_time) };

        if success == 0 {
            Everything::get_last_error()?;
        }

        Ok(file_time.as_u64())
    }

    /// Returns the last accessed date of the result at the index.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_getresultdateaccessed/>  
    pub fn get_result_size(&self, index: u32) -> Result<u64, EverythingError> {
        let mut size: LARGE_INTEGER = LARGE_INTEGER { QuadPart: 0 };

        let success = unsafe { Everything_GetResultSize(index, &mut size) };

        if success == 0 {
            Everything::get_last_error()?;
        }

        Ok(unsafe { size.QuadPart as u64 })
    }

    /// Returns the extension of the result at the index.  
    /// iterates from a string pointer to find a null terminator returning an owned string.  
    /// See <https://www.voidtools.com/support/everything/sdk/everything_getresultextension/>  
    pub fn get_result_extension(&self, index: u32) -> Result<String, EverythingError> {
        let result_ptr = unsafe { Everything_GetResultExtensionW(index) };

        if result_ptr.is_null() {
            Everything::get_last_error()?;
        }

        parse_string_ptr(result_ptr)
    }

    /// Waits for the Everything database to be fully loaded before returning an instance.
    pub fn new() -> Everything {
        Everything::wait_db_loaded();
        Everything
    }

    pub fn version() -> String {
        unsafe {
            let major = Everything_GetMajorVersion();
            let minor = Everything_GetMinorVersion();
            let revision = Everything_GetRevision();
            let build = Everything_GetBuildNumber();

            format!("{}.{}.{}.{}", major, minor, revision, build)
        }
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
    use lazy_static::lazy_static;
    use std::path::Path;

    lazy_static! {}

    #[test]
    fn parses_string_ptr() {
        let test_string = "test\0";
        let test_string_ptr = test_string.encode_utf16().collect::<Vec<u16>>();
        let test_string_ptr = test_string_ptr.as_ptr();
        let parsed_string = parse_string_ptr(test_string_ptr).unwrap();
        assert_eq!(parsed_string, test_string[0..4]);
    }

    #[test]
    fn parses_full_path() {
        let test_dir_path = Path::canonicalize(Path::new("../test")).unwrap();
        let test_dir_path = test_dir_path.to_str().unwrap();
        let test_dir_path = test_dir_path.trim_start_matches(r"\\?\");

        println!("{}", test_dir_path);

        let evthing = Everything::new();

        evthing.set_search(test_dir_path);
        evthing.set_request_flags(EverythingRequestFlags::FullPathAndFileName);

        evthing.query().unwrap();

        let num_results = evthing.get_result_count();

        assert!(num_results > 0);

        for path in evthing.full_path_iter().flatten() {
            println!("{}", path);
            assert!(path.contains(test_dir_path));
        }
    }

    #[test]
    fn searches() {
        let everything: Everything = Everything::new();

        everything.set_search("test");
        let search = everything.get_search().unwrap();
        assert_eq!(search, "test");

        everything.set_max_results(10);
        let max_results = everything.get_max_results();
        assert_eq!(max_results, 10);

        everything.set_result_offset(10);
        let offset = everything.get_result_offset();
        assert_eq!(offset, 10);

        let flag = EverythingRequestFlags::FullPathAndFileName
            | EverythingRequestFlags::DateCreated
            | EverythingRequestFlags::DateModified
            | EverythingRequestFlags::Size
            | EverythingRequestFlags::Extension;
        everything.set_request_flags(flag);

        let flags = everything.get_request_flags();
        assert_eq!(flags, flag);

        everything.set_sort(EverythingSort::DateCreatedDescending);

        everything.query().unwrap();

        let num_results = everything.get_result_count();
        assert_eq!(num_results, 10);

        let full_path_results: Vec<Result<String, EverythingError>> =
            everything.full_path_iter().collect();

        assert_eq!(full_path_results.len(), 10);

        let mut last_date_created = everything.get_result_created_date(0).unwrap();
        for idx in 0..num_results {
            let result = everything.get_result_full_path(idx).unwrap();
            let iter_result = full_path_results[idx as usize].as_ref().unwrap();
            assert_eq!(result, *iter_result);

            let size = everything.get_result_size(idx).unwrap();
            assert!(size > 0);

            let created_date = everything.get_result_created_date(idx).unwrap();
            assert!(created_date > 0);
            assert!(created_date <= last_date_created);
            last_date_created = created_date;

            let modified_date = everything.get_result_count_modified_date(idx).unwrap();
            assert!(modified_date > 0);
        }
    }
}
