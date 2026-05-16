mod support;

use audiounit::prelude::*;

fn main() -> Result<(), AuError> {
    let unit = AuAudioUnit::instantiate(
        support::default_output_description(),
        InstantiationOptions::InProcess,
    )?;
    let parameter = support::first_parameter(&unit)?;
    let value = parameter.value();
    println!("parameter {} = {} ({})", parameter.identifier(), value, parameter.string_from_value(value));
    parameter.set_value(value);
    parameter.set_value_at_host_time(value, 0);
    println!("short name: {}", parameter.display_name_with_length(8));
    Ok(())
}
