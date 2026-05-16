//! Legacy `AudioUnit` / `MusicDevice` C API wrappers.
//!
//! These call the raw `AudioToolbox` C functions directly via `libc`-compatible
//! FFI, since they take opaque `AudioUnit` / `AudioComponentInstance` pointers.

use core::ffi::c_void;

/// Opaque `AudioComponent` handle.
pub type AudioComponent = *mut c_void;
/// Opaque `AudioComponentInstance` handle.
pub type AudioComponentInstance = *mut c_void;
/// Opaque `AudioUnit` handle.
pub type AudioUnit = *mut c_void;
/// Opaque `MusicDeviceComponent` handle.
pub type MusicDeviceComponent = *mut c_void;
/// `AudioUnitPropertyListenerProc` callback prototype.
pub type AudioUnitPropertyListener = unsafe extern "C" fn(
    in_ref_con: *mut c_void,
    in_unit: *mut c_void,
    in_id: u32,
    in_scope: u32,
    in_element: u32,
);

/// Property IDs (`kAudioUnitProperty_*`).
pub mod property_id {
    pub const CLASS_INFO: u32 = 0;
    pub const MAKE_CONNECTION: u32 = 1;
    pub const SAMPLE_RATE: u32 = 2;
    pub const PARAMETER_LIST: u32 = 3;
    pub const PARAMETER_INFO: u32 = 4;
    pub const STREAM_FORMAT: u32 = 8;
    pub const MAX_FRAMES_PER_SLICE: u32 = 14;
    pub const LAST_RENDER_ERROR: u32 = 22;
    pub const SET_RENDER_CALLBACK: u32 = 23;
}

/// `kAudioUnitScope_*` constants.
pub mod scope {
    pub const GLOBAL: u32 = 0;
    pub const INPUT: u32 = 1;
    pub const OUTPUT: u32 = 2;
    pub const GROUP: u32 = 3;
    pub const PART: u32 = 4;
    pub const NOTE: u32 = 5;
    pub const LAYER: u32 = 6;
    pub const LAYER_ITEM: u32 = 7;
}

/// `AUParameterEventType` constants.
pub mod parameter_event_type {
    pub const IMMEDIATE: u32 = 1;
    pub const RAMPED: u32 = 2;
}

/// `MusicDevice` note event constants.
pub mod music_note_event {
    pub const USE_GROUP_INSTRUMENT: u32 = 0xFFFF_FFFF;
    pub const UNUSED: u32 = 0xFFFF_FFFF;
}

/// Identifier for a music-device instrument / program.
pub type MusicDeviceInstrumentId = u32;
/// Music-device group / MIDI-channel identifier.
pub type MusicDeviceGroupId = u32;
/// Identifier returned by `MusicDeviceStartNote`.
pub type NoteInstanceId = u32;

/// `AURenderCallbackStruct` (matches `CoreAudio` header layout).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AURenderCallbackStruct {
    pub input_proc: Option<AURenderCallback>,
    pub input_proc_ref_con: *mut c_void,
}

unsafe impl Send for AURenderCallbackStruct {}

/// Prototype for an `AURenderCallback`.
///
/// # Safety
/// This is a raw C function pointer; all standard `AudioUnit` callback safety
/// rules apply.
pub type AURenderCallback = unsafe extern "C" fn(
    in_ref_con: *mut c_void,
    io_action_flags: *mut u32,
    in_time_stamp: *const AudioTimeStamp,
    in_bus_number: u32,
    in_number_frames: u32,
    io_data: *mut AudioBufferList,
) -> i32;

/// Minimal `AudioTimeStamp` layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioTimeStamp {
    pub sample_time: f64,
    pub host_time: u64,
    pub rate_scalar: f64,
    pub word_clock_time: u64,
    _smpte_time: [u8; 32],
    pub flags: u32,
    pub reserved: u32,
}

/// `AudioBuffer` — one channel's worth of data.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioBuffer {
    pub number_channels: u32,
    pub data_byte_size: u32,
    pub data: *mut c_void,
}

