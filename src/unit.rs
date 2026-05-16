//! Backwards-compatible re-exports for the original `unit` module.

pub use crate::au_audio_unit::{AuAudioUnit, AuAudioUnitInfo, AuAudioUnitPreset};
pub use crate::av_audio_unit::{AvAudioUnit, AvAudioUnitInfo, InstantiationOptions};

/// Backwards-compatible alias for the pre-0.2 modern AU handle type.
pub type AuAudioUnitHandle = AuAudioUnit;
