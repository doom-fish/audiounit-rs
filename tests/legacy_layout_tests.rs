//! ABI layout assertions for the `#[repr(C)]` structs in `src/legacy.rs`.
//!
//! These structs mirror Apple's `AudioToolbox` / `CoreAudioTypes` C structs and
//! are passed by value (or via packed buffers) across the raw C FFI boundary.
//! If their size or alignment ever drifts from what the OS ABI expects, the
//! data marshalling silently corrupts. These tests pin the layout so accidental
//! field reordering / type changes are caught at `cargo test` time rather than
//! as runtime garbage.

use std::mem::{align_of, size_of};

use audiounit::legacy::{
    au_verify_ffi_layout, AURenderCallbackStruct, AudioBuffer, AudioBufferList,
    AudioStreamBasicDescription, AudioTimeStamp, AudioUnitParameterEvent,
    AudioUnitParameterEventImmediate, AudioUnitParameterEventRamp, AudioUnitParameterEventValue,
    MusicDeviceNoteParams, MusicDeviceStdNoteParams, NoteParamsControlValue,
};

#[test]
fn au_render_callback_struct_layout() {
    assert_eq!(size_of::<AURenderCallbackStruct>(), 16);
    assert_eq!(align_of::<AURenderCallbackStruct>(), 8);
}

#[test]
fn audio_time_stamp_layout() {
    assert_eq!(size_of::<AudioTimeStamp>(), 64);
    assert_eq!(align_of::<AudioTimeStamp>(), 8);
}

#[test]
fn audio_buffer_layout() {
    assert_eq!(size_of::<AudioBuffer>(), 16);
    assert_eq!(align_of::<AudioBuffer>(), 8);
}

#[test]
fn audio_buffer_list_layout() {
    assert_eq!(size_of::<AudioBufferList>(), 24);
    assert_eq!(align_of::<AudioBufferList>(), 8);
}

#[test]
fn audio_stream_basic_description_layout() {
    assert_eq!(size_of::<AudioStreamBasicDescription>(), 40);
    assert_eq!(align_of::<AudioStreamBasicDescription>(), 8);
}

#[test]
fn note_params_control_value_layout() {
    assert_eq!(size_of::<NoteParamsControlValue>(), 8);
    assert_eq!(align_of::<NoteParamsControlValue>(), 4);
}

#[test]
fn music_device_std_note_params_layout() {
    assert_eq!(size_of::<MusicDeviceStdNoteParams>(), 12);
    assert_eq!(align_of::<MusicDeviceStdNoteParams>(), 4);
}

#[test]
fn music_device_note_params_layout() {
    assert_eq!(size_of::<MusicDeviceNoteParams>(), 20);
    assert_eq!(align_of::<MusicDeviceNoteParams>(), 4);
}

#[test]
fn audio_unit_parameter_event_layout() {
    assert_eq!(size_of::<AudioUnitParameterEventImmediate>(), 8);
    assert_eq!(align_of::<AudioUnitParameterEventImmediate>(), 4);
    assert_eq!(size_of::<AudioUnitParameterEventRamp>(), 16);
    assert_eq!(align_of::<AudioUnitParameterEventRamp>(), 4);
    assert_eq!(size_of::<AudioUnitParameterEventValue>(), 16);
    assert_eq!(align_of::<AudioUnitParameterEventValue>(), 4);
    assert_eq!(size_of::<AudioUnitParameterEvent>(), 32);
    assert_eq!(align_of::<AudioUnitParameterEvent>(), 4);
}

/// Cross-language ABI check: asks the Swift bridge to verify that the real
/// `MemoryLayout` (size/alignment) of the underlying `AudioToolbox` /
/// `CoreAudioTypes` structs matches the values pinned on the Rust side. A
/// `false` return means the Rust definitions genuinely disagree with the OS
/// ABI, which is a real bug.
#[test]
fn legacy_layout_matches_os_abi() {
    // SAFETY: `au_verify_ffi_layout` takes no arguments and only reads
    // compile-time `MemoryLayout` constants in the Swift bridge.
    let matches = unsafe { au_verify_ffi_layout() };
    assert!(
        matches,
        "Swift FFI struct layout disagrees with Rust layout (ABI mismatch)"
    );
}