/// `AudioBufferList` with a flexible `mBuffers` array (C99 FAM).
#[repr(C)]
pub struct AudioBufferList {
    pub number_buffers: u32,
    pub buffers: [AudioBuffer; 1],
}

/// `AudioStreamBasicDescription` (ASBD).
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioStreamBasicDescription {
    pub sample_rate: f64,
    pub format_id: u32,
    pub format_flags: u32,
    pub bytes_per_packet: u32,
    pub frames_per_packet: u32,
    pub bytes_per_frame: u32,
    pub channels_per_frame: u32,
    pub bits_per_channel: u32,
    pub reserved: u32,
}

/// One control/value pair in `MusicDeviceNoteParams`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NoteParamsControlValue {
    pub id: u32,
    pub value: f32,
}

/// Convenience start-note parameters for the common pitch/velocity case.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MusicDeviceStdNoteParams {
    pub arg_count: u32,
    pub pitch: f32,
    pub velocity: f32,
}

impl MusicDeviceStdNoteParams {
    #[must_use]
    pub const fn new(pitch: f32, velocity: f32) -> Self {
        Self {
            arg_count: 2,
            pitch,
            velocity,
        }
    }
}

/// Variable-length note-start parameter struct for `MusicDeviceStartNote`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MusicDeviceNoteParams {
    pub arg_count: u32,
    pub pitch: f32,
    pub velocity: f32,
    pub controls: [NoteParamsControlValue; 1],
}

/// Immediate parameter-event payload.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioUnitParameterEventImmediate {
    pub buffer_offset: u32,
    pub value: f32,
}

/// Ramp parameter-event payload.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioUnitParameterEventRamp {
    pub start_buffer_offset: i32,
    pub duration_in_frames: u32,
    pub start_value: f32,
    pub end_value: f32,
}

/// Union payload for `AudioUnitParameterEvent`.
#[repr(C)]
#[derive(Clone, Copy)]
pub union AudioUnitParameterEventValue {
    pub ramp: AudioUnitParameterEventRamp,
    pub immediate: AudioUnitParameterEventImmediate,
}

/// `AudioUnitParameterEvent` as passed to `AudioUnitScheduleParameters`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AudioUnitParameterEvent {
    pub scope: u32,
    pub element: u32,
    pub parameter: u32,
    pub event_type: u32,
    pub event_values: AudioUnitParameterEventValue,
}

impl AudioUnitParameterEvent {
    #[must_use]
    pub const fn immediate(
        scope: u32,
        element: u32,
        parameter: u32,
        buffer_offset: u32,
        value: f32,
    ) -> Self {
        Self {
            scope,
            element,
            parameter,
            event_type: parameter_event_type::IMMEDIATE,
            event_values: AudioUnitParameterEventValue {
                immediate: AudioUnitParameterEventImmediate {
                    buffer_offset,
                    value,
                },
            },
        }
    }

    #[must_use]
    pub const fn ramp(
        scope: u32,
        element: u32,
        parameter: u32,
        start_buffer_offset: i32,
        duration_in_frames: u32,
        start_value: f32,
        end_value: f32,
    ) -> Self {
        Self {
            scope,
            element,
            parameter,
            event_type: parameter_event_type::RAMPED,
            event_values: AudioUnitParameterEventValue {
                ramp: AudioUnitParameterEventRamp {
                    start_buffer_offset,
                    duration_in_frames,
                    start_value,
                    end_value,
                },
            },
        }
    }
}

