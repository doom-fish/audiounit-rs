mod support;

use audiounit::prelude::*;

#[test]
fn av_audio_unit_generator_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let generator = AvAudioUnitGenerator::new(support::speech_synthesis_description())?;
    let bypass = generator.bypass();
    generator.set_bypass(bypass);
    assert!(!generator.info()?.av_audio_unit.name.is_empty());
    Ok(())
}
