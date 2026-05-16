mod support;

use audiounit::prelude::*;

#[test]
fn au_audio_unit_bus_array_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let unit = AuAudioUnit::instantiate(
        support::default_output_description(),
        InstantiationOptions::InProcess,
    )?;
    let bus_array = unit.output_busses();
    let info = bus_array.info()?;
    assert!(info.count >= 1);
    assert!(bus_array.bus_at(0).is_some());
    if info.count_changeable {
        bus_array.set_bus_count(info.count)?;
    }
    Ok(())
}
