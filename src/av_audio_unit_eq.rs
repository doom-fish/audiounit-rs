//! `AVAudioUnitEQ` wrappers.

use core::ffi::c_void;

use serde::{Deserialize, Deserializer};

use crate::av_audio_unit::AvAudioUnit;
use crate::av_audio_unit_effect::{AvAudioUnitEffect, AvAudioUnitEffectInfo};
use crate::error::AuError;
use crate::ffi;
use crate::util::{status_result, take_json};

/// Safe wrapper for `AVAudioUnitEQFilterType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum AvAudioUnitEqFilterType {
    Parametric = 0,
    LowPass = 1,
    HighPass = 2,
    ResonantLowPass = 3,
    ResonantHighPass = 4,
    BandPass = 5,
    BandStop = 6,
    LowShelf = 7,
    HighShelf = 8,
    ResonantLowShelf = 9,
    ResonantHighShelf = 10,
}

impl AvAudioUnitEqFilterType {
    #[must_use]
    pub const fn from_raw(raw: i64) -> Self {
        match raw {
            1 => Self::LowPass,
            2 => Self::HighPass,
            3 => Self::ResonantLowPass,
            4 => Self::ResonantHighPass,
            5 => Self::BandPass,
            6 => Self::BandStop,
            7 => Self::LowShelf,
            8 => Self::HighShelf,
            9 => Self::ResonantLowShelf,
            10 => Self::ResonantHighShelf,
            _ => Self::Parametric,
        }
    }
}

fn deserialize_filter_type<'de, D>(deserializer: D) -> Result<AvAudioUnitEqFilterType, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(AvAudioUnitEqFilterType::from_raw(i64::deserialize(
        deserializer,
    )?))
}

/// Snapshot of an EQ band.
#[derive(Debug, Clone, Deserialize)]
pub struct AvAudioUnitEqBandInfo {
    #[serde(rename = "filterType", deserialize_with = "deserialize_filter_type")]
    pub filter_type: AvAudioUnitEqFilterType,
    pub frequency: f32,
    pub bandwidth: f32,
    pub gain: f32,
    pub bypass: bool,
}

/// Alias preserving Apple's acronym casing.
pub type AvAudioUnitEQBandInfo = AvAudioUnitEqBandInfo;

/// Snapshot of an `AVAudioUnitEQ`.
#[derive(Debug, Clone, Deserialize)]
pub struct AvAudioUnitEqInfo {
    #[serde(flatten)]
    pub effect: AvAudioUnitEffectInfo,
    #[serde(rename = "globalGain")]
    pub global_gain: f32,
    #[serde(rename = "bandCount")]
    pub band_count: usize,
    pub bands: Vec<AvAudioUnitEqBandInfo>,
}

/// Alias preserving Apple's acronym casing.
pub type AvAudioUnitEQInfo = AvAudioUnitEqInfo;

/// Owned handle to a single `AVAudioUnitEQFilterParameters` object.
pub struct AvAudioUnitEqBand {
    ptr: *mut c_void,
}

/// Alias preserving Apple's acronym casing.
pub type AvAudioUnitEQBand = AvAudioUnitEqBand;

unsafe impl Send for AvAudioUnitEqBand {}

impl Drop for AvAudioUnitEqBand {
    fn drop(&mut self) {
        unsafe { ffi::au_av_eq_band_release(self.ptr) };
    }
}

