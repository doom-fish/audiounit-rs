//! Modern `AUAudioUnit` wrappers.

use core::ffi::c_void;
use std::ffi::CString;

use serde::Deserialize;

use crate::au_audio_unit_bus_array::AuAudioUnitBusArray;
use crate::au_audio_unit_v2_bridge::AuAudioUnitV2Bridge;
use crate::au_parameter_tree::AuParameterTree;
use crate::av_audio_unit::InstantiationOptions;
use crate::component_description::AudioComponentDescription;
use crate::error::AuError;
use crate::ffi;
use crate::util::{status_result, take_json};

/// Snapshot of an `AUAudioUnitPreset`.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AuAudioUnitPreset {
    pub number: isize,
    pub name: String,
}

/// Snapshot of an `AUAudioUnit`.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Deserialize)]
pub struct AuAudioUnitInfo {
    #[serde(rename = "componentDescription")]
    pub component_description: AudioComponentDescription,
    #[serde(rename = "componentName")]
    pub component_name: Option<String>,
    #[serde(rename = "audioUnitName")]
    pub audio_unit_name: Option<String>,
    #[serde(rename = "manufacturerName")]
    pub manufacturer_name: Option<String>,
    #[serde(rename = "audioUnitShortName")]
    pub audio_unit_short_name: Option<String>,
    #[serde(rename = "componentVersion")]
    pub component_version: u32,
    #[serde(rename = "renderResourcesAllocated")]
    pub render_resources_allocated: bool,
    #[serde(rename = "maximumFramesToRender")]
    pub maximum_frames_to_render: usize,
    #[serde(rename = "hasRenderBlock")]
    pub has_render_block: bool,
    #[serde(rename = "hasScheduleParameterBlock")]
    pub has_schedule_parameter_block: bool,
    #[serde(rename = "allParameterValues")]
    pub all_parameter_values: bool,
    #[serde(rename = "musicDeviceOrEffect")]
    pub music_device_or_effect: bool,
    #[serde(rename = "virtualMIDICableCount")]
    pub virtual_midi_cable_count: isize,
    #[serde(rename = "hasScheduleMIDIEventBlock")]
    pub has_schedule_midi_event_block: bool,
    #[serde(rename = "hasScheduleMIDIEventListBlock")]
    pub has_schedule_midi_event_list_block: bool,
    #[serde(rename = "midiOutputNames")]
    pub midi_output_names: Vec<String>,
    #[serde(rename = "providesUserInterface")]
    pub provides_user_interface: bool,
    #[serde(rename = "audioUnitMIDIProtocol")]
    pub audio_unit_midi_protocol: u32,
    #[serde(rename = "hostMIDIProtocol")]
    pub host_midi_protocol: u32,
    #[serde(rename = "fullStatePlist")]
    pub full_state_plist: Option<String>,
    #[serde(rename = "fullStateForDocumentPlist")]
    pub full_state_for_document_plist: Option<String>,
    #[serde(rename = "factoryPresets")]
    pub factory_presets: Vec<AuAudioUnitPreset>,
    #[serde(rename = "userPresets")]
    pub user_presets: Vec<AuAudioUnitPreset>,
    #[serde(rename = "supportsUserPresets")]
    pub supports_user_presets: bool,
    #[serde(rename = "isLoadedInProcess")]
    pub is_loaded_in_process: bool,
    #[serde(rename = "currentPreset")]
    pub current_preset: Option<AuAudioUnitPreset>,
    pub latency: f64,
    #[serde(rename = "tailTime")]
    pub tail_time: f64,
    #[serde(rename = "renderQuality")]
    pub render_quality: isize,
    #[serde(rename = "shouldBypassEffect")]
    pub should_bypass_effect: bool,
    #[serde(rename = "canProcessInPlace")]
    pub can_process_in_place: bool,
    #[serde(rename = "renderingOffline")]
    pub rendering_offline: bool,
    #[serde(rename = "channelCapabilities")]
    pub channel_capabilities: Vec<i32>,
    #[serde(rename = "contextName")]
    pub context_name: Option<String>,
    #[serde(rename = "migrateFromPlugin")]
    pub migrate_from_plugin: Vec<String>,
    #[serde(rename = "supportsMPE")]
    pub supports_mpe: bool,
    #[serde(rename = "channelMap")]
    pub channel_map: Vec<i32>,
    #[serde(rename = "inputBusCount")]
    pub input_bus_count: usize,
    #[serde(rename = "outputBusCount")]
    pub output_bus_count: usize,
    #[serde(rename = "parameterTreeAvailable")]
    pub parameter_tree_available: bool,
    #[serde(rename = "isV2Bridge")]
    pub is_v2_bridge: bool,
}

