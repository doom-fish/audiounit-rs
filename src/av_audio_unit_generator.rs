//! `AVAudioUnitGenerator` wrappers.

use core::ffi::c_void;

use serde::Deserialize;

use crate::av_audio_unit::{AvAudioUnit, AvAudioUnitInfo};
use crate::component_description::AudioComponentDescription;
use crate::error::AuError;
use crate::ffi;
use crate::util::{status_result, take_json};

/// Snapshot of an `AVAudioUnitGenerator`.
#[derive(Debug, Clone, Deserialize)]
pub struct AvAudioUnitGeneratorInfo {
    #[serde(flatten)]
    pub av_audio_unit: AvAudioUnitInfo,
    pub bypass: bool,
}

/// Owned handle to an `AVAudioUnitGenerator`.
pub struct AvAudioUnitGenerator {
    ptr: *mut c_void,
}

unsafe impl Send for AvAudioUnitGenerator {}

impl Drop for AvAudioUnitGenerator {
    fn drop(&mut self) {
        unsafe { ffi::au_av_generator_release(self.ptr) };
    }
}

impl AvAudioUnitGenerator {
    /// Create a generator unit for the given component description.
    pub fn new(description: AudioComponentDescription) -> Result<Self, AuError> {
        let mut unit_ptr = core::ptr::null_mut();
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_av_generator_create(
                description.component_type,
                description.component_subtype,
                description.component_manufacturer,
                description.component_flags,
                description.component_flags_mask,
                &mut unit_ptr,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        Ok(Self { ptr: unit_ptr })
    }

    /// Snapshot the generator metadata.
    pub fn info(&self) -> Result<AvAudioUnitGeneratorInfo, AuError> {
        let ptr = unsafe { ffi::au_av_generator_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Current bypass state.
    pub fn bypass(&self) -> bool {
        unsafe { ffi::au_av_generator_get_bypass(self.ptr) }
    }

    /// Set the bypass state.
    pub fn set_bypass(&self, value: bool) {
        unsafe { ffi::au_av_generator_set_bypass(self.ptr, value) };
    }

    /// Clone the base `AVAudioUnit` handle.
    pub fn as_av_audio_unit(&self) -> AvAudioUnit {
        let ptr = unsafe { ffi::au_av_generator_as_avunit(self.ptr) };
        AvAudioUnit::from_raw(ptr)
    }
}
