//! Compatibility layer for the logical `AVAudioUnitInstrument` area.
//!
//! Apple does not expose a separate public `AVAudioUnitInstrument` class in the
//! current macOS SDK. The public base class for instrument-style units is
//! `AVAudioUnitMIDIInstrument`, which this module re-exports.

use crate::av_audio_unit_midi_instrument::{
    AvAudioUnitMidiInstrument, AvAudioUnitMidiInstrumentInfo,
};
use crate::ffi;

/// Compatibility alias for the public `AVAudioUnitMIDIInstrument` type.
pub type AvAudioUnitInstrument = AvAudioUnitMidiInstrument;

/// Compatibility alias for the public `AVAudioUnitMIDIInstrument` snapshot.
pub type AvAudioUnitInstrumentInfo = AvAudioUnitMidiInstrumentInfo;

/// Whether the current SDK exposes a separate public `AVAudioUnitInstrument` class.
pub fn public_api_available() -> bool {
    unsafe { ffi::au_av_instrument_public_api_available() }
}
