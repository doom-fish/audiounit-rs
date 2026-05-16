//! `AudioComponentDescription` helpers and type/manufacturer constants.

/// Safe wrapper around `AudioComponentDescription`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AudioComponentDescription {
    pub component_type: u32,
    pub component_subtype: u32,
    pub component_manufacturer: u32,
    pub component_flags: u32,
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
    // ------------------------------------------------------------------
    // Component types (`kAudioUnitType_*`)
    // ------------------------------------------------------------------

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

    // ------------------------------------------------------------------
    // Manufacturer (`kAudioUnitManufacturer_*`)
    // ------------------------------------------------------------------

    /// `kAudioUnitManufacturer_Apple` — 'appl'
    pub const AUDIO_UNIT_MANUFACTURER_APPLE: u32 = 0x6170_706c;

    // ------------------------------------------------------------------
    // Common Apple subtypes
    // ------------------------------------------------------------------

    /// `kAudioUnitSubType_DefaultOutput` — 'def '
    pub const AUDIO_UNIT_SUBTYPE_DEFAULT_OUTPUT: u32 = 0x6465_6620;
    /// `kAudioUnitSubType_SystemOutput` — 'sys '
    pub const AUDIO_UNIT_SUBTYPE_SYSTEM_OUTPUT: u32 = 0x7379_7320;
    /// `kAudioUnitSubType_HALOutput` — 'ahal'
    pub const AUDIO_UNIT_SUBTYPE_HAL_OUTPUT: u32 = 0x6168_616c;
    /// `kAudioUnitSubType_VoiceProcessingIO` — 'vpio'
    pub const AUDIO_UNIT_SUBTYPE_VOICE_PROCESSING_IO: u32 = 0x7670_696f;
    /// `kAudioUnitSubType_DLSSynth` — 'dls '
    pub const AUDIO_UNIT_SUBTYPE_DLS_SYNTH: u32 = 0x646c_7320;
    /// `kAudioUnitSubType_Sampler` — 'samp'
    pub const AUDIO_UNIT_SUBTYPE_SAMPLER: u32 = 0x7361_6d70;
    /// `kAudioUnitSubType_MultiChannelMixer` — 'mcmx'
    pub const AUDIO_UNIT_SUBTYPE_MULTI_CHANNEL_MIXER: u32 = 0x6d63_6d78;
    /// `kAudioUnitSubType_MatrixMixer` — 'mxmx'
    pub const AUDIO_UNIT_SUBTYPE_MATRIX_MIXER: u32 = 0x6d78_6d78;
    /// `kAudioUnitSubType_AUConverter` — 'conv'
    pub const AUDIO_UNIT_SUBTYPE_AU_CONVERTER: u32 = 0x636f_6e76;
    /// `kAudioUnitSubType_PeakLimiter` — 'lmtr'
    pub const AUDIO_UNIT_SUBTYPE_PEAK_LIMITER: u32 = 0x6c6d_7472;
}
