//! `AudioComponentDescription` helpers and type/manufacturer constants.

use serde::{Deserialize, Serialize};

/// Safe wrapper around `AudioComponentDescription`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioComponentDescription {
    #[serde(rename = "componentType")]
    pub component_type: u32,
    #[serde(rename = "componentSubType")]
    pub component_subtype: u32,
    #[serde(rename = "componentManufacturer")]
    pub component_manufacturer: u32,
    #[serde(rename = "componentFlags")]
    pub component_flags: u32,
    #[serde(rename = "componentFlagsMask")]
    pub component_flags_mask: u32,
}

impl AudioComponentDescription {
    /// Construct from individual fields.
    #[must_use]
    pub const fn new(
        component_type: u32,
        component_subtype: u32,
        component_manufacturer: u32,
    ) -> Self {
        Self {
            component_type,
            component_subtype,
            component_manufacturer,
            component_flags: 0,
            component_flags_mask: 0,
        }
    }

    /// Construct from all fields.
    #[must_use]
    pub const fn with_flags(
        component_type: u32,
        component_subtype: u32,
        component_manufacturer: u32,
        component_flags: u32,
        component_flags_mask: u32,
    ) -> Self {
        Self {
            component_type,
            component_subtype,
            component_manufacturer,
            component_flags,
            component_flags_mask,
        }
    }

    /// Construct an Apple-supplied component description.
    #[must_use]
    pub const fn apple(component_type: u32, component_subtype: u32) -> Self {
        Self::new(
            component_type,
            component_subtype,
            constants::AUDIO_UNIT_MANUFACTURER_APPLE,
        )
    }

    /// All-zeros description — used to match all components.
    #[must_use]
    pub const fn any() -> Self {
        Self {
            component_type: 0,
            component_subtype: 0,
            component_manufacturer: 0,
            component_flags: 0,
            component_flags_mask: 0,
        }
    }
}

/// Type and manufacturer four-CC constants from `AUComponent.h`.
pub mod constants {
    /// `kAudioUnitType_Output` — 'auou'
    pub const AUDIO_UNIT_TYPE_OUTPUT: u32 = 0x6175_6f75;
    /// `kAudioUnitType_MusicDevice` — 'aumu'
    pub const AUDIO_UNIT_TYPE_MUSIC_DEVICE: u32 = 0x6175_6d75;
    /// `kAudioUnitType_MusicEffect` — 'aumf'
    pub const AUDIO_UNIT_TYPE_MUSIC_EFFECT: u32 = 0x6175_6d66;
    /// `kAudioUnitType_FormatConverter` — 'aufc'
    pub const AUDIO_UNIT_TYPE_FORMAT_CONVERTER: u32 = 0x6175_6663;
    /// `kAudioUnitType_Effect` — 'aufx'
    pub const AUDIO_UNIT_TYPE_EFFECT: u32 = 0x6175_6678;
    /// `kAudioUnitType_Mixer` — 'aumx'
    pub const AUDIO_UNIT_TYPE_MIXER: u32 = 0x6175_6d78;
    /// `kAudioUnitType_Panner` — 'aupn'
    pub const AUDIO_UNIT_TYPE_PANNER: u32 = 0x6175_706e;
    /// `kAudioUnitType_Generator` — 'augn'
    pub const AUDIO_UNIT_TYPE_GENERATOR: u32 = 0x6175_676e;
    /// `kAudioUnitType_OfflineEffect` — 'auol'
    pub const AUDIO_UNIT_TYPE_OFFLINE_EFFECT: u32 = 0x6175_6f6c;
    /// `kAudioUnitType_MIDIProcessor` — 'aumi'
    pub const AUDIO_UNIT_TYPE_MIDI_PROCESSOR: u32 = 0x6175_6d69;

    /// `kAudioUnitManufacturer_Apple` — 'appl'
    pub const AUDIO_UNIT_MANUFACTURER_APPLE: u32 = 0x6170_706c;

    /// `kAudioUnitSubType_DefaultOutput` — 'def '
    pub const AUDIO_UNIT_SUBTYPE_DEFAULT_OUTPUT: u32 = 0x6465_6620;
    /// `kAudioUnitSubType_SystemOutput` — 'sys '
    pub const AUDIO_UNIT_SUBTYPE_SYSTEM_OUTPUT: u32 = 0x7379_7320;
    /// `kAudioUnitSubType_HALOutput` — 'ahal'
    pub const AUDIO_UNIT_SUBTYPE_HAL_OUTPUT: u32 = 0x6168_616c;
    /// `kAudioUnitSubType_VoiceProcessingIO` — 'vpio'
    pub const AUDIO_UNIT_SUBTYPE_VOICE_PROCESSING_IO: u32 = 0x7670_696f;
    /// `kAudioUnitSubType_AUAudioTap` — 'atap'
    pub const AUDIO_UNIT_SUBTYPE_AUDIO_TAP_IO: u32 = 0x6174_6170;
    /// `kAudioUnitSubType_GenericOutput` — 'genr'
    pub const AUDIO_UNIT_SUBTYPE_GENERIC_OUTPUT: u32 = 0x6765_6e72;
    /// `kAudioUnitSubType_RemoteIO / AUAudioIO` — 'rioc'
    pub const AUDIO_UNIT_SUBTYPE_AUDIO_IO: u32 = 0x7269_6f63;
    /// `kAudioUnitSubType_DLSSynth` — 'dls '
    pub const AUDIO_UNIT_SUBTYPE_DLS_SYNTH: u32 = 0x646c_7320;
    /// `kAudioUnitSubType_Sampler` — 'samp'
    pub const AUDIO_UNIT_SUBTYPE_SAMPLER: u32 = 0x7361_6d70;
    /// `kAudioUnitSubType_MIDISynth` — 'msyn'
    pub const AUDIO_UNIT_SUBTYPE_MIDI_SYNTH: u32 = 0x6d73_796e;
    /// `kAudioUnitSubType_MultiChannelMixer` — 'mcmx'
    pub const AUDIO_UNIT_SUBTYPE_MULTI_CHANNEL_MIXER: u32 = 0x6d63_6d78;
    /// `kAudioUnitSubType_MatrixMixer` — 'mxmx'
    pub const AUDIO_UNIT_SUBTYPE_MATRIX_MIXER: u32 = 0x6d78_6d78;
    /// `kAudioUnitSubType_AUConverter` — 'conv'
    pub const AUDIO_UNIT_SUBTYPE_AU_CONVERTER: u32 = 0x636f_6e76;
    /// `kAudioUnitSubType_AUAudioFilePlayer` — 'afpl'
    pub const AUDIO_UNIT_SUBTYPE_AUDIO_FILE_PLAYER: u32 = 0x6166_706c;
    /// `kAudioUnitSubType_AUScheduledSoundPlayer` — 'sspl'
    pub const AUDIO_UNIT_SUBTYPE_SCHEDULED_SOUND_PLAYER: u32 = 0x7373_706c;
    /// `kAudioUnitSubType_SpeechSynthesis` — 'ttsp'
    pub const AUDIO_UNIT_SUBTYPE_SPEECH_SYNTHESIS: u32 = 0x7474_7370;
    /// `kAudioUnitSubType_PeakLimiter` — 'lmtr'
    pub const AUDIO_UNIT_SUBTYPE_PEAK_LIMITER: u32 = 0x6c6d_7472;
}
