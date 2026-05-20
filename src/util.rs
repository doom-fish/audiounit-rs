use core::ffi::c_char;
use std::ffi::CString;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::AuError;
use crate::ffi;

pub fn take_string(ptr: *mut c_char) -> Option<String> {
    unsafe { doom_fish_utils::ffi_string::take_owned_cstring_c(ptr, |p| ffi::au_string_free(p)) }
}

pub fn take_json<T: DeserializeOwned>(ptr: *mut c_char) -> Result<T, AuError> {
    let json = take_string(ptr).ok_or_else(|| {
        AuError::Serialization("Swift bridge returned a null JSON payload".to_owned())
    })?;
    serde_json::from_str(&json).map_err(|error| AuError::Serialization(error.to_string()))
}

pub fn status_result(status: i32, error_ptr: *mut c_char) -> Result<(), AuError> {
    if status == ffi::status::OK {
        Ok(())
    } else {
        Err(crate::error::from_status(status, error_ptr))
    }
}

pub fn property_status_result(status: i32) -> Result<(), AuError> {
    if status == ffi::status::OK {
        Ok(())
    } else {
        Err(crate::error::from_status(status, core::ptr::null_mut()))
    }
}

pub fn cstring_from_path<P: AsRef<Path>>(path: P) -> Result<CString, AuError> {
    CString::new(path.as_ref().to_string_lossy().into_owned())
        .map_err(|error| AuError::InvalidArgument(error.to_string()))
}

pub fn json_cstring<T: Serialize>(value: &T) -> Result<CString, AuError> {
    let json =
        serde_json::to_string(value).map_err(|error| AuError::Serialization(error.to_string()))?;
    CString::new(json).map_err(|error| AuError::InvalidArgument(error.to_string()))
}
