mod support;

use audiounit::prelude::*;

#[test]
fn av_audio_unit_load_preset_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let source = AvAudioUnitDelay::new()?;
    source.set_delay_time(0.42);
    source.set_feedback(21.0);
    let preset_path = support::write_full_state_preset(&source.as_av_audio_unit(), "delay-preset")?;

    let target = AvAudioUnitDelay::new()?;
    target
        .as_av_audio_unit()
        .load_audio_unit_preset(&preset_path)?;
    assert!(!target.info()?.effect.av_audio_unit.name.is_empty());

    let missing_path =
        preset_path.with_file_name(format!("missing-preset-{}.aupreset", std::process::id()));
    assert!(target
        .as_av_audio_unit()
        .load_audio_unit_preset(&missing_path)
        .is_err());

    let _ = std::fs::remove_file(preset_path);
    Ok(())
}

#[test]
fn av_audio_unit_sampler_and_eq_smoke() -> Result<(), Box<dyn std::error::Error>> {
    assert!(support::system_sound_bank_path().exists());

    let sampler = AvAudioUnitSampler::new()?;
    sampler.set_stereo_pan(-10.0);
    sampler.set_overall_gain(-3.0);
    sampler.set_global_tuning(25.0);
    sampler.load_sound_bank_instrument(support::system_sound_bank_path(), 0, 0x79, 0)?;
    let midi = sampler.as_midi_instrument();
    midi.send_program_change(0, 0);
    midi.start_note(60, 96, 0);
    midi.stop_note(60, 0);
    let sampler_info = sampler.info()?;
    assert!(!sampler_info.midi_instrument.av_audio_unit.name.is_empty());

    let eq = AvAudioUnitEq::new(2)?;
    eq.set_global_gain(3.0);
    let band = eq.band_at(0).ok_or("missing eq band")?;
    band.set_filter_type(AvAudioUnitEqFilterType::LowPass);
    band.set_frequency(1_200.0);
    band.set_bandwidth(1.5);
    band.set_gain(-6.0);
    band.set_bypass(false);
    let band_info = band.info()?;
    assert_eq!(band_info.filter_type, AvAudioUnitEqFilterType::LowPass);
    assert!((band_info.frequency - 1_200.0).abs() < 1.0);
    assert_eq!(eq.band_count(), 2);
    assert_eq!(eq.info()?.band_count, 2);
    Ok(())
}

#[test]
fn av_audio_unit_distortion_reverb_and_time_effects_smoke() -> Result<(), Box<dyn std::error::Error>>
{
    let distortion = AvAudioUnitDistortion::new()?;
    distortion.load_factory_preset(AvAudioUnitDistortionPreset::SpeechAlienChatter);
    distortion.set_pre_gain(-12.0);
    distortion.set_wet_dry_mix(65.0);
    distortion.set_bypass(false);
    assert!(!distortion.info()?.effect.av_audio_unit.name.is_empty());

    let reverb = AvAudioUnitReverb::new()?;
    reverb.load_factory_preset(AvAudioUnitReverbPreset::LargeHall);
    reverb.set_wet_dry_mix(40.0);
    reverb.set_bypass(false);
    assert!(!reverb.info()?.effect.av_audio_unit.name.is_empty());

    let generic = AvAudioUnitTimeEffect::new(support::varispeed_description())?;
    generic.set_bypass(false);
    assert_eq!(
        generic
            .info()?
            .av_audio_unit
            .audio_component_description
            .component_subtype,
        support::varispeed_description().component_subtype
    );

    let time_pitch = AvAudioUnitTimePitch::new()?;
    time_pitch.set_rate(1.2);
    time_pitch.set_pitch(150.0);
    time_pitch.set_overlap(10.0);
    time_pitch.set_bypass(false);
    let time_pitch_info = time_pitch.info()?;
    assert!((time_pitch_info.rate - 1.2).abs() < 0.01);
    assert_eq!(
        time_pitch_info
            .time_effect
            .av_audio_unit
            .audio_component_description
            .component_subtype,
        support::new_time_pitch_description().component_subtype
    );

    let varispeed = AvAudioUnitVarispeed::new()?;
    varispeed.set_rate(1.5);
    varispeed.set_bypass(false);
    assert!((varispeed.info()?.rate - 1.5).abs() < 0.01);
    Ok(())
}
