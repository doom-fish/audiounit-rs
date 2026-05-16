mod support;

use audiounit::prelude::*;

#[test]
fn au_parameter_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let unit = AuAudioUnit::instantiate(
        support::default_output_description(),
        InstantiationOptions::InProcess,
    )?;
    let parameter = support::first_parameter(&unit)?;
    let value = parameter.value();
    assert!(!parameter.identifier().is_empty());
    let _ = parameter.display_name_with_length(8);
    parameter.set_value(value);
    parameter.set_value_at_host_time(value, 0);
    let _ = parameter.string_from_value(value);
    Ok(())
}
