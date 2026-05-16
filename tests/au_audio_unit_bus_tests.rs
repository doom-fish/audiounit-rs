mod support;

use audiounit::prelude::*;

#[test]
fn au_audio_unit_bus_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let unit = AuAudioUnit::instantiate(
        support::default_output_description(),
        InstantiationOptions::InProcess,
    )?;
    let bus = unit.output_busses().bus_at(0).expect("missing output bus");
    let info = bus.info()?;
    assert_eq!(info.index, 0);
    assert!(info.format.sample_rate > 0.0);
    assert!(bus.owner_audio_unit().info()?.output_bus_count >= 1);
    Ok(())
}
