//! `AUParameter` wrappers.

use core::ffi::c_void;
use std::ffi::CString;

use serde::Deserialize;

use crate::error::AuError;
use crate::ffi;
use crate::util::{take_json, take_string};

/// `AUParameterAutomationEventType` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AuParameterAutomationEventType {
    Value = 0,
    Touch = 1,
    Release = 2,
}

/// Snapshot of an `AUParameter`.
#[derive(Debug, Clone, Deserialize)]
pub struct AuParameterInfo {
    pub kind: String,
    pub identifier: String,
    #[serde(rename = "keyPath")]
    pub key_path: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "minValue")]
    pub min_value: f32,
    #[serde(rename = "maxValue")]
    pub max_value: f32,
    pub unit: u32,
    #[serde(rename = "unitName")]
    pub unit_name: Option<String>,
    pub flags: u32,
    pub address: u64,
    #[serde(rename = "valueStrings")]
    pub value_strings: Option<Vec<String>>,
    #[serde(rename = "dependentParameters")]
    pub dependent_parameters: Option<Vec<u64>>,
    pub value: f32,
}

/// Owned handle to an `AUParameter`.
pub struct AuParameter {
    ptr: *mut c_void,
}

unsafe impl Send for AuParameter {}
unsafe impl Sync for AuParameter {}

impl Drop for AuParameter {
    fn drop(&mut self) {
        unsafe { ffi::au_parameter_release(self.ptr) };
    }
}

impl AuParameter {
    pub(crate) fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    /// Snapshot the parameter metadata.
    pub fn info(&self) -> Result<AuParameterInfo, AuError> {
        let ptr = unsafe { ffi::au_parameter_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Short identifier string.
    pub fn identifier(&self) -> String {
        unsafe { take_string(ffi::au_parameter_identifier(self.ptr)).unwrap_or_default() }
    }

    /// Localized display name.
    pub fn display_name(&self) -> String {
        unsafe { take_string(ffi::au_parameter_display_name(self.ptr)).unwrap_or_default() }
    }

    /// Abbreviated display name.
    pub fn display_name_with_length(&self, length: isize) -> String {
        unsafe {
            take_string(ffi::au_parameter_display_name_with_length(self.ptr, length))
                .unwrap_or_default()
        }
    }

    /// Address used to identify the parameter within its tree.
    pub fn address(&self) -> u64 {
        unsafe { ffi::au_parameter_address(self.ptr) }
    }

    /// Minimum value.
    pub fn min_value(&self) -> f32 {
        unsafe { ffi::au_parameter_min_value(self.ptr) }
    }

    /// Maximum value.
    pub fn max_value(&self) -> f32 {
        unsafe { ffi::au_parameter_max_value(self.ptr) }
    }

    /// Raw `AUParameterUnit` enum value.
    pub fn unit(&self) -> u32 {
        unsafe { ffi::au_parameter_unit(self.ptr) }
    }

    /// Current value.
    pub fn value(&self) -> f32 {
        unsafe { ffi::au_parameter_get_value(self.ptr) }
    }

    /// Set the current value.
    pub fn set_value(&self, value: f32) {
        unsafe { ffi::au_parameter_set_value(self.ptr, value) };
    }

    /// Set the current value at a specific host time.
    pub fn set_value_at_host_time(&self, value: f32, host_time: u64) {
        unsafe { ffi::au_parameter_set_value_at_host_time(self.ptr, value, host_time) };
    }

    /// Set the current value with an automation event type.
    pub fn set_value_with_event(
        &self,
        value: f32,
        host_time: u64,
        event_type: AuParameterAutomationEventType,
    ) {
        unsafe {
            ffi::au_parameter_set_value_with_event(self.ptr, value, host_time, event_type as u32);
        };
    }

    /// Localized string representation of `value`.
    pub fn string_from_value(&self, value: f32) -> String {
        unsafe {
            take_string(ffi::au_parameter_string_from_value(self.ptr, value)).unwrap_or_default()
        }
    }

    /// Convert a localized value string back to a numeric value.
    pub fn value_from_string(&self, value: &str) -> Result<f32, AuError> {
        let value =
            CString::new(value).map_err(|error| AuError::InvalidArgument(error.to_string()))?;
        Ok(unsafe { ffi::au_parameter_value_from_string(self.ptr, value.as_ptr()) })
    }
}
