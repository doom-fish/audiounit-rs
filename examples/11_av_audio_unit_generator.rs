mod support;

use audiounit::prelude::*;

fn main() -> Result<(), AuError> {
    let generator = AvAudioUnitGenerator::new(support::speech_synthesis_description())?;
    let info = generator.info()?;
    println!(
        "generator {} bypass={}",
        info.av_audio_unit.name, info.bypass
    );
    generator.set_bypass(generator.bypass());
    Ok(())
}