#[link(name = "AudioToolbox", kind = "framework")]
extern "C" {
    fn AudioComponentInstanceNew(
        in_component: AudioComponent,
        out_instance: *mut AudioComponentInstance,
    ) -> i32;
    fn AudioComponentInstanceDispose(in_instance: AudioComponentInstance) -> i32;
    fn AudioOutputUnitStart(in_unit: AudioUnit) -> i32;
    fn AudioOutputUnitStop(in_unit: AudioUnit) -> i32;
    fn AudioUnitInitialize(in_unit: AudioUnit) -> i32;
    fn AudioUnitUninitialize(in_unit: AudioUnit) -> i32;

    fn AudioUnitGetPropertyInfo(
        in_unit: AudioUnit,
        in_id: u32,
        in_scope: u32,
        in_element: u32,
        out_data_size: *mut u32,
        out_writable: *mut bool,
    ) -> i32;
    fn AudioUnitGetProperty(
        in_unit: AudioUnit,
        in_id: u32,
        in_scope: u32,
        in_element: u32,
        out_data: *mut c_void,
        io_data_size: *mut u32,
    ) -> i32;
    fn AudioUnitSetProperty(
        in_unit: AudioUnit,
        in_id: u32,
        in_scope: u32,
        in_element: u32,
        in_data: *const c_void,
        in_data_size: u32,
    ) -> i32;
    fn AudioUnitGetParameter(
        in_unit: AudioUnit,
        in_id: u32,
        in_scope: u32,
        in_element: u32,
        out_value: *mut f32,
    ) -> i32;
    fn AudioUnitSetParameter(
        in_unit: AudioUnit,
        in_id: u32,
        in_scope: u32,
        in_element: u32,
        in_value: f32,
        in_buffer_offset_in_frames: u32,
    ) -> i32;
    fn AudioUnitAddPropertyListener(
        in_unit: AudioUnit,
        in_id: u32,
        in_proc: AudioUnitPropertyListener,
        in_proc_user_data: *mut c_void,
    ) -> i32;
    fn AudioUnitRemovePropertyListenerWithUserData(
        in_unit: AudioUnit,
        in_id: u32,
        in_proc: AudioUnitPropertyListener,
        in_proc_user_data: *mut c_void,
    ) -> i32;
    fn AudioUnitScheduleParameters(
        in_unit: AudioUnit,
        in_parameter_event: *const AudioUnitParameterEvent,
        in_num_param_events: u32,
    ) -> i32;
    fn AudioUnitRender(
        in_unit: AudioUnit,
        io_action_flags: *mut u32,
        in_time_stamp: *const AudioTimeStamp,
        in_output_bus_number: u32,
        in_number_frames: u32,
        io_data: *mut AudioBufferList,
    ) -> i32;

    fn MusicDeviceMIDIEvent(
        in_unit: MusicDeviceComponent,
        in_status: u32,
        in_data1: u32,
        in_data2: u32,
        in_offset_sample_frame: u32,
    ) -> i32;
    fn MusicDeviceMIDIEventList(
        in_unit: MusicDeviceComponent,
        in_offset_sample_frame: u32,
        event_list: *const c_void,
    ) -> i32;
    fn MusicDeviceStartNote(
        in_unit: MusicDeviceComponent,
        in_instrument: MusicDeviceInstrumentId,
        in_group_id: MusicDeviceGroupId,
        out_note_instance_id: *mut NoteInstanceId,
        in_offset_sample_frame: u32,
        in_params: *const MusicDeviceNoteParams,
    ) -> i32;
    fn MusicDeviceStopNote(
        in_unit: MusicDeviceComponent,
        in_group_id: MusicDeviceGroupId,
        in_note_instance_id: NoteInstanceId,
        in_offset_sample_frame: u32,
    ) -> i32;
    fn MusicDeviceSysEx(in_unit: MusicDeviceComponent, in_data: *const u8, in_length: u32) -> i32;
}

/// Create a new `AudioComponentInstance` from an `AudioComponent`.
///
/// # Safety
/// `component_ptr` must be a valid `AudioComponent` handle.
pub unsafe fn audio_component_instance_new(
    component_ptr: AudioComponent,
) -> Result<AudioComponentInstance, i32> {
    let mut instance: AudioComponentInstance = core::ptr::null_mut();
    let status = AudioComponentInstanceNew(component_ptr, &mut instance);
    if status != 0 {
        Err(status)
    } else {
        Ok(instance)
    }
}

