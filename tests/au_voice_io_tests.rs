mod support;

use audiounit::prelude::*;

#[test]
fn au_voice_io_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let voice = AuVoiceIo::new(InstantiationOptions::InProcess)?;
    let info = voice.info()?;
    voice.set_bypass_voice_processing(info.bypass_voice_processing)?;
    voice.set_enable_agc(info.voice_processing_enable_agc)?;
    voice.set_mute_output(info.mute_output)?;
    if let Some(config) = voice.other_audio_ducking_configuration()? {
        voice.set_other_audio_ducking(
            config.enable_advanced_ducking,
            support::ducking_level_from_raw(config.ducking_level),
        )?;
    }
    assert!(!info.av_audio_unit.name.is_empty());
    Ok(())
}
