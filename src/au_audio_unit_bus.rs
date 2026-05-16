//! `AUAudioUnitBus` wrappers.

use core::ffi::c_void;
use std::ffi::CString;

use serde::Deserialize;

use crate::au_audio_unit::AuAudioUnit;
use crate::error::AuError;
use crate::ffi;
use crate::util::{status_result, take_json};

/// Snapshot of an `AVAudioFormat` used on a bus.
#[derive(Debug, Clone, Deserialize)]
pub struct AudioFormatInfo {
    #[serde(rename = "sampleRate")]
    pub sample_rate: f64,
    #[serde(rename = "channelCount")]
    pub channel_count: u32,
    #[serde(rename = "commonFormat")]
    pub common_format: i64,
    pub interleaved: bool,
    pub standard: bool,
    #[serde(rename = "settingsDescription")]
    pub settings_description: String,
}

/// Snapshot of an `AUAudioUnitBus`.
#[derive(Debug, Clone, Deserialize)]
pub struct AuAudioUnitBusInfo {
    pub format: AudioFormatInfo,
    #[serde(rename = "shouldAllocateBuffer")]
    pub should_allocate_buffer: bool,
    pub enabled: bool,
    pub name: Option<String>,
    pub index: usize,
    #[serde(rename = "busType")]
    pub bus_type: u64,
    #[serde(rename = "supportedChannelLayoutTags")]
    pub supported_channel_layout_tags: Vec<i32>,
    #[serde(rename = "contextPresentationLatency")]
    pub context_presentation_latency: f64,
}

/// Owned handle to an `AUAudioUnitBus`.
pub struct AuAudioUnitBus {
    ptr: *mut c_void,
}

unsafe impl Send for AuAudioUnitBus {}

impl Drop for AuAudioUnitBus {
    fn drop(&mut self) {
        unsafe { ffi::au_bus_release(self.ptr) };
    }
}

impl AuAudioUnitBus {
    pub(crate) fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    /// Snapshot the bus metadata.
    pub fn info(&self) -> Result<AuAudioUnitBusInfo, AuError> {
        let ptr = unsafe { ffi::au_bus_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Set a standard floating-point format on the bus.
    pub fn set_standard_format(&self, sample_rate: f64, channel_count: u32) -> Result<(), AuError> {
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_bus_set_standard_format(self.ptr, sample_rate, channel_count, &mut error_ptr)
        };
        status_result(status, error_ptr)
    }

    /// Set `shouldAllocateBuffer`.
    pub fn set_should_allocate_buffer(&self, value: bool) {
        unsafe { ffi::au_bus_set_should_allocate_buffer(self.ptr, value) };
    }

    /// Set `enabled`.
    pub fn set_enabled(&self, value: bool) {
        unsafe { ffi::au_bus_set_enabled(self.ptr, value) };
    }

    /// Set the bus name.
    pub fn set_name(&self, value: Option<&str>) -> Result<(), AuError> {
        let value = value
            .map(CString::new)
            .transpose()
            .map_err(|error| AuError::InvalidArgument(error.to_string()))?;
        unsafe {
            ffi::au_bus_set_name(
                self.ptr,
                value
                    .as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr()),
            );
        };
        Ok(())
    }

    /// Set `contextPresentationLatency`.
    pub fn set_context_presentation_latency(&self, value: f64) {
        unsafe { ffi::au_bus_set_context_presentation_latency(self.ptr, value) };
    }

    /// Access the owning `AUAudioUnit`.
    pub fn owner_audio_unit(&self) -> AuAudioUnit {
        let ptr = unsafe { ffi::au_bus_owner_audio_unit(self.ptr) };
        AuAudioUnit::from_raw(ptr)
    }
}
