//! `AVAudioUnit` instantiation and metadata.

use core::ffi::c_void;
use std::ffi::CString;
use std::path::Path;

use serde::Deserialize;

use crate::au_audio_unit::AuAudioUnit;
use crate::component_description::AudioComponentDescription;
use crate::error::AuError;
use crate::ffi;
use crate::util::{status_result, take_json};

/// Options for `AVAudioUnit` instantiation.
#[derive(Debug, Clone, Copy, Default)]
#[repr(u32)]
pub enum InstantiationOptions {
    /// Instantiate in-process (default).
    #[default]
    InProcess = 0,
    /// Instantiate out-of-process.
    OutOfProcess = 1,
}

/// Metadata snapshot for an `AVAudioUnit`.
#[derive(Debug, Clone, Deserialize)]
pub struct AvAudioUnitInfo {
    #[serde(rename = "audioComponentDescription")]
    pub audio_component_description: AudioComponentDescription,
    pub name: String,
    #[serde(rename = "manufacturerName")]
    pub manufacturer_name: String,
    pub version: usize,
}

/// A live `AVAudioUnit` instance.
pub struct AvAudioUnit {
    ptr: *mut c_void,
}

unsafe impl Send for AvAudioUnit {}

impl Drop for AvAudioUnit {
    fn drop(&mut self) {
        unsafe { ffi::au_avunit_release(self.ptr) };
    }
}

impl AvAudioUnit {
    pub(crate) fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    /// Instantiate an `AVAudioUnit` for the given component description.
    pub fn instantiate(
        description: AudioComponentDescription,
        options: InstantiationOptions,
    ) -> Result<Self, AuError> {
        let mut unit_ptr: *mut c_void = core::ptr::null_mut();
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_instantiate_sync(
                description.component_type,
                description.component_subtype,
                description.component_manufacturer,
                description.component_flags,
                description.component_flags_mask,
                options as u32,
                &mut unit_ptr,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        Ok(Self { ptr: unit_ptr })
    }

    /// Snapshot the unit's metadata.
    pub fn info(&self) -> Result<AvAudioUnitInfo, AuError> {
        let ptr = unsafe { ffi::au_avunit_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Load a `.aupreset` file.
    pub fn load_audio_unit_preset<P: AsRef<Path>>(&self, path: P) -> Result<(), AuError> {
        let path = CString::new(path.as_ref().to_string_lossy().into_owned())
            .map_err(|error| AuError::InvalidArgument(error.to_string()))?;
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_avunit_load_audio_unit_preset(self.ptr, path.as_ptr(), &mut error_ptr)
        };
        status_result(status, error_ptr)
    }

    /// The legacy `AudioUnit` opaque pointer.
    pub fn audio_unit_ptr(&self) -> *mut c_void {
        unsafe { ffi::au_avunit_audio_unit(self.ptr) }
    }

    /// Clone the underlying `AUAudioUnit` handle.
    pub fn au_audio_unit(&self) -> AuAudioUnit {
        let ptr = unsafe { ffi::au_avunit_auaudiounit(self.ptr) };
        AuAudioUnit::from_raw(ptr)
    }

    /// Returns the `AUParameterTree` for this unit, if present.
    pub fn parameter_tree(&self) -> Option<crate::au_parameter_tree::AuParameterTree> {
        self.au_audio_unit().parameter_tree()
    }
}
