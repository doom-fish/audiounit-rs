//! Modern `AUAudioUnit` wrappers.

use core::ffi::c_void;
use std::ffi::CString;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::au_audio_unit_bus_array::AuAudioUnitBusArray;
use crate::au_audio_unit_v2_bridge::AuAudioUnitV2Bridge;
use crate::au_parameter_tree::AuParameterTree;
use crate::av_audio_unit::InstantiationOptions;
use crate::component_description::AudioComponentDescription;
use crate::error::AuError;
use crate::ffi;
use crate::legacy::{AudioBufferList, AudioTimeStamp};
use crate::util::{json_cstring, property_status_result, status_result, take_json};

/// Special `AUEventSampleTime` value meaning “as soon as possible.”
pub const AU_EVENT_SAMPLE_TIME_IMMEDIATE: i64 = -0x1_0000_0000;

/// `AUHostTransportStateFlags` bitflags.
pub mod host_transport_state_flags {
    /// The transport state changed since the previous callback.
    pub const CHANGED: u64 = 1;
    /// The host transport is moving.
    pub const MOVING: u64 = 2;
    /// The host is recording or armed to record.
    pub const RECORDING: u64 = 4;
    /// The host is cycling / looping.
    pub const CYCLING: u64 = 8;
}

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

/// Opaque token for a render observer capture registered on an `AUAudioUnit`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuRenderObserverToken {
    raw: isize,
}

/// One captured render-observer callback.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AuRenderObserverEvent {
    #[serde(rename = "actionFlags")]
    pub action_flags: u32,
    #[serde(rename = "sampleTime")]
    pub sample_time: f64,
    #[serde(rename = "hostTime")]
    pub host_time: u64,
    #[serde(rename = "frameCount")]
    pub frame_count: u32,
    #[serde(rename = "outputBusNumber")]
    pub output_bus_number: isize,
}

/// One captured MIDI 1.0 output event emitted through `MIDIOutputEventBlock`.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AuMidiOutputEvent {
    #[serde(rename = "eventSampleTime")]
    pub event_sample_time: i64,
    pub cable: u8,
    pub bytes: Vec<u8>,
}

/// Summary of one captured `MIDIOutputEventListBlock` callback.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AuMidiOutputEventListSummary {
    #[serde(rename = "eventSampleTime")]
    pub event_sample_time: i64,
    pub cable: u8,
    pub protocol: i32,
    #[serde(rename = "numPackets")]
    pub num_packets: u32,
}

/// Fixed host musical-context values installed on an `AUAudioUnit`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuHostMusicalContext {
    #[serde(rename = "currentTempo")]
    pub current_tempo: Option<f64>,
    #[serde(rename = "timeSignatureNumerator")]
    pub time_signature_numerator: Option<f64>,
    #[serde(rename = "timeSignatureDenominator")]
    pub time_signature_denominator: Option<isize>,
    #[serde(rename = "currentBeatPosition")]
    pub current_beat_position: Option<f64>,
    #[serde(rename = "sampleOffsetToNextBeat")]
    pub sample_offset_to_next_beat: Option<isize>,
    #[serde(rename = "currentMeasureDownbeatPosition")]
    pub current_measure_downbeat_position: Option<f64>,
}

/// Fixed host transport-state values installed on an `AUAudioUnit`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuHostTransportState {
    #[serde(rename = "transportStateFlags")]
    pub transport_state_flags: u64,
    #[serde(rename = "currentSamplePosition")]
    pub current_sample_position: Option<f64>,
    #[serde(rename = "cycleStartBeatPosition")]
    pub cycle_start_beat_position: Option<f64>,
    #[serde(rename = "cycleEndBeatPosition")]
    pub cycle_end_beat_position: Option<f64>,
}

/// Raw 5-byte MIDI-CI profile identifier.
pub type AuMidiCiProfileId = [u8; 5];

