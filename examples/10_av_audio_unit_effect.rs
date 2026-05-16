mod support;

use audiounit::prelude::*;

fn main() -> Result<(), AuError> {
    let effect = AvAudioUnitEffect::new(support::peak_limiter_description())?;
    let info = effect.info()?;
    println!("effect {} bypass={}", info.av_audio_unit.name, info.bypass);
    effect.set_bypass(effect.bypass());
    Ok(())
}
