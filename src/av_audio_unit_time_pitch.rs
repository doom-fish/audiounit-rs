//! `AVAudioUnitTimePitch` wrappers.

use core::ffi::c_void;

use serde::Deserialize;

use crate::av_audio_unit::AvAudioUnit;
use crate::av_audio_unit_time_effect::{AvAudioUnitTimeEffect, AvAudioUnitTimeEffectInfo};
use crate::error::AuError;
use crate::ffi;
use crate::util::{status_result, take_json};

/// Snapshot of an `AVAudioUnitTimePitch`.
#[derive(Debug, Clone, Deserialize)]
pub struct AvAudioUnitTimePitchInfo {
    #[serde(flatten)]
    pub time_effect: AvAudioUnitTimeEffectInfo,
    pub rate: f32,
    pub pitch: f32,
    pub overlap: f32,
}

/// Owned handle to an `AVAudioUnitTimePitch`.
pub struct AvAudioUnitTimePitch {
    ptr: *mut c_void,
}

unsafe impl Send for AvAudioUnitTimePitch {}

impl Drop for AvAudioUnitTimePitch {
    fn drop(&mut self) {
        unsafe { ffi::au_av_time_pitch_release(self.ptr) };
    }
}

impl AvAudioUnitTimePitch {
    /// Create a time-pitch unit.
    pub fn new() -> Result<Self, AuError> {
        let mut unit_ptr = core::ptr::null_mut();
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe { ffi::au_av_time_pitch_create(&mut unit_ptr, &mut error_ptr) };
        status_result(status, error_ptr)?;
        Ok(Self { ptr: unit_ptr })
    }

    /// Snapshot the time-pitch metadata.
    pub fn info(&self) -> Result<AvAudioUnitTimePitchInfo, AuError> {
        let ptr = unsafe { ffi::au_av_time_pitch_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Current playback rate.
    pub fn rate(&self) -> f32 {
        unsafe { ffi::au_av_time_pitch_get_rate(self.ptr) }
    }

    /// Set the playback rate.
    pub fn set_rate(&self, value: f32) {
        unsafe { ffi::au_av_time_pitch_set_rate(self.ptr, value) };
    }

    /// Current pitch shift in cents.
    pub fn pitch(&self) -> f32 {
        unsafe { ffi::au_av_time_pitch_get_pitch(self.ptr) }
    }

    /// Set the pitch shift in cents.
    pub fn set_pitch(&self, value: f32) {
        unsafe { ffi::au_av_time_pitch_set_pitch(self.ptr, value) };
    }

    /// Current overlap amount.
    pub fn overlap(&self) -> f32 {
        unsafe { ffi::au_av_time_pitch_get_overlap(self.ptr) }
    }

    /// Set the overlap amount.
    pub fn set_overlap(&self, value: f32) {
        unsafe { ffi::au_av_time_pitch_set_overlap(self.ptr, value) };
    }

    /// Clone the base `AVAudioUnitTimeEffect` handle.
    pub fn as_time_effect(&self) -> AvAudioUnitTimeEffect {
        let ptr = unsafe { ffi::au_av_time_pitch_as_time_effect(self.ptr) };
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
