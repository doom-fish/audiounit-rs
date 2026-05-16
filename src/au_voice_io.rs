//! Voice-processing I/O wrappers built on Apple's `kAudioUnitSubType_VoiceProcessingIO`.

use core::ffi::c_void;

use serde::Deserialize;

use crate::av_audio_unit::{AvAudioUnit, AvAudioUnitInfo, InstantiationOptions};
use crate::error::AuError;
use crate::ffi;
use crate::util::{property_status_result, status_result, take_json};

/// Ducking levels exposed by `AUVoiceIO`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AuVoiceIoOtherAudioDuckingLevel {
    Default = 0,
    Min = 10,
    Mid = 20,
    Max = 30,
}

/// Snapshot of `AUVoiceIO`'s ducking configuration.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AuVoiceIoOtherAudioDuckingConfiguration {
    #[serde(rename = "enableAdvancedDucking")]
    pub enable_advanced_ducking: bool,
    #[serde(rename = "duckingLevel")]
    pub ducking_level: u32,
}

/// Snapshot of an `AUVoiceIO` instance.
#[derive(Debug, Clone, Deserialize)]
pub struct AuVoiceIoInfo {
    #[serde(rename = "avAudioUnit")]
    pub av_audio_unit: AvAudioUnitInfo,
    #[serde(rename = "bypassVoiceProcessing")]
    pub bypass_voice_processing: bool,
    #[serde(rename = "voiceProcessingEnableAGC")]
    pub voice_processing_enable_agc: bool,
    #[serde(rename = "muteOutput")]
    pub mute_output: bool,
    #[serde(rename = "otherAudioDuckingConfiguration")]
    pub other_audio_ducking_configuration: Option<AuVoiceIoOtherAudioDuckingConfiguration>,
}

/// Owned handle to an AUVoiceIO-backed `AVAudioUnit`.
pub struct AuVoiceIo {
    ptr: *mut c_void,
}

/// Alias preserving Apple's acronym casing.
pub type AuVoiceIO = AuVoiceIo;

unsafe impl Send for AuVoiceIo {}

impl Drop for AuVoiceIo {
    fn drop(&mut self) {
        unsafe { ffi::au_voice_io_release(self.ptr) };
    }
}

impl AuVoiceIo {
    /// Create a new voice-processing I/O unit.
    pub fn new(options: InstantiationOptions) -> Result<Self, AuError> {
        let mut unit_ptr = core::ptr::null_mut();
        let mut error_ptr = core::ptr::null_mut();
        let status =
            unsafe { ffi::au_voice_io_create(options as u32, &mut unit_ptr, &mut error_ptr) };
        status_result(status, error_ptr)?;
        Ok(Self { ptr: unit_ptr })
    }

    /// Snapshot the voice-processing metadata.
    pub fn info(&self) -> Result<AuVoiceIoInfo, AuError> {
        let ptr = unsafe { ffi::au_voice_io_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Clone the underlying `AVAudioUnit` handle.
    pub fn as_av_audio_unit(&self) -> AvAudioUnit {
        let ptr = unsafe { ffi::au_voice_io_as_avunit(self.ptr) };
        AvAudioUnit::from_raw(ptr)
    }

    /// Whether voice processing is bypassed.
    pub fn bypass_voice_processing(&self) -> bool {
        unsafe { ffi::au_voice_io_get_bypass_voice_processing(self.ptr) }
    }

    /// Enable or disable bypassing voice processing.
    pub fn set_bypass_voice_processing(&self, value: bool) -> Result<(), AuError> {
        let status = unsafe { ffi::au_voice_io_set_bypass_voice_processing(self.ptr, value) };
        property_status_result(status)
    }

    /// Whether automatic gain control is enabled.
    pub fn enable_agc(&self) -> bool {
        unsafe { ffi::au_voice_io_get_enable_agc(self.ptr) }
    }

    /// Enable or disable AGC.
    pub fn set_enable_agc(&self, value: bool) -> Result<(), AuError> {
        let status = unsafe { ffi::au_voice_io_set_enable_agc(self.ptr, value) };
        property_status_result(status)
    }

    /// Whether output muting is enabled.
    pub fn mute_output(&self) -> bool {
        unsafe { ffi::au_voice_io_get_mute_output(self.ptr) }
    }

    /// Enable or disable output muting.
    pub fn set_mute_output(&self, value: bool) -> Result<(), AuError> {
        let status = unsafe { ffi::au_voice_io_set_mute_output(self.ptr, value) };
        property_status_result(status)
    }

    /// Read the other-audio ducking configuration, if the OS exposes it.
    pub fn other_audio_ducking_configuration(
        &self,
    ) -> Result<Option<AuVoiceIoOtherAudioDuckingConfiguration>, AuError> {
        let ptr = unsafe { ffi::au_voice_io_get_other_audio_ducking_json(self.ptr) };
        if ptr.is_null() {
            Ok(None)
        } else {
            take_json(ptr).map(Some)
        }
    }

    /// Set the other-audio ducking configuration.
    pub fn set_other_audio_ducking(
        &self,
        enable_advanced_ducking: bool,
        ducking_level: AuVoiceIoOtherAudioDuckingLevel,
    ) -> Result<(), AuError> {
        let status = unsafe {
            ffi::au_voice_io_set_other_audio_ducking(
                self.ptr,
                enable_advanced_ducking,
                ducking_level as u32,
            )
        };
        property_status_result(status)
    }
}
