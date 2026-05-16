mod support;

use audiounit::prelude::*;

fn main() -> Result<(), AuError> {
    let unit = AuAudioUnit::instantiate(
        support::default_output_description(),
        InstantiationOptions::InProcess,
    )?;
    let bus_array = unit.output_busses();
    let info = bus_array.info()?;
    println!(
        "bus-array count={} changeable={} type={}",
        info.count, info.count_changeable, info.bus_type
    );
    if info.count_changeable {
        bus_array.set_bus_count(info.count)?;
    }
    for bus in info.busses {
        println!(
            "  bus {}: {} ch @ {} Hz",
            bus.index, bus.format.channel_count, bus.format.sample_rate
        );
    }
    Ok(())
}
