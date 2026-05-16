//! `AVAudioUnitVarispeed` wrappers.

use core::ffi::c_void;

use serde::Deserialize;

use crate::av_audio_unit::AvAudioUnit;
use crate::av_audio_unit_time_effect::{AvAudioUnitTimeEffect, AvAudioUnitTimeEffectInfo};
use crate::error::AuError;
use crate::ffi;
use crate::util::{status_result, take_json};

/// Snapshot of an `AVAudioUnitVarispeed`.
#[derive(Debug, Clone, Deserialize)]
pub struct AvAudioUnitVarispeedInfo {
    #[serde(flatten)]
    pub time_effect: AvAudioUnitTimeEffectInfo,
    pub rate: f32,
}

/// Owned handle to an `AVAudioUnitVarispeed`.
pub struct AvAudioUnitVarispeed {
    ptr: *mut c_void,
}

unsafe impl Send for AvAudioUnitVarispeed {}

impl Drop for AvAudioUnitVarispeed {
    fn drop(&mut self) {
        unsafe { ffi::au_av_varispeed_release(self.ptr) };
    }
}

impl AvAudioUnitVarispeed {
    /// Create a varispeed unit.
    pub fn new() -> Result<Self, AuError> {
        let mut unit_ptr = core::ptr::null_mut();
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe { ffi::au_av_varispeed_create(&mut unit_ptr, &mut error_ptr) };
        status_result(status, error_ptr)?;
        Ok(Self { ptr: unit_ptr })
    }

    /// Snapshot the varispeed metadata.
    pub fn info(&self) -> Result<AvAudioUnitVarispeedInfo, AuError> {
        let ptr = unsafe { ffi::au_av_varispeed_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Current playback rate.
    pub fn rate(&self) -> f32 {
        unsafe { ffi::au_av_varispeed_get_rate(self.ptr) }
    }

    /// Set the playback rate.
    pub fn set_rate(&self, value: f32) {
        unsafe { ffi::au_av_varispeed_set_rate(self.ptr, value) };
    }

    /// Clone the base `AVAudioUnitTimeEffect` handle.
    pub fn as_time_effect(&self) -> AvAudioUnitTimeEffect {
        let ptr = unsafe { ffi::au_av_varispeed_as_time_effect(self.ptr) };
        AvAudioUnitTimeEffect::from_raw(ptr)
    }

    /// Current bypass state.
    pub fn bypass(&self) -> bool {
        self.as_time_effect().bypass()
    }

    /// Set the bypass state.
    pub fn set_bypass(&self, value: bool) {
        self.as_time_effect().set_bypass(value);
    }

    /// Clone the base `AVAudioUnit` handle.
    pub fn as_av_audio_unit(&self) -> AvAudioUnit {
        self.as_time_effect().as_av_audio_unit()
    }
}
