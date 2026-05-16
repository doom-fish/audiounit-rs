mod support;

use audiounit::prelude::*;

fn main() -> Result<(), AuError> {
    let unit = AuAudioUnit::instantiate(
        support::default_output_description(),
        InstantiationOptions::InProcess,
    )?;
    let info = unit.info()?;
    println!(
        "AUAudioUnit: {} ({})",
        info.audio_unit_name
            .unwrap_or_else(|| "<unnamed>".to_owned()),
        support::fourcc_to_string(info.component_description.component_subtype)
    );
    println!("input busses: {}", info.input_bus_count);
    println!("output busses: {}", info.output_bus_count);
    println!(
        "parameter tree available: {}",
        info.parameter_tree_available
    );
    Ok(())
}
