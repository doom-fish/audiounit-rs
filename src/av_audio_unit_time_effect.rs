//! `AVAudioUnitTimeEffect` wrappers.

use core::ffi::c_void;

use serde::Deserialize;

use crate::av_audio_unit::{AvAudioUnit, AvAudioUnitInfo};
use crate::component_description::AudioComponentDescription;
use crate::error::AuError;
use crate::ffi;
use crate::util::{status_result, take_json};

/// Snapshot of an `AVAudioUnitTimeEffect`.
#[derive(Debug, Clone, Deserialize)]
pub struct AvAudioUnitTimeEffectInfo {
    #[serde(flatten)]
    pub av_audio_unit: AvAudioUnitInfo,
    pub bypass: bool,
}

/// Owned handle to an `AVAudioUnitTimeEffect`.
pub struct AvAudioUnitTimeEffect {
    ptr: *mut c_void,
}

unsafe impl Send for AvAudioUnitTimeEffect {}

impl Drop for AvAudioUnitTimeEffect {
    fn drop(&mut self) {
        unsafe { ffi::au_av_time_effect_release(self.ptr) };
    }
}

impl AvAudioUnitTimeEffect {
    pub(crate) fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    /// Create a time-effect unit for the given component description.
    pub fn new(description: AudioComponentDescription) -> Result<Self, AuError> {
        let mut unit_ptr = core::ptr::null_mut();
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_av_time_effect_create(
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

    /// Snapshot the time-effect metadata.
    pub fn info(&self) -> Result<AvAudioUnitTimeEffectInfo, AuError> {
        let ptr = unsafe { ffi::au_av_time_effect_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Current bypass state.
    pub fn bypass(&self) -> bool {
        unsafe { ffi::au_av_time_effect_get_bypass(self.ptr) }
    }

    /// Set the bypass state.
    pub fn set_bypass(&self, value: bool) {
        unsafe { ffi::au_av_time_effect_set_bypass(self.ptr, value) };
    }

    /// Clone the base `AVAudioUnit` handle.
    pub fn as_av_audio_unit(&self) -> AvAudioUnit {
        let ptr = unsafe { ffi::au_av_time_effect_as_avunit(self.ptr) };
        AvAudioUnit::from_raw(ptr)
    }
}
