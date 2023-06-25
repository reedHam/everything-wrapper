extern crate everything_sys_bindgen;

use everything_sys_bindgen::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

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
