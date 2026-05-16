//! `AUAudioUnitBusArray` wrappers.

use core::ffi::c_void;

use serde::Deserialize;

use crate::au_audio_unit_bus::{AuAudioUnitBus, AuAudioUnitBusInfo};
use crate::error::AuError;
use crate::ffi;
use crate::util::{status_result, take_json};

/// Snapshot of an `AUAudioUnitBusArray`.
#[derive(Debug, Clone, Deserialize)]
pub struct AuAudioUnitBusArrayInfo {
    pub count: usize,
    #[serde(rename = "countChangeable")]
    pub count_changeable: bool,
    #[serde(rename = "busType")]
    pub bus_type: u64,
    pub busses: Vec<AuAudioUnitBusInfo>,
}

/// Owned handle to an `AUAudioUnitBusArray`.
pub struct AuAudioUnitBusArray {
    ptr: *mut c_void,
}

unsafe impl Send for AuAudioUnitBusArray {}

impl Drop for AuAudioUnitBusArray {
    fn drop(&mut self) {
        unsafe { ffi::au_bus_array_release(self.ptr) };
    }
}

impl AuAudioUnitBusArray {
    pub(crate) fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    /// Snapshot the array metadata.
    pub fn info(&self) -> Result<AuAudioUnitBusArrayInfo, AuError> {
        let ptr = unsafe { ffi::au_bus_array_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Convenience accessor for the current array length.
    pub fn len(&self) -> Result<usize, AuError> {
        Ok(self.info()?.count)
    }

    /// Returns true when the array is empty.
    pub fn is_empty(&self) -> Result<bool, AuError> {
        Ok(self.len()? == 0)
    }

    /// Get the bus at `index`.
    pub fn bus_at(&self, index: usize) -> Option<AuAudioUnitBus> {
        let ptr = unsafe { ffi::au_bus_array_bus_at(self.ptr, index) };
        if ptr.is_null() {
            None
        } else {
            Some(AuAudioUnitBus::from_raw(ptr))
        }
    }

    /// Change the number of busses in the array.
    pub fn set_bus_count(&self, count: usize) -> Result<(), AuError> {
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe { ffi::au_bus_array_set_bus_count(self.ptr, count, &mut error_ptr) };
        status_result(status, error_ptr)
    }
}