/// Dispose a previously created `AudioComponentInstance`.
///
/// # Safety
/// `instance_ptr` must be a live instance that is not otherwise owned.
pub unsafe fn audio_component_instance_dispose(
    instance_ptr: AudioComponentInstance,
) -> Result<(), i32> {
    let status = AudioComponentInstanceDispose(instance_ptr);
    if status != 0 {
        Err(status)
    } else {
        Ok(())
    }
}

/// Start an output unit.
///
/// # Safety
/// `unit_ptr` must be a valid output `AudioUnit`.
pub unsafe fn audio_output_unit_start(unit_ptr: AudioUnit) -> Result<(), i32> {
    let status = AudioOutputUnitStart(unit_ptr);
    if status != 0 {
        Err(status)
    } else {
        Ok(())
    }
}

/// Stop an output unit.
///
/// # Safety
/// `unit_ptr` must be a valid output `AudioUnit`.
pub unsafe fn audio_output_unit_stop(unit_ptr: AudioUnit) -> Result<(), i32> {
    let status = AudioOutputUnitStop(unit_ptr);
    if status != 0 {
        Err(status)
    } else {
        Ok(())
    }
}

/// Initialize an `AudioUnit`.
///
/// # Safety
/// `unit_ptr` must be a valid `AudioUnit`.
pub unsafe fn audio_unit_initialize(unit_ptr: AudioUnit) -> Result<(), i32> {
    let status = AudioUnitInitialize(unit_ptr);
    if status != 0 {
        Err(status)
    } else {
        Ok(())
    }
}

/// Uninitialize an `AudioUnit`.
///
/// # Safety
/// `unit_ptr` must be a valid `AudioUnit`.
pub unsafe fn audio_unit_uninitialize(unit_ptr: AudioUnit) -> Result<(), i32> {
    let status = AudioUnitUninitialize(unit_ptr);
    if status != 0 {
        Err(status)
    } else {
        Ok(())
    }
}

/// Get size and writability of an `AudioUnit` property.
///
/// # Safety
/// `unit_ptr` must be a valid `AudioUnit` handle.
pub unsafe fn audio_unit_get_property_info(
    unit_ptr: AudioUnit,
    property_id: u32,
    scope: u32,
    element: u32,
) -> Result<(u32, bool), i32> {
    let mut size: u32 = 0;
    let mut writable: bool = false;
    let status = AudioUnitGetPropertyInfo(
        unit_ptr,
        property_id,
        scope,
        element,
        &mut size,
        &mut writable,
    );
    if status != 0 {
        Err(status)
    } else {
        Ok((size, writable))
    }
}

/// Get an `AudioUnit` property value into a caller-provided buffer.
///
/// # Safety
/// `out_data` / `io_data_size` must match the property layout.
pub unsafe fn audio_unit_get_property(
    unit_ptr: AudioUnit,
    property_id: u32,
    scope: u32,
    element: u32,
    out_data: *mut c_void,
    io_data_size: *mut u32,
) -> Result<(), i32> {
    let status = AudioUnitGetProperty(
        unit_ptr,
        property_id,
        scope,
        element,
        out_data,
        io_data_size,
    );
    if status != 0 {
        Err(status)
    } else {
        Ok(())
    }
}

/// Set an `AudioUnit` property from a caller-provided buffer.
///
/// # Safety
/// `in_data` / `in_data_size` must match the property layout.
pub unsafe fn audio_unit_set_property(
    unit_ptr: AudioUnit,
    property_id: u32,
    scope: u32,
    element: u32,
    in_data: *const c_void,
    in_data_size: u32,
) -> Result<(), i32> {
    let status = AudioUnitSetProperty(unit_ptr, property_id, scope, element, in_data, in_data_size);
    if status != 0 {
        Err(status)
    } else {
        Ok(())
    }
}

