mod support;

use audiounit::prelude::*;

fn main() -> Result<(), AuError> {
    let components =
        ComponentManager::components_matching_predicate(&ComponentPredicate::all(vec![
            ComponentPredicate::type_name_contains("Effect"),
            ComponentPredicate::manufacturer_name_contains("Apple"),
        ]))?;
    println!("apple effects: {}", components.len());
    if let Some(component) = components.first() {
        println!(
            "first effect: {} architectures={} tags={:?}",
            component.name(),
            component.available_architectures()?.len(),
            component.tags()
        );
    }

    let delay = AvAudioUnitDelay::new()?;
    delay.set_feedback(27.0);
    delay.set_wet_dry_mix(35.0);
    println!(
        "delay {} feedback={} wetDryMix={}",
        delay.info()?.effect.av_audio_unit.name,
        delay.feedback(),
        delay.wet_dry_mix()
    );

    let sampler = AvAudioUnitSampler::new()?;
    sampler.load_sound_bank_instrument(support::system_sound_bank_path(), 0, 0x79, 0)?;
    let midi = sampler.as_midi_instrument();
    midi.start_note(60, 96, 0);
    midi.stop_note(60, 0);
    println!(
        "sampler tuning={} pan={} gain={}",
        sampler.global_tuning(),
        sampler.stereo_pan(),
        sampler.overall_gain()
    );

    let time_pitch = AvAudioUnitTimePitch::new()?;
    time_pitch.set_rate(1.1);
    time_pitch.set_pitch(50.0);
    println!(
        "time pitch rate={} pitch={}",
        time_pitch.rate(),
        time_pitch.pitch()
    );
    Ok(())
}
