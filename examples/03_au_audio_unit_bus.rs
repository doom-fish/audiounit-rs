mod support;

use audiounit::prelude::*;

fn main() -> Result<(), AuError> {
    let unit = AuAudioUnit::instantiate(
        support::default_output_description(),
        InstantiationOptions::InProcess,
    )?;
    let bus_array = unit.output_busses();
    let bus = bus_array
        .bus_at(0)
        .ok_or_else(|| AuError::Unavailable("default output had no output bus".to_owned()))?;
    let info = bus.info()?;
    println!(
        "output bus {} sample-rate={} channels={}",
        info.index, info.format.sample_rate, info.format.channel_count
    );
    bus.set_should_allocate_buffer(info.should_allocate_buffer);
    bus.set_enabled(info.enabled);
    bus.set_context_presentation_latency(info.context_presentation_latency);
    Ok(())
}
