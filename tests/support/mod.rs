#![allow(dead_code)]

use audiounit::prelude::*;

pub const fn default_output_description() -> AudioComponentDescription {
    AudioComponentDescription::apple(
        AUDIO_UNIT_TYPE_OUTPUT,
        AUDIO_UNIT_SUBTYPE_DEFAULT_OUTPUT,
    )
}

pub const fn peak_limiter_description() -> AudioComponentDescription {
    AudioComponentDescription::apple(
        AUDIO_UNIT_TYPE_EFFECT,
        AUDIO_UNIT_SUBTYPE_PEAK_LIMITER,
    )
}

pub const fn speech_synthesis_description() -> AudioComponentDescription {
    AudioComponentDescription::apple(
        AUDIO_UNIT_TYPE_GENERATOR,
        AUDIO_UNIT_SUBTYPE_SPEECH_SYNTHESIS,
    )
}

pub const fn dls_synth_description() -> AudioComponentDescription {
    AudioComponentDescription::apple(
        AUDIO_UNIT_TYPE_MUSIC_DEVICE,
        AUDIO_UNIT_SUBTYPE_DLS_SYNTH,
    )
}

pub fn first_parameter(unit: &AuAudioUnit) -> Result<AuParameter, AuError> {
    let tree = unit
        .parameter_tree()
        .ok_or_else(|| AuError::Unavailable("unit has no parameter tree".to_owned()))?;
    let group = tree.root_group();
    let parameters = group.all_parameters()?;
    let address = parameters
        .first()
        .ok_or_else(|| AuError::Unavailable("parameter tree was empty".to_owned()))?
        .address;
    tree.parameter_with_address(address).ok_or_else(|| {
        AuError::Unavailable("failed to resolve first parameter by address".to_owned())
    })
}

pub const fn ducking_level_from_raw(raw: u32) -> AuVoiceIoOtherAudioDuckingLevel {
    match raw {
        10 => AuVoiceIoOtherAudioDuckingLevel::Min,
        20 => AuVoiceIoOtherAudioDuckingLevel::Mid,
        30 => AuVoiceIoOtherAudioDuckingLevel::Max,
        _ => AuVoiceIoOtherAudioDuckingLevel::Default,
    }
}