/// Snapshot of one MIDI-CI profile.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AuMidiCiProfile {
    pub name: String,
    #[serde(rename = "profileId")]
    pub profile_id: AuMidiCiProfileId,
}

/// Snapshot of the enabled/disabled MIDI-CI profiles on a cable/channel.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AuMidiCiProfileState {
    #[serde(rename = "enabledProfiles")]
    pub enabled_profiles: Vec<AuMidiCiProfile>,
    #[serde(rename = "disabledProfiles")]
    pub disabled_profiles: Vec<AuMidiCiProfile>,
}

/// Opaque message-channel handle returned by `AUAudioUnit.messageChannelFor:`.
pub struct AuMessageChannel {
    ptr: *mut c_void,
}

unsafe impl Send for AuMessageChannel {}

impl Drop for AuMessageChannel {
    fn drop(&mut self) {
        unsafe { ffi::au_message_channel_release(self.ptr) };
    }
}

impl AuMessageChannel {
    pub(crate) fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    /// Call the underlying `AUMessageChannel` with a JSON object message.
    pub fn call_audio_unit_json(&self, message: &Value) -> Result<Value, AuError> {
        if !message.is_object() {
            return Err(AuError::InvalidArgument(
                "message channel payload must be a JSON object".to_owned(),
            ));
        }
        let message = json_cstring(message)?;
        let mut json_ptr = core::ptr::null_mut();
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_message_channel_call_audio_unit_json(
                self.ptr,
                message.as_ptr(),
                &mut json_ptr,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        take_json(json_ptr)
    }
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

    /// The underlying `AudioComponent` handle.
    pub fn component_ptr(&self) -> *mut c_void {
        unsafe { ffi::au_auaudiounit_component(self.ptr) }
    }

    /// Allocate render resources.
    pub fn allocate_render_resources(&self) -> Result<(), AuError> {
        let mut error_ptr = core::ptr::null_mut();
        let status =
            unsafe { ffi::au_auaudiounit_allocate_render_resources(self.ptr, &mut error_ptr) };
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

    /// Invoke the unit's `renderBlock` without a pull-input callback.
    ///
    /// # Safety
    /// `action_flags`, `time_stamp`, and `output_data` must point to valid Core Audio layouts.
    /// Passing a unit that expects a pull-input block will typically return an `OSStatus` error.
    pub unsafe fn render(
        &self,
        action_flags: *mut u32,
        time_stamp: *const AudioTimeStamp,
        frame_count: u32,
        output_bus_number: isize,
        output_data: *mut AudioBufferList,
    ) -> Result<(), AuError> {
        property_status_result(ffi::au_auaudiounit_render(
            self.ptr,
            action_flags,
            time_stamp.cast(),
            frame_count,
            output_bus_number,
            output_data.cast(),
        ))
    }

    /// Invoke the cached `scheduleParameterBlock`.
    pub fn schedule_parameter(
        &self,
        event_sample_time: i64,
        ramp_duration_sample_frames: u32,
        parameter_address: u64,
        value: f32,
    ) -> Result<(), AuError> {
        property_status_result(unsafe {
            ffi::au_auaudiounit_schedule_parameter(
                self.ptr,
                event_sample_time,
                ramp_duration_sample_frames,
                parameter_address,
                value,
            )
        })
    }

    /// Install a capture-backed render observer and receive its token.
    pub fn add_render_observer_capture(&self) -> Result<AuRenderObserverToken, AuError> {
        let mut token = 0isize;
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_auaudiounit_add_render_observer_capture(self.ptr, &mut token, &mut error_ptr)
        };
        status_result(status, error_ptr)?;
        Ok(AuRenderObserverToken { raw: token })
    }

    /// Drain the events captured for a render observer token.
    pub fn take_render_observer_events(
        &self,
        token: AuRenderObserverToken,
    ) -> Result<Vec<AuRenderObserverEvent>, AuError> {
        let ptr =
            unsafe { ffi::au_auaudiounit_take_render_observer_events_json(self.ptr, token.raw) };
        take_json(ptr)
    }

    /// Remove a previously installed render observer.
    pub fn remove_render_observer(&self, token: AuRenderObserverToken) {
        unsafe { ffi::au_auaudiounit_remove_render_observer_capture(self.ptr, token.raw) };
    }

    /// Invoke `scheduleMIDIEventBlock` with raw MIDI 1.0 bytes.
    pub fn schedule_midi_event(
        &self,
        event_sample_time: i64,
        cable: u8,
        midi_bytes: &[u8],
    ) -> Result<(), AuError> {
        if midi_bytes.is_empty() {
            return Err(AuError::InvalidArgument(
                "midi event payload must not be empty".to_owned(),
            ));
        }
        property_status_result(unsafe {
            ffi::au_auaudiounit_schedule_midi_event(
                self.ptr,
                event_sample_time,
                cable,
                midi_bytes.as_ptr(),
                midi_bytes.len(),
            )
        })
    }

    /// Invoke `scheduleMIDIEventListBlock` with a raw `MIDIEventList` pointer.
    ///
    /// # Safety
    /// `event_list` must point to a valid `CoreMIDI::MIDIEventList`.
    pub unsafe fn schedule_midi_event_list_raw(
        &self,
        event_sample_time: i64,
        cable: u8,
        event_list: *const c_void,
    ) -> Result<(), AuError> {
        if event_list.is_null() {
            return Err(AuError::InvalidArgument(
                "MIDIEventList pointer must not be null".to_owned(),
            ));
        }
        property_status_result(ffi::au_auaudiounit_schedule_midi_event_list(
            self.ptr,
            event_sample_time,
            cable,
            event_list,
        ))
    }

    /// Enable or disable built-in capture for `MIDIOutputEventBlock` callbacks.
    pub fn set_midi_output_event_capture_enabled(&self, enabled: bool) {
        unsafe { ffi::au_auaudiounit_set_midi_output_event_capture_enabled(self.ptr, enabled) };
    }

    /// Drain captured `MIDIOutputEventBlock` callbacks.
    pub fn take_captured_midi_output_events(&self) -> Result<Vec<AuMidiOutputEvent>, AuError> {
        let ptr = unsafe { ffi::au_auaudiounit_take_midi_output_events_json(self.ptr) };
        take_json(ptr)
    }

    /// Enable or disable built-in capture for `MIDIOutputEventListBlock` callbacks.
    pub fn set_midi_output_event_list_capture_enabled(&self, enabled: bool) {
        unsafe {
            ffi::au_auaudiounit_set_midi_output_event_list_capture_enabled(self.ptr, enabled);
        };
    }

    /// Drain captured `MIDIOutputEventListBlock` callbacks.
    pub fn take_captured_midi_output_event_lists(
        &self,
    ) -> Result<Vec<AuMidiOutputEventListSummary>, AuError> {
        let ptr = unsafe { ffi::au_auaudiounit_take_midi_output_event_lists_json(self.ptr) };
        take_json(ptr)
    }

    /// Install or clear a fixed musical-context block.
    pub fn set_musical_context(
        &self,
        context: Option<&AuHostMusicalContext>,
    ) -> Result<(), AuError> {
        let value = context.map(json_cstring).transpose()?;
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_auaudiounit_set_musical_context_json(
                self.ptr,
                value
                    .as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr()),
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)
    }

