mod support;

use audiounit::prelude::*;

#[test]
fn au_audio_unit_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let unit = AuAudioUnit::instantiate(
        support::default_output_description(),
        InstantiationOptions::InProcess,
    )?;
    let info = unit.info()?;
    assert!(info.output_bus_count >= 1);
    assert!(info.parameter_tree_available);
    let _ = unit.parameters_for_overview(8)?;
    Ok(())
}
