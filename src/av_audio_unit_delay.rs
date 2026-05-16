//! `AVAudioUnitDelay` wrappers.

use core::ffi::c_void;

use serde::Deserialize;

use crate::av_audio_unit::AvAudioUnit;
use crate::av_audio_unit_effect::{AvAudioUnitEffect, AvAudioUnitEffectInfo};
use crate::error::AuError;
use crate::ffi;
use crate::util::{status_result, take_json};

/// Snapshot of an `AVAudioUnitDelay`.
#[derive(Debug, Clone, Deserialize)]
pub struct AvAudioUnitDelayInfo {
    #[serde(flatten)]
    pub effect: AvAudioUnitEffectInfo,
    #[serde(rename = "delayTime")]
    pub delay_time: f64,
    pub feedback: f32,
    #[serde(rename = "lowPassCutoff")]
    pub low_pass_cutoff: f32,
    #[serde(rename = "wetDryMix")]
    pub wet_dry_mix: f32,
}

/// Owned handle to an `AVAudioUnitDelay`.
pub struct AvAudioUnitDelay {
    ptr: *mut c_void,
}

unsafe impl Send for AvAudioUnitDelay {}

impl Drop for AvAudioUnitDelay {
    fn drop(&mut self) {
        unsafe { ffi::au_av_delay_release(self.ptr) };
    }
}

impl AvAudioUnitDelay {
    /// Create a delay unit.
    pub fn new() -> Result<Self, AuError> {
        let mut unit_ptr = core::ptr::null_mut();
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe { ffi::au_av_delay_create(&mut unit_ptr, &mut error_ptr) };
        status_result(status, error_ptr)?;
        Ok(Self { ptr: unit_ptr })
    }

    /// Snapshot the delay metadata.
    pub fn info(&self) -> Result<AvAudioUnitDelayInfo, AuError> {
        let ptr = unsafe { ffi::au_av_delay_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Current delay time in seconds.
    pub fn delay_time(&self) -> f64 {
        unsafe { ffi::au_av_delay_get_delay_time(self.ptr) }
    }

    /// Set the delay time in seconds.
    pub fn set_delay_time(&self, value: f64) {
        unsafe { ffi::au_av_delay_set_delay_time(self.ptr, value) };
    }

    /// Current feedback amount.
    pub fn feedback(&self) -> f32 {
        unsafe { ffi::au_av_delay_get_feedback(self.ptr) }
    }

    /// Set the feedback amount.
    pub fn set_feedback(&self, value: f32) {
        unsafe { ffi::au_av_delay_set_feedback(self.ptr, value) };
    }

    /// Current low-pass cutoff.
    pub fn low_pass_cutoff(&self) -> f32 {
        unsafe { ffi::au_av_delay_get_low_pass_cutoff(self.ptr) }
    }

    /// Set the low-pass cutoff.
    pub fn set_low_pass_cutoff(&self, value: f32) {
        unsafe { ffi::au_av_delay_set_low_pass_cutoff(self.ptr, value) };
    }

    /// Current wet/dry mix.
    pub fn wet_dry_mix(&self) -> f32 {
        unsafe { ffi::au_av_delay_get_wet_dry_mix(self.ptr) }
    }

    /// Set the wet/dry mix.
    pub fn set_wet_dry_mix(&self, value: f32) {
        unsafe { ffi::au_av_delay_set_wet_dry_mix(self.ptr, value) };
    }

    /// Clone the base `AVAudioUnitEffect` handle.
    pub fn as_effect(&self) -> AvAudioUnitEffect {
        let ptr = unsafe { ffi::au_av_delay_as_effect(self.ptr) };
        AvAudioUnitEffect::from_raw(ptr)
    }

    /// Current bypass state.
    pub fn bypass(&self) -> bool {
        self.as_effect().bypass()
    }

    /// Set the bypass state.
    pub fn set_bypass(&self, value: bool) {
        self.as_effect().set_bypass(value);
    }

    /// Clone the base `AVAudioUnit` handle.
    pub fn as_av_audio_unit(&self) -> AvAudioUnit {
        self.as_effect().as_av_audio_unit()
    }
}