    /// Return the currently installed fixed musical context, if any.
    pub fn musical_context(&self) -> Result<Option<AuHostMusicalContext>, AuError> {
        let ptr = unsafe { ffi::au_auaudiounit_musical_context_json(self.ptr) };
        take_json(ptr)
    }

    /// Install or clear a fixed transport-state block.
    pub fn set_transport_state(&self, state: Option<&AuHostTransportState>) -> Result<(), AuError> {
        let value = state.map(json_cstring).transpose()?;
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_auaudiounit_set_transport_state_json(
                self.ptr,
                value
                    .as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr()),
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)
    }

    /// Return the currently installed fixed transport state, if any.
    pub fn transport_state(&self) -> Result<Option<AuHostTransportState>, AuError> {
        let ptr = unsafe { ffi::au_auaudiounit_transport_state_json(self.ptr) };
        take_json(ptr)
    }

    /// Query the unit's MIDI-CI profile state for a cable/channel.
    pub fn profile_state_for_cable_channel(
        &self,
        cable: u8,
        channel: u8,
    ) -> Result<Option<AuMidiCiProfileState>, AuError> {
        let mut json_ptr = core::ptr::null_mut();
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_auaudiounit_profile_state_for_cable_channel_json(
                self.ptr,
                cable,
                channel,
                &mut json_ptr,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        if json_ptr.is_null() {
            Ok(None)
        } else {
            take_json(json_ptr)
        }
    }

    /// Enable a MIDI-CI profile on a cable/channel.
    pub fn enable_profile(
        &self,
        profile_id: AuMidiCiProfileId,
        name: Option<&str>,
        cable: u8,
        channel: u8,
    ) -> Result<(), AuError> {
        let name = name
            .map(CString::new)
            .transpose()
            .map_err(|error| AuError::InvalidArgument(error.to_string()))?;
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_auaudiounit_enable_profile(
                self.ptr,
                profile_id.as_ptr(),
                profile_id.len(),
                name.as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr()),
                cable,
                channel,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)
    }

    /// Disable a MIDI-CI profile on a cable/channel.
    pub fn disable_profile(
        &self,
        profile_id: AuMidiCiProfileId,
        name: Option<&str>,
        cable: u8,
        channel: u8,
    ) -> Result<(), AuError> {
        let name = name
            .map(CString::new)
            .transpose()
            .map_err(|error| AuError::InvalidArgument(error.to_string()))?;
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_auaudiounit_disable_profile(
                self.ptr,
                profile_id.as_ptr(),
                profile_id.len(),
                name.as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr()),
                cable,
                channel,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)
    }

    /// Query a named `AUMessageChannel`, if the unit provides one.
    pub fn message_channel(&self, name: &str) -> Result<Option<AuMessageChannel>, AuError> {
        let name =
            CString::new(name).map_err(|error| AuError::InvalidArgument(error.to_string()))?;
        let mut channel_ptr = core::ptr::null_mut();
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_auaudiounit_message_channel(
                self.ptr,
                name.as_ptr(),
                &mut channel_ptr,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        if channel_ptr.is_null() {
            Ok(None)
        } else {
            Ok(Some(AuMessageChannel::from_raw(channel_ptr)))
        }
    }

    /// Whether the unit's I/O device can perform input.
    pub fn can_perform_input(&self) -> bool {
        unsafe { ffi::au_auaudiounit_can_perform_input(self.ptr) }
    }

    /// Whether the unit's I/O device can perform output.
    pub fn can_perform_output(&self) -> bool {
        unsafe { ffi::au_auaudiounit_can_perform_output(self.ptr) }
    }

    /// Select the underlying hardware device for an input/output unit.
    pub fn set_device_id(&self, device_id: u32) -> Result<(), AuError> {
        let mut error_ptr = core::ptr::null_mut();
        let status =
            unsafe { ffi::au_auaudiounit_set_device_id(self.ptr, device_id, &mut error_ptr) };
        status_result(status, error_ptr)
    }

    /// Start the audio hardware for an input/output unit.
    pub fn start_hardware(&self) -> Result<(), AuError> {
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe { ffi::au_auaudiounit_start_hardware(self.ptr, &mut error_ptr) };
        status_result(status, error_ptr)
    }

    /// Stop the audio hardware for an input/output unit.
    pub fn stop_hardware(&self) {
        unsafe { ffi::au_auaudiounit_stop_hardware(self.ptr) };
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
                value
                    .as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr()),
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
                name.as_ref()
                    .map_or(core::ptr::null(), |value| value.as_ptr()),
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
