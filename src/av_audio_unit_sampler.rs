//! `AVAudioUnitSampler` wrappers.

use core::ffi::c_void;
use std::path::Path;

use serde::Deserialize;

use crate::av_audio_unit::AvAudioUnit;
use crate::av_audio_unit_midi_instrument::{
    AvAudioUnitMidiInstrument, AvAudioUnitMidiInstrumentInfo,
};
use crate::error::AuError;
use crate::ffi;
use crate::util::{cstring_from_path, status_result, take_json};

/// Snapshot of an `AVAudioUnitSampler`.
#[derive(Debug, Clone, Deserialize)]
pub struct AvAudioUnitSamplerInfo {
    #[serde(flatten)]
    pub midi_instrument: AvAudioUnitMidiInstrumentInfo,
    #[serde(rename = "stereoPan")]
    pub stereo_pan: f32,
    #[serde(rename = "overallGain")]
    pub overall_gain: f32,
    #[serde(rename = "globalTuning")]
    pub global_tuning: f32,
}

/// Owned handle to an `AVAudioUnitSampler`.
pub struct AvAudioUnitSampler {
    ptr: *mut c_void,
}

unsafe impl Send for AvAudioUnitSampler {}

impl Drop for AvAudioUnitSampler {
    fn drop(&mut self) {
        unsafe { ffi::au_av_sampler_release(self.ptr) };
    }
}

impl AvAudioUnitSampler {
    /// Create a sampler unit.
    pub fn new() -> Result<Self, AuError> {
        let mut unit_ptr = core::ptr::null_mut();
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe { ffi::au_av_sampler_create(&mut unit_ptr, &mut error_ptr) };
        status_result(status, error_ptr)?;
        Ok(Self { ptr: unit_ptr })
    }

    /// Snapshot the sampler metadata.
    pub fn info(&self) -> Result<AvAudioUnitSamplerInfo, AuError> {
        let ptr = unsafe { ffi::au_av_sampler_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Current stereo pan.
    pub fn stereo_pan(&self) -> f32 {
        unsafe { ffi::au_av_sampler_get_stereo_pan(self.ptr) }
    }

    /// Set stereo pan.
    pub fn set_stereo_pan(&self, value: f32) {
        unsafe { ffi::au_av_sampler_set_stereo_pan(self.ptr, value) };
    }

    /// Current overall gain in dB.
    pub fn overall_gain(&self) -> f32 {
        unsafe { ffi::au_av_sampler_get_overall_gain(self.ptr) }
    }

    /// Set overall gain in dB.
    pub fn set_overall_gain(&self, value: f32) {
        unsafe { ffi::au_av_sampler_set_overall_gain(self.ptr, value) };
    }

    /// Current global tuning in cents.
    pub fn global_tuning(&self) -> f32 {
        unsafe { ffi::au_av_sampler_get_global_tuning(self.ptr) }
    }

    /// Set global tuning in cents.
    pub fn set_global_tuning(&self, value: f32) {
        unsafe { ffi::au_av_sampler_set_global_tuning(self.ptr, value) };
    }

    /// Load an instrument from a DLS or SF2 sound bank.
    #[allow(clippy::similar_names)]
    pub fn load_sound_bank_instrument<P: AsRef<Path>>(
        &self,
        path: P,
        program: u8,
        bank_msb: u8,
        bank_lsb: u8,
    ) -> Result<(), AuError> {
        let path = cstring_from_path(path)?;
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_av_sampler_load_sound_bank_instrument(
                self.ptr,
                path.as_ptr(),
                program,
                bank_msb,
                bank_lsb,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)
    }

    /// Load an instrument preset or audio file.
    pub fn load_instrument<P: AsRef<Path>>(&self, path: P) -> Result<(), AuError> {
        let path = cstring_from_path(path)?;
        let mut error_ptr = core::ptr::null_mut();
        let status =
            unsafe { ffi::au_av_sampler_load_instrument(self.ptr, path.as_ptr(), &mut error_ptr) };
        status_result(status, error_ptr)
    }

    /// Clone the base `AVAudioUnitMIDIInstrument` handle.
    pub fn as_midi_instrument(&self) -> AvAudioUnitMidiInstrument {
        let ptr = unsafe { ffi::au_av_sampler_as_midi_instrument(self.ptr) };
        AvAudioUnitMidiInstrument::from_raw(ptr)
    }

    /// Clone the base `AVAudioUnit` handle.
    pub fn as_av_audio_unit(&self) -> AvAudioUnit {
        self.as_midi_instrument().as_av_audio_unit()
    }
}