/// Get an `AudioUnit` parameter value.
///
/// # Safety
/// `unit_ptr` must be a valid `AudioUnit`.
pub unsafe fn audio_unit_get_parameter(
    unit_ptr: AudioUnit,
    parameter_id: u32,
    scope: u32,
    element: u32,
) -> Result<f32, i32> {
    let mut value: f32 = 0.0;
    let status = AudioUnitGetParameter(unit_ptr, parameter_id, scope, element, &mut value);
    if status != 0 {
        Err(status)
    } else {
        Ok(value)
    }
}

/// Set an `AudioUnit` parameter value.
///
/// # Safety
/// `unit_ptr` must be a valid `AudioUnit`.
pub unsafe fn audio_unit_set_parameter(
    unit_ptr: AudioUnit,
    parameter_id: u32,
    scope: u32,
    element: u32,
    value: f32,
    buffer_offset_in_frames: u32,
) -> Result<(), i32> {
    let status = AudioUnitSetParameter(
        unit_ptr,
        parameter_id,
        scope,
        element,
        value,
        buffer_offset_in_frames,
    );
    if status != 0 {
        Err(status)
    } else {
        Ok(())
    }
}

/// Register a property-change listener on an `AudioUnit`.
///
/// # Safety
/// The callback and `ref_con` must remain valid until removed.
pub unsafe fn audio_unit_add_property_listener(
    unit_ptr: AudioUnit,
    property_id: u32,
    listener: AudioUnitPropertyListener,
    ref_con: *mut c_void,
) -> Result<(), i32> {
    let status = AudioUnitAddPropertyListener(unit_ptr, property_id, listener, ref_con);
    if status != 0 {
        Err(status)
    } else {
        Ok(())
    }
}

/// Remove a previously registered property-change listener.
///
/// # Safety
/// `listener` / `ref_con` must match the original registration.
pub unsafe fn audio_unit_remove_property_listener_with_user_data(
    unit_ptr: AudioUnit,
    property_id: u32,
    listener: AudioUnitPropertyListener,
    ref_con: *mut c_void,
) -> Result<(), i32> {
    let status =
        AudioUnitRemovePropertyListenerWithUserData(unit_ptr, property_id, listener, ref_con);
    if status != 0 {
        Err(status)
    } else {
        Ok(())
    }
}

/// Schedule one or more parameter events for the current render cycle.
///
/// # Safety
/// `events` must point to `event_count` valid `AudioUnitParameterEvent`s.
pub unsafe fn audio_unit_schedule_parameters(
    unit_ptr: AudioUnit,
    events: *const AudioUnitParameterEvent,
    event_count: u32,
) -> Result<(), i32> {
    let status = AudioUnitScheduleParameters(unit_ptr, events, event_count);
    if status != 0 {
        Err(status)
    } else {
        Ok(())
    }
}

/// Invoke `AudioUnitRender`.
///
/// # Safety
/// `time_stamp` and `io_data` must point to valid Core Audio layouts.
pub unsafe fn audio_unit_render(
    unit_ptr: AudioUnit,
    io_action_flags: *mut u32,
    time_stamp: *const AudioTimeStamp,
    output_bus_number: u32,
    number_frames: u32,
    io_data: *mut AudioBufferList,
) -> Result<(), i32> {
    let status = AudioUnitRender(
        unit_ptr,
        io_action_flags,
        time_stamp,
        output_bus_number,
        number_frames,
        io_data,
    );
    if status != 0 {
        Err(status)
    } else {
        Ok(())
    }
}

/// Register a render callback on an `AudioUnit`.
///
/// # Safety
/// The callback and ref-con must remain valid for the lifetime of the
/// registration.
pub unsafe fn audio_unit_set_render_callback(
    unit_ptr: AudioUnit,
    scope: u32,
    element: u32,
    callback: AURenderCallback,
    ref_con: *mut c_void,
) -> Result<(), i32> {
    let cbs = AURenderCallbackStruct {
        input_proc: Some(callback),
        input_proc_ref_con: ref_con,
    };
    let size = u32::try_from(std::mem::size_of::<AURenderCallbackStruct>()).unwrap_or(8);
    audio_unit_set_property(
        unit_ptr,
        property_id::SET_RENDER_CALLBACK,
        scope,
        element,
        core::ptr::addr_of!(cbs).cast(),
        size,
    )
}