/// Owned handle to an `AUAudioUnit`.
pub struct AuAudioUnit {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AuAudioUnit {}

impl Drop for AuAudioUnit {
    fn drop(&mut self) {
        unsafe { ffi::au_auaudiounit_release(self.ptr) };
    }
}

impl AuAudioUnit {
    pub(crate) fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    /// Instantiate an `AUAudioUnit` directly.
    pub fn instantiate(
        description: AudioComponentDescription,
        options: InstantiationOptions,
    ) -> Result<Self, AuError> {
        let mut unit_ptr: *mut c_void = core::ptr::null_mut();
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_auaudiounit_instantiate_sync(
                description.component_type,
                description.component_subtype,
                description.component_manufacturer,
                description.component_flags,
                description.component_flags_mask,
                options as u32,
                &mut unit_ptr,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        Ok(Self { ptr: unit_ptr })
    }

    /// Snapshot the unit's current metadata.
    pub fn info(&self) -> Result<AuAudioUnitInfo, AuError> {
        let ptr = unsafe { ffi::au_auaudiounit_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Allocate render resources.
    pub fn allocate_render_resources(&self) -> Result<(), AuError> {
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe { ffi::au_auaudiounit_allocate_render_resources(self.ptr, &mut error_ptr) };
        status_result(status, error_ptr)
    }

    /// Deallocate render resources.
    pub fn deallocate_render_resources(&self) {
        unsafe { ffi::au_auaudiounit_deallocate_render_resources(self.ptr) };
    }

    /// Reset transient rendering state.
    pub fn reset(&self) {
        unsafe { ffi::au_auaudiounit_reset(self.ptr) };
    }

    /// Access the input busses.
    pub fn input_busses(&self) -> AuAudioUnitBusArray {
        let ptr = unsafe { ffi::au_auaudiounit_input_busses(self.ptr) };
        AuAudioUnitBusArray::from_raw(ptr)
    }

    /// Access the output busses.
    pub fn output_busses(&self) -> AuAudioUnitBusArray {
        let ptr = unsafe { ffi::au_auaudiounit_output_busses(self.ptr) };
        AuAudioUnitBusArray::from_raw(ptr)
    }

    /// Access the parameter tree, if available.
    pub fn parameter_tree(&self) -> Option<AuParameterTree> {
        let ptr = unsafe { ffi::au_auaudiounit_parameter_tree(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AuParameterTree::from_raw(ptr))
        }
    }

    /// Get overview parameter addresses.
    pub fn parameters_for_overview(&self, count: usize) -> Result<Vec<u64>, AuError> {
        let ptr = unsafe { ffi::au_auaudiounit_parameters_for_overview_json(self.ptr, count) };
        take_json(ptr)
    }

    /// Set `maximumFramesToRender`.
    pub fn set_maximum_frames_to_render(&self, value: u32) {
        unsafe { ffi::au_auaudiounit_set_maximum_frames_to_render(self.ptr, value) };
    }

    /// Set `renderQuality`.
    pub fn set_render_quality(&self, value: isize) {
        unsafe { ffi::au_auaudiounit_set_render_quality(self.ptr, value) };
    }

    /// Set `shouldBypassEffect`.
    pub fn set_should_bypass_effect(&self, value: bool) {
        unsafe { ffi::au_auaudiounit_set_should_bypass_effect(self.ptr, value) };
    }

    /// Set `renderingOffline`.
    pub fn set_rendering_offline(&self, value: bool) {
        unsafe { ffi::au_auaudiounit_set_rendering_offline(self.ptr, value) };
    }

    /// Set `contextName`.
    pub fn set_context_name(&self, value: Option<&str>) -> Result<(), AuError> {
        let value = value
            .map(CString::new)
            .transpose()
            .map_err(|error| AuError::InvalidArgument(error.to_string()))?;
        unsafe {
            ffi::au_auaudiounit_set_context_name(
                self.ptr,
                value.as_ref().map_or(core::ptr::null(), |value| value.as_ptr()),
            );
        };
        Ok(())
    }

    /// Set or clear the current preset.
    pub fn set_current_preset(&self, preset: Option<&AuAudioUnitPreset>) -> Result<(), AuError> {
        let name = preset
            .map(|preset| CString::new(preset.name.clone()))
            .transpose()
            .map_err(|error| AuError::InvalidArgument(error.to_string()))?;
        unsafe {
            ffi::au_auaudiounit_set_current_preset(
                self.ptr,
                preset.map_or(0, |preset| preset.number),
                name.as_ref().map_or(core::ptr::null(), |value| value.as_ptr()),
            );
        };
        Ok(())
    }

    /// Set or clear the channel map.
    pub fn set_channel_map(&self, map: Option<&[i32]>) {
        match map {
            Some(map) => unsafe {
                ffi::au_auaudiounit_set_channel_map(self.ptr, map.as_ptr(), map.len());
            },
            None => unsafe {
                ffi::au_auaudiounit_set_channel_map(self.ptr, core::ptr::null(), 0);
            },
        }
    }

    /// Attempt to cast the unit to `AUAudioUnitV2Bridge`.
    pub fn as_v2_bridge(&self) -> Option<AuAudioUnitV2Bridge> {
        AuAudioUnitV2Bridge::from_au_audio_unit(self)
    }
}
