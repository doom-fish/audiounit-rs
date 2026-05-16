//! `AVAudioUnitMIDIInstrument` wrappers.

use core::ffi::c_void;

use serde::Deserialize;

use crate::av_audio_unit::{AvAudioUnit, AvAudioUnitInfo};
use crate::component_description::AudioComponentDescription;
use crate::error::AuError;
use crate::ffi;
use crate::util::{status_result, take_json};

/// Snapshot of an `AVAudioUnitMIDIInstrument`.
#[derive(Debug, Clone, Deserialize)]
pub struct AvAudioUnitMidiInstrumentInfo {
    #[serde(flatten)]
    pub av_audio_unit: AvAudioUnitInfo,
    #[serde(rename = "supportsMIDIEventList")]
    pub supports_midi_event_list: bool,
}

/// Owned handle to an `AVAudioUnitMIDIInstrument`.
pub struct AvAudioUnitMidiInstrument {
    ptr: *mut c_void,
}

/// Backwards-friendly alias preserving Apple's acronym casing.
pub type AvAudioUnitMIDIInstrument = AvAudioUnitMidiInstrument;

unsafe impl Send for AvAudioUnitMidiInstrument {}

impl Drop for AvAudioUnitMidiInstrument {
    fn drop(&mut self) {
        unsafe { ffi::au_av_midi_instrument_release(self.ptr) };
    }
}

impl AvAudioUnitMidiInstrument {
    /// Create a MIDI instrument for the given component description.
    pub fn new(description: AudioComponentDescription) -> Result<Self, AuError> {
        let mut unit_ptr = core::ptr::null_mut();
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_av_midi_instrument_create(
                description.component_type,
                description.component_subtype,
                description.component_manufacturer,
                description.component_flags,
                description.component_flags_mask,
                &mut unit_ptr,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        Ok(Self { ptr: unit_ptr })
    }

    /// Snapshot the instrument metadata.
    pub fn info(&self) -> Result<AvAudioUnitMidiInstrumentInfo, AuError> {
        let ptr = unsafe { ffi::au_av_midi_instrument_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Clone the base `AVAudioUnit` handle.
    pub fn as_av_audio_unit(&self) -> AvAudioUnit {
        let ptr = unsafe { ffi::au_av_midi_instrument_as_avunit(self.ptr) };
        AvAudioUnit::from_raw(ptr)
    }

    /// Send a MIDI note-on event.
    pub fn start_note(&self, note: u8, velocity: u8, channel: u8) {
        unsafe { ffi::au_av_midi_instrument_start_note(self.ptr, note, velocity, channel) };
    }

    /// Send a MIDI note-off event.
    pub fn stop_note(&self, note: u8, channel: u8) {
        unsafe { ffi::au_av_midi_instrument_stop_note(self.ptr, note, channel) };
    }

    /// Send a MIDI controller event.
    pub fn send_controller(&self, controller: u8, value: u8, channel: u8) {
        unsafe { ffi::au_av_midi_instrument_send_controller(self.ptr, controller, value, channel) };
    }

    /// Send a pitch-bend event.
    pub fn send_pitch_bend(&self, value: u16, channel: u8) {
        unsafe { ffi::au_av_midi_instrument_send_pitch_bend(self.ptr, value, channel) };
    }

    /// Send channel pressure.
    pub fn send_pressure(&self, pressure: u8, channel: u8) {
        unsafe { ffi::au_av_midi_instrument_send_pressure(self.ptr, pressure, channel) };
    }

    /// Send polyphonic key pressure.
    pub fn send_pressure_for_key(&self, key: u8, value: u8, channel: u8) {
        unsafe {
            ffi::au_av_midi_instrument_send_pressure_for_key(
                self.ptr, key, value, channel,
            );
        };
    }

    /// Send a program-change event.
    pub fn send_program_change(&self, program: u8, channel: u8) {
        unsafe { ffi::au_av_midi_instrument_send_program_change(self.ptr, program, channel) };
    }

    /// Send a program-change plus bank-select event.
    #[allow(clippy::similar_names)]
    pub fn send_program_change_bank(
        &self,
        program: u8,
        bank_msb_value: u8,
        bank_lsb_value: u8,
        channel: u8,
    ) {
        unsafe {
            ffi::au_av_midi_instrument_send_program_change_bank(
                self.ptr,
                program,
                bank_msb_value,
                bank_lsb_value,
                channel,
            );
        };
    }

    /// Send a two-byte MIDI event.
    pub fn send_midi_event2(&self, status: u8, data1: u8, data2: u8) {
        unsafe { ffi::au_av_midi_instrument_send_midi_event2(self.ptr, status, data1, data2) };
    }

    /// Send a one-byte MIDI event.
    pub fn send_midi_event1(&self, status: u8, data1: u8) {
        unsafe { ffi::au_av_midi_instrument_send_midi_event1(self.ptr, status, data1) };
    }

    /// Send a `SysEx` event.
    pub fn send_sysex(&self, bytes: &[u8]) {
        unsafe { ffi::au_av_midi_instrument_send_sysex(self.ptr, bytes.as_ptr(), bytes.len()) };
    }

    /// Forward a raw `MIDIEventList` pointer to the instrument.
    ///
    /// # Safety
    /// The caller must ensure the pointer refers to a valid `MIDIEventList`.
    pub unsafe fn send_event_list_raw(&self, list: *const c_void) {
        ffi::au_av_midi_instrument_send_event_list(self.ptr, list);
    }
}
