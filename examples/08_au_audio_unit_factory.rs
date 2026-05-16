mod support;

use audiounit::prelude::*;

fn main() -> Result<(), AuError> {
    let factory = AuAudioUnitFactory::new();
    let unit = factory.create_audio_unit(support::default_output_description())?;
    let info = unit.info()?;
    println!("factory created {}", info.audio_unit_name.unwrap_or_else(|| "<unnamed>".to_owned()));
    Ok(())
}