/// Send a raw MIDI channel message to a music device.
///
/// # Safety
/// `unit_ptr` must be a valid `MusicDeviceComponent`.
pub unsafe fn music_device_midi_event(
    unit_ptr: MusicDeviceComponent,
    status_byte: u32,
    data1: u32,
    data2: u32,
    offset_sample_frame: u32,
) -> Result<(), i32> {
    let status = MusicDeviceMIDIEvent(unit_ptr, status_byte, data1, data2, offset_sample_frame);
    if status != 0 {
        Err(status)
    } else {
        Ok(())
    }
}

/// Send a raw `MIDIEventList` to a music device.
///
/// # Safety
/// `event_list` must point to a valid `CoreMIDI::MIDIEventList`.
pub unsafe fn music_device_midi_event_list_raw(
    unit_ptr: MusicDeviceComponent,
    offset_sample_frame: u32,
    event_list: *const c_void,
) -> Result<(), i32> {
    let status = MusicDeviceMIDIEventList(unit_ptr, offset_sample_frame, event_list);
    if status != 0 {
        Err(status)
    } else {
        Ok(())
    }
}

/// Start a note using a raw `MusicDeviceNoteParams` pointer.
///
/// # Safety
/// `params` must point to a valid `MusicDeviceNoteParams`-compatible layout.
pub unsafe fn music_device_start_note(
    unit_ptr: MusicDeviceComponent,
    instrument: MusicDeviceInstrumentId,
    group_id: MusicDeviceGroupId,
    offset_sample_frame: u32,
    params: *const MusicDeviceNoteParams,
) -> Result<NoteInstanceId, i32> {
    let mut note_id: NoteInstanceId = 0;
    let status = MusicDeviceStartNote(
        unit_ptr,
        instrument,
        group_id,
        &mut note_id,
        offset_sample_frame,
        params,
    );
    if status != 0 {
        Err(status)
    } else {
        Ok(note_id)
    }
}

/// Start a note using pitch / velocity only.
///
/// # Safety
/// `unit_ptr` must be a valid `MusicDeviceComponent`.
pub unsafe fn music_device_start_note_std(
    unit_ptr: MusicDeviceComponent,
    instrument: MusicDeviceInstrumentId,
    group_id: MusicDeviceGroupId,
    offset_sample_frame: u32,
    pitch: f32,
    velocity: f32,
) -> Result<NoteInstanceId, i32> {
    let params = MusicDeviceStdNoteParams::new(pitch, velocity);
    music_device_start_note(
        unit_ptr,
        instrument,
        group_id,
        offset_sample_frame,
        core::ptr::addr_of!(params).cast(),
    )
}

/// Stop a note started with `MusicDeviceStartNote`.
///
/// # Safety
/// `unit_ptr` must be a valid `MusicDeviceComponent`.
pub unsafe fn music_device_stop_note(
    unit_ptr: MusicDeviceComponent,
    group_id: MusicDeviceGroupId,
    note_instance_id: NoteInstanceId,
    offset_sample_frame: u32,
) -> Result<(), i32> {
    let status = MusicDeviceStopNote(unit_ptr, group_id, note_instance_id, offset_sample_frame);
    if status != 0 {
        Err(status)
    } else {
        Ok(())
    }
}

/// Send a `SysEx` message to a music device.
///
/// # Safety
/// `bytes` must remain valid for the duration of the call.
pub unsafe fn music_device_sysex(unit_ptr: MusicDeviceComponent, bytes: &[u8]) -> Result<(), i32> {
    let status = MusicDeviceSysEx(
        unit_ptr,
        bytes.as_ptr(),
        u32::try_from(bytes.len()).unwrap_or(u32::MAX),
    );
    if status != 0 {
        Err(status)
    } else {
        Ok(())
    }
}
