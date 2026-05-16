mod support;

use audiounit::prelude::*;

fn main() -> Result<(), AuError> {
    let voice = AuVoiceIo::new(InstantiationOptions::InProcess)?;
    let info = voice.info()?;
    println!(
        "voice-io {} mute={}",
        info.av_audio_unit.name, info.mute_output
    );
    voice.set_bypass_voice_processing(voice.bypass_voice_processing())?;
    voice.set_enable_agc(voice.enable_agc())?;
    voice.set_mute_output(voice.mute_output())?;
    if let Some(config) = voice.other_audio_ducking_configuration()? {
        voice.set_other_audio_ducking(
            config.enable_advanced_ducking,
            support::ducking_level_from_raw(config.ducking_level),
        )?;
    }
    Ok(())
}
