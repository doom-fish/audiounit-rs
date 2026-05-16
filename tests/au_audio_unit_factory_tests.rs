mod support;

use audiounit::prelude::*;

#[test]
fn au_audio_unit_factory_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let factory = AuAudioUnitFactory::new();
    let unit = factory.create_audio_unit(support::default_output_description())?;
    assert!(unit.info()?.output_bus_count >= 1);
    Ok(())
}
