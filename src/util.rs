use core::ffi::c_char;
use std::ffi::{CStr, CString};
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::AuError;
use crate::ffi;

pub fn take_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    // SAFETY: ptr is checked for null above; Swift bridge guarantees it points to a valid,
    // null-terminated C string if not null.
    let string = unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    // SAFETY: ptr is guaranteed to be a valid allocation from Swift bridge; freeing it here
    // is necessary to prevent memory leaks since the bridge allocates the string.
    unsafe { ffi::au_string_free(ptr) };
    Some(string)
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
