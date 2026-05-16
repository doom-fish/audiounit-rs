mod support;

use audiounit::prelude::*;

#[test]
fn av_audio_unit_effect_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let effect = AvAudioUnitEffect::new(support::peak_limiter_description())?;
    let bypass = effect.bypass();
    effect.set_bypass(bypass);
    assert!(!effect.info()?.av_audio_unit.name.is_empty());
    Ok(())
}
