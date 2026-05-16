//! Legacy `AudioUnit` C API wrappers.
//!
//! These call the raw `AudioToolbox` C functions directly via `libc`-compatible
//! FFI, since they take an opaque `AudioUnit` pointer (which is itself an
//! `AudioComponentInstance *`).

use core::ffi::c_void;

/// Property IDs (`kAudioUnitProperty_*`).
pub mod property_id {
    pub const SAMPLE_RATE: u32 = 2;
    pub const PARAMETER_LIST: u32 = 3;
    pub const PARAMETER_INFO: u32 = 4;
    pub const STREAM_FORMAT: u32 = 8;
    pub const SET_RENDER_CALLBACK: u32 = 23;
    pub const CLASS_INFO: u32 = 0;
    pub const MAKE_CONNECTION: u32 = 1;
    pub const MAX_FRAMES_PER_SLICE: u32 = 14;
    pub const LAST_RENDER_ERROR: u32 = 22;
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

/// Minimal `AudioTimeStamp` layout (only the fields we need for FFI).
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

// ---------------------------------------------------------------------------
// Raw AudioToolbox bindings
// ---------------------------------------------------------------------------

#[link(name = "AudioToolbox", kind = "framework")]
extern "C" {
    fn AudioUnitGetPropertyInfo(
        in_unit: *mut c_void,
        in_id: u32,
        in_scope: u32,
        in_element: u32,
        out_data_size: *mut u32,
        out_writable: *mut bool,
    ) -> i32;

    fn AudioUnitGetProperty(
        in_unit: *mut c_void,
        in_id: u32,
        in_scope: u32,
        in_element: u32,
        out_data: *mut c_void,
        io_data_size: *mut u32,
    ) -> i32;

    fn AudioUnitSetProperty(
        in_unit: *mut c_void,
        in_id: u32,
        in_scope: u32,
        in_element: u32,
        in_data: *const c_void,
        in_data_size: u32,
    ) -> i32;

    fn AudioUnitGetParameter(
        in_unit: *mut c_void,
        in_id: u32,
        in_scope: u32,
        in_element: u32,
        out_value: *mut f32,
    ) -> i32;

    fn AudioUnitSetParameter(
        in_unit: *mut c_void,
        in_id: u32,
        in_scope: u32,
        in_element: u32,
        in_value: f32,
        in_buffer_offset_in_frames: u32,
    ) -> i32;
}

// ---------------------------------------------------------------------------
// Safe wrappers
// ---------------------------------------------------------------------------

/// Get size and writability of an `AudioUnit` property.
///
/// # Errors
/// Returns the `OSStatus` error code on failure.
///
/// # Safety
/// `unit_ptr` must be a valid `AudioUnit` handle obtained from an
/// [`AvAudioUnit`][crate::unit::AvAudioUnit].
pub unsafe fn audio_unit_get_property_info(
    unit_ptr: *mut c_void,
    property_id: u32,
    scope: u32,
    element: u32,
) -> Result<(u32, bool), i32> {
    let mut size: u32 = 0;
    let mut writable: bool = false;
    let status =
        AudioUnitGetPropertyInfo(unit_ptr, property_id, scope, element, &mut size, &mut writable);
    if status != 0 {
        Err(status)
    } else {
        Ok((size, writable))
    }
}

/// Get an `AudioUnit` property value into a caller-provided buffer.
///
/// # Errors
/// Returns the `OSStatus` error code on failure.
///
/// # Safety
/// `unit_ptr` must be a valid `AudioUnit`, and `out_data` / `io_data_size`
/// must be correctly sized for the property being requested.
pub unsafe fn audio_unit_get_property(
    unit_ptr: *mut c_void,
    property_id: u32,
    scope: u32,
    element: u32,
    out_data: *mut c_void,
    io_data_size: *mut u32,
) -> Result<(), i32> {
    let status =
        AudioUnitGetProperty(unit_ptr, property_id, scope, element, out_data, io_data_size);
    if status != 0 {
        Err(status)
    } else {
        Ok(())
    }
}

/// Set an `AudioUnit` property from a caller-provided buffer.
///
/// # Errors
/// Returns the `OSStatus` error code on failure.
///
/// # Safety
/// `unit_ptr` must be valid and `in_data`/`in_data_size` must match the
/// expected layout for the given property.
pub unsafe fn audio_unit_set_property(
    unit_ptr: *mut c_void,
    property_id: u32,
    scope: u32,
    element: u32,
    in_data: *const c_void,
    in_data_size: u32,
) -> Result<(), i32> {
    let status =
        AudioUnitSetProperty(unit_ptr, property_id, scope, element, in_data, in_data_size);
    if status != 0 {
        Err(status)
    } else {
        Ok(())
    }
}

/// Get an `AudioUnit` parameter value.
///
/// # Errors
/// Returns the `OSStatus` error code on failure.
///
/// # Safety
/// `unit_ptr` must be a valid `AudioUnit`.
pub unsafe fn audio_unit_get_parameter(
    unit_ptr: *mut c_void,
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
/// # Errors
/// Returns the `OSStatus` error code on failure.
///
/// # Safety
/// `unit_ptr` must be a valid `AudioUnit`.
pub unsafe fn audio_unit_set_parameter(
    unit_ptr: *mut c_void,
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

/// Register a render callback on an `AudioUnit`.
///
/// # Errors
/// Returns the `OSStatus` error code on failure.
///
/// # Safety
/// The callback and ref-con must remain valid for the lifetime of the
/// `AudioUnit`. The caller is responsible for removing the callback before
/// the unit is released.
pub unsafe fn audio_unit_set_render_callback(
    unit_ptr: *mut c_void,
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