impl AvAudioUnitEqBand {
    fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    /// Snapshot the band's parameters.
    pub fn info(&self) -> Result<AvAudioUnitEqBandInfo, AuError> {
        let ptr = unsafe { ffi::au_av_eq_band_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Current filter type.
    pub fn filter_type(&self) -> AvAudioUnitEqFilterType {
        let raw = unsafe { ffi::au_av_eq_band_get_filter_type(self.ptr) };
        AvAudioUnitEqFilterType::from_raw(raw)
    }

    /// Set the filter type.
    pub fn set_filter_type(&self, value: AvAudioUnitEqFilterType) {
        unsafe { ffi::au_av_eq_band_set_filter_type(self.ptr, value as i64) };
    }

    /// Current center/cutoff frequency.
    pub fn frequency(&self) -> f32 {
        unsafe { ffi::au_av_eq_band_get_frequency(self.ptr) }
    }

    /// Set the center/cutoff frequency.
    pub fn set_frequency(&self, value: f32) {
        unsafe { ffi::au_av_eq_band_set_frequency(self.ptr, value) };
    }

    /// Current bandwidth in octaves.
    pub fn bandwidth(&self) -> f32 {
        unsafe { ffi::au_av_eq_band_get_bandwidth(self.ptr) }
    }

    /// Set bandwidth in octaves.
    pub fn set_bandwidth(&self, value: f32) {
        unsafe { ffi::au_av_eq_band_set_bandwidth(self.ptr, value) };
    }

    /// Current gain in dB.
    pub fn gain(&self) -> f32 {
        unsafe { ffi::au_av_eq_band_get_gain(self.ptr) }
    }

    /// Set gain in dB.
    pub fn set_gain(&self, value: f32) {
        unsafe { ffi::au_av_eq_band_set_gain(self.ptr, value) };
    }

    /// Whether the band is bypassed.
    pub fn bypass(&self) -> bool {
        unsafe { ffi::au_av_eq_band_get_bypass(self.ptr) }
    }

    /// Set the band's bypass state.
    pub fn set_bypass(&self, value: bool) {
        unsafe { ffi::au_av_eq_band_set_bypass(self.ptr, value) };
    }
}

/// Owned handle to an `AVAudioUnitEQ`.
pub struct AvAudioUnitEq {
    ptr: *mut c_void,
}

/// Alias preserving Apple's acronym casing.
pub type AvAudioUnitEQ = AvAudioUnitEq;

/// Alias preserving Apple's acronym casing.
pub type AvAudioUnitEQFilterType = AvAudioUnitEqFilterType;

unsafe impl Send for AvAudioUnitEq {}

impl Drop for AvAudioUnitEq {
    fn drop(&mut self) {
        unsafe { ffi::au_av_eq_release(self.ptr) };
    }
}

impl AvAudioUnitEq {
    /// Create an EQ with the requested number of bands.
    pub fn new(number_of_bands: usize) -> Result<Self, AuError> {
        let mut unit_ptr = core::ptr::null_mut();
        let mut error_ptr = core::ptr::null_mut();
        let status =
            unsafe { ffi::au_av_eq_create(number_of_bands, &mut unit_ptr, &mut error_ptr) };
        status_result(status, error_ptr)?;
        Ok(Self { ptr: unit_ptr })
    }

    /// Snapshot the EQ metadata.
    pub fn info(&self) -> Result<AvAudioUnitEqInfo, AuError> {
        let ptr = unsafe { ffi::au_av_eq_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Current global gain.
    pub fn global_gain(&self) -> f32 {
        unsafe { ffi::au_av_eq_get_global_gain(self.ptr) }
    }

    /// Set global gain.
    pub fn set_global_gain(&self, value: f32) {
        unsafe { ffi::au_av_eq_set_global_gain(self.ptr, value) };
    }

    /// Number of available EQ bands.
    pub fn band_count(&self) -> usize {
        unsafe { ffi::au_av_eq_band_count(self.ptr) }
    }

    /// Access a specific EQ band.
    pub fn band_at(&self, index: usize) -> Option<AvAudioUnitEqBand> {
        let ptr = unsafe { ffi::au_av_eq_band_at(self.ptr, index) };
        if ptr.is_null() {
            None
        } else {
            Some(AvAudioUnitEqBand::from_raw(ptr))
        }
    }

    /// Clone the base `AVAudioUnitEffect` handle.
    pub fn as_effect(&self) -> AvAudioUnitEffect {
        let ptr = unsafe { ffi::au_av_eq_as_effect(self.ptr) };
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
