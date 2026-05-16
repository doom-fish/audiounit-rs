use core::ffi::c_char;
use std::ffi::CStr;

use serde::de::DeserializeOwned;

use crate::error::AuError;
use crate::ffi;

pub fn take_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let string = unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned();
    unsafe { ffi::au_string_free(ptr) };
    Some(string)
}

pub fn take_json<T: DeserializeOwned>(ptr: *mut c_char) -> Result<T, AuError> {
    let json = take_string(ptr).ok_or_else(|| AuError::Serialization("Swift bridge returned a null JSON payload".to_owned()))?;
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
