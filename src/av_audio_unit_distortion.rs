//! `AVAudioUnitDistortion` wrappers.

use core::ffi::c_void;

use serde::Deserialize;

use crate::av_audio_unit::AvAudioUnit;
use crate::av_audio_unit_effect::{AvAudioUnitEffect, AvAudioUnitEffectInfo};
use crate::error::AuError;
use crate::ffi;
use crate::util::{status_result, take_json};

/// Factory presets for `AVAudioUnitDistortion`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum AvAudioUnitDistortionPreset {
    DrumsBitBrush = 0,
    DrumsBufferBeats = 1,
    DrumsLoFi = 2,
    MultiBrokenSpeaker = 3,
    MultiCellphoneConcert = 4,
    MultiDecimated1 = 5,
    MultiDecimated2 = 6,
    MultiDecimated3 = 7,
    MultiDecimated4 = 8,
    MultiDistortedFunk = 9,
    MultiDistortedCubed = 10,
    MultiDistortedSquared = 11,
    MultiEcho1 = 12,
    MultiEcho2 = 13,
    MultiEchoTight1 = 14,
    MultiEchoTight2 = 15,
    MultiEverythingIsBroken = 16,
    SpeechAlienChatter = 17,
    SpeechCosmicInterference = 18,
    SpeechGoldenPi = 19,
    SpeechRadioTower = 20,
    SpeechWaves = 21,
}

/// Snapshot of an `AVAudioUnitDistortion`.
#[derive(Debug, Clone, Deserialize)]
pub struct AvAudioUnitDistortionInfo {
    #[serde(flatten)]
    pub effect: AvAudioUnitEffectInfo,
    #[serde(rename = "preGain")]
    pub pre_gain: f32,
    #[serde(rename = "wetDryMix")]
    pub wet_dry_mix: f32,
}

/// Owned handle to an `AVAudioUnitDistortion`.
pub struct AvAudioUnitDistortion {
    ptr: *mut c_void,
}

unsafe impl Send for AvAudioUnitDistortion {}

impl Drop for AvAudioUnitDistortion {
    fn drop(&mut self) {
        unsafe { ffi::au_av_distortion_release(self.ptr) };
    }
}

impl AvAudioUnitDistortion {
    /// Create a distortion unit.
    pub fn new() -> Result<Self, AuError> {
        let mut unit_ptr = core::ptr::null_mut();
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe { ffi::au_av_distortion_create(&mut unit_ptr, &mut error_ptr) };
        status_result(status, error_ptr)?;
        Ok(Self { ptr: unit_ptr })
    }

    /// Snapshot the distortion metadata.
    pub fn info(&self) -> Result<AvAudioUnitDistortionInfo, AuError> {
        let ptr = unsafe { ffi::au_av_distortion_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Load a factory preset.
    pub fn load_factory_preset(&self, preset: AvAudioUnitDistortionPreset) {
        unsafe { ffi::au_av_distortion_load_factory_preset(self.ptr, preset as i64) };
    }

    /// Current pre-gain.
    pub fn pre_gain(&self) -> f32 {
        unsafe { ffi::au_av_distortion_get_pre_gain(self.ptr) }
    }

    /// Set pre-gain.
    pub fn set_pre_gain(&self, value: f32) {
        unsafe { ffi::au_av_distortion_set_pre_gain(self.ptr, value) };
    }

    /// Current wet/dry mix.
    pub fn wet_dry_mix(&self) -> f32 {
        unsafe { ffi::au_av_distortion_get_wet_dry_mix(self.ptr) }
    }

    /// Set wet/dry mix.
    pub fn set_wet_dry_mix(&self, value: f32) {
        unsafe { ffi::au_av_distortion_set_wet_dry_mix(self.ptr, value) };
    }

    /// Clone the base `AVAudioUnitEffect` handle.
    pub fn as_effect(&self) -> AvAudioUnitEffect {
        let ptr = unsafe { ffi::au_av_distortion_as_effect(self.ptr) };
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
