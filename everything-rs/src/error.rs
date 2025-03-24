#[cfg(target_os = "windows")]
extern crate everything_sys_bindgen;

use everything_sys_bindgen::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use ts_rs::TS;

/// Represents errors that can occur when using the Everything SDK and that will be returned by Everything.get_last_error().   
/// See <https://www.voidtools.com/support/everything/sdk/everything_getlasterror/>   
/// Note that there is no documentation for EVERYTHING_ERROR_INVALIDREQUEST or EVERYTHING_ERROR_INVALIDPARAMETER   
#[repr(u32)]
#[derive(
    thiserror::Error,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    TS,
)]
pub enum EverythingSDKError {
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

impl From<EverythingSDKError> for String {
    fn from(error: EverythingSDKError) -> Self {
        format!("{:?}", error)
    }
}

impl TryFrom<u32> for EverythingSDKError {
    type Error = &'static str;

    fn try_from(error: u32) -> Result<Self, Self::Error> {
        let err = match error {
            EVERYTHING_OK => EverythingSDKError::Ok,
            EVERYTHING_ERROR_MEMORY => EverythingSDKError::Memory,
            EVERYTHING_ERROR_IPC => EverythingSDKError::Ipc,
            EVERYTHING_ERROR_REGISTERCLASSEX => EverythingSDKError::RegisterClassEx,
            EVERYTHING_ERROR_CREATEWINDOW => EverythingSDKError::CreateWindow,
            EVERYTHING_ERROR_CREATETHREAD => EverythingSDKError::CreateThread,
            EVERYTHING_ERROR_INVALIDINDEX => EverythingSDKError::InvalidIndex,
            EVERYTHING_ERROR_INVALIDCALL => EverythingSDKError::InvalidCall,
            EVERYTHING_ERROR_INVALIDREQUEST => EverythingSDKError::InvalidRequest,
            EVERYTHING_ERROR_INVALIDPARAMETER => EverythingSDKError::InvalidParameter,
            _ => Err("Unknown error code")?,
        };
        Ok(err)
    }
}

impl Display for EverythingSDKError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EverythingError {
    #[error("Everything SDK error: {0}")]
    SDKError(#[from] EverythingSDKError),
    #[error("Timeout waiting for Everything to respond")]
    DatabaseTimeout,
    #[error("Null pointer returned by Everything SDK")]
    NullPointerError,
}

impl EverythingError {
    /// Returns a user-friendly error message with hints on how to resolve the issue
    pub fn to_user_friendly_message(&self) -> String {
        match self {
            EverythingError::SDKError(EverythingSDKError::Ipc) => {
                "Connection to Everything service failed. Is Everything running? \
                Please make sure the Everything application is running and try again."
                    .to_string()
            }
            EverythingError::SDKError(EverythingSDKError::Memory) => {
                "Out of memory error. The system might be low on resources.".to_string()
            }
            EverythingError::DatabaseTimeout => {
                "Timeout waiting for Everything database. The database might be still loading or \
                the Everything service might be busy. Please try again later."
                    .to_string()
            }
            _ => self.to_string(),
        }
    }
}

pub type EverythingResult<T> = Result<T, EverythingError>;
