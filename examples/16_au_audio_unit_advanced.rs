mod support;

use audiounit::legacy;
use audiounit::prelude::*;

fn status_err(label: &str, status: i32) -> Box<dyn std::error::Error> {
    format!("{label} failed with OSStatus {status}").into()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let unit = AuAudioUnit::instantiate(
        support::default_output_description(),
        InstantiationOptions::InProcess,
    )?;
    println!("AUAudioUnit component pointer: {:?}", unit.component_ptr());
    println!(
        "I/O capabilities: input={} output={}",
        unit.can_perform_input(),
        unit.can_perform_output()
    );

    unit.set_musical_context(Some(&AuHostMusicalContext {
        current_tempo: Some(120.0),
        time_signature_numerator: Some(4.0),
        time_signature_denominator: Some(4),
        current_beat_position: Some(32.0),
        sample_offset_to_next_beat: Some(0),
        current_measure_downbeat_position: Some(29.0),
    }))?;
    println!("musical context: {:?}", unit.musical_context()?);

    let tree = unit.parameter_tree().ok_or("missing parameter tree")?;
    let token = tree.add_parameter_observer_capture()?;
    let parameter = support::first_parameter(&unit)?;
    let value = parameter.value();
    parameter.set_value_with_event(value, 0, AuParameterAutomationEventType::Value);
    let events = tree.take_parameter_observer_events(token)?;
    println!("captured parameter events: {}", events.len());
    tree.remove_parameter_observer(token);

    let raw_unit = unsafe {
        legacy::audio_component_instance_new(unit.component_ptr())
            .map_err(|status| status_err("AudioComponentInstanceNew", status))?
    };
    unsafe {
        legacy::audio_unit_initialize(raw_unit)
            .map_err(|status| status_err("AudioUnitInitialize", status))?;
        legacy::audio_output_unit_start(raw_unit)
            .map_err(|status| status_err("AudioOutputUnitStart", status))?;
        legacy::audio_output_unit_stop(raw_unit)
            .map_err(|status| status_err("AudioOutputUnitStop", status))?;
        legacy::audio_unit_uninitialize(raw_unit)
            .map_err(|status| status_err("AudioUnitUninitialize", status))?;
        legacy::audio_component_instance_dispose(raw_unit)
            .map_err(|status| status_err("AudioComponentInstanceDispose", status))?;
    }
    println!("legacy raw component lifecycle ok");
    Ok(())
}
