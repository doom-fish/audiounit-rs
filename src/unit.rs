//! Backwards-compatible re-exports for the original `unit` module.

pub use crate::au_audio_unit::{
    host_transport_state_flags, AuAudioUnit, AuAudioUnitInfo, AuAudioUnitPreset,
    AuHostMusicalContext, AuHostTransportState, AuMessageChannel, AuMidiCiProfile,
    AuMidiCiProfileId, AuMidiCiProfileState, AuMidiOutputEvent, AuMidiOutputEventListSummary,
    AuRenderObserverEvent, AuRenderObserverToken, AU_EVENT_SAMPLE_TIME_IMMEDIATE,
};
pub use crate::av_audio_unit::{AvAudioUnit, AvAudioUnitInfo, InstantiationOptions};

/// Backwards-compatible alias for the pre-0.2 modern AU handle type.
pub type AuAudioUnitHandle = AuAudioUnit;
