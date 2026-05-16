//! `AVAudioUnitReverb` wrappers.

use core::ffi::c_void;

use serde::Deserialize;

use crate::av_audio_unit::AvAudioUnit;
use crate::av_audio_unit_effect::{AvAudioUnitEffect, AvAudioUnitEffectInfo};
use crate::error::AuError;
use crate::ffi;
use crate::util::{status_result, take_json};

/// Factory presets for `AVAudioUnitReverb`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum AvAudioUnitReverbPreset {
    SmallRoom = 0,
    MediumRoom = 1,
    LargeRoom = 2,
    MediumHall = 3,
    LargeHall = 4,
    Plate = 5,
    MediumChamber = 6,
    LargeChamber = 7,
    Cathedral = 8,
    LargeRoom2 = 9,
    MediumHall2 = 10,
    MediumHall3 = 11,
    LargeHall2 = 12,
}

/// Snapshot of an `AVAudioUnitReverb`.
#[derive(Debug, Clone, Deserialize)]
pub struct AvAudioUnitReverbInfo {
    #[serde(flatten)]
    pub effect: AvAudioUnitEffectInfo,
    #[serde(rename = "wetDryMix")]
    pub wet_dry_mix: f32,
}

/// Owned handle to an `AVAudioUnitReverb`.
pub struct AvAudioUnitReverb {
    ptr: *mut c_void,
}

unsafe impl Send for AvAudioUnitReverb {}

impl Drop for AvAudioUnitReverb {
    fn drop(&mut self) {
        unsafe { ffi::au_av_reverb_release(self.ptr) };
    }
}

impl AvAudioUnitReverb {
    /// Create a reverb unit.
    pub fn new() -> Result<Self, AuError> {
        let mut unit_ptr = core::ptr::null_mut();
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe { ffi::au_av_reverb_create(&mut unit_ptr, &mut error_ptr) };
        status_result(status, error_ptr)?;
        Ok(Self { ptr: unit_ptr })
    }

    /// Snapshot the reverb metadata.
    pub fn info(&self) -> Result<AvAudioUnitReverbInfo, AuError> {
        let ptr = unsafe { ffi::au_av_reverb_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Load a factory preset.
    pub fn load_factory_preset(&self, preset: AvAudioUnitReverbPreset) {
        unsafe { ffi::au_av_reverb_load_factory_preset(self.ptr, preset as i64) };
    }

    /// Current wet/dry mix.
    pub fn wet_dry_mix(&self) -> f32 {
        unsafe { ffi::au_av_reverb_get_wet_dry_mix(self.ptr) }
    }

    /// Set wet/dry mix.
    pub fn set_wet_dry_mix(&self, value: f32) {
        unsafe { ffi::au_av_reverb_set_wet_dry_mix(self.ptr, value) };
    }

    /// Clone the base `AVAudioUnitEffect` handle.
    pub fn as_effect(&self) -> AvAudioUnitEffect {
        let ptr = unsafe { ffi::au_av_reverb_as_effect(self.ptr) };
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
