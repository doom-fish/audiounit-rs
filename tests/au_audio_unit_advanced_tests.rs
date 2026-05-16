mod support;

use audiounit::prelude::*;

#[test]
fn au_audio_unit_host_context_and_capture_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let unit = AuAudioUnit::instantiate(
        support::default_output_description(),
        InstantiationOptions::InProcess,
    )?;

    assert!(!unit.component_ptr().is_null());
    assert!(!unit.can_perform_input());
    assert!(unit.can_perform_output());

    unit.set_musical_context(Some(&AuHostMusicalContext {
        current_tempo: Some(132.0),
        time_signature_numerator: Some(4.0),
        time_signature_denominator: Some(4),
        current_beat_position: Some(16.0),
        sample_offset_to_next_beat: Some(64),
        current_measure_downbeat_position: Some(13.0),
    }))?;
    let musical_context = unit.musical_context()?.expect("missing musical context");
    assert_eq!(musical_context.current_tempo, Some(132.0));
    assert_eq!(musical_context.time_signature_denominator, Some(4));

    unit.set_transport_state(Some(&AuHostTransportState {
        transport_state_flags: host_transport_state_flags::MOVING
            | host_transport_state_flags::CHANGED,
        current_sample_position: Some(512.0),
        cycle_start_beat_position: Some(8.0),
        cycle_end_beat_position: Some(12.0),
    }))?;
    let transport_state = unit.transport_state()?.expect("missing transport state");
    assert_eq!(
        transport_state.transport_state_flags,
        host_transport_state_flags::MOVING | host_transport_state_flags::CHANGED
    );

    let render_token = unit.add_render_observer_capture()?;
    assert!(unit.take_render_observer_events(render_token)?.is_empty());
    unit.remove_render_observer(render_token);

    unit.set_midi_output_event_capture_enabled(true);
    assert!(unit.take_captured_midi_output_events()?.is_empty());
    unit.set_midi_output_event_capture_enabled(false);

    unit.set_midi_output_event_list_capture_enabled(true);
    assert!(unit.take_captured_midi_output_event_lists()?.is_empty());
    unit.set_midi_output_event_list_capture_enabled(false);

    Ok(())
}

#[test]
fn au_parameter_observer_capture_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let unit = AuAudioUnit::instantiate(
        support::default_output_description(),
        InstantiationOptions::InProcess,
    )?;
    let tree = unit.parameter_tree().expect("missing parameter tree");
    let value_token = tree.add_parameter_observer_capture()?;
    let recording_token = tree.add_parameter_recording_observer_capture()?;
    let automation_token = tree.add_parameter_automation_observer_capture()?;
    let parameter = support::first_parameter(&unit)?;
    let info = parameter.info()?;
    let current_value = parameter.value();
    let new_value = if (current_value - info.max_value).abs() > f32::EPSILON {
        info.max_value
    } else {
        info.min_value
    };
    parameter.set_value(new_value);
    parameter.set_value_with_event(new_value, 0, AuParameterAutomationEventType::Value);

    let _ = tree.take_parameter_observer_events(value_token)?;
    let _ = tree.take_parameter_recording_events(recording_token)?;
    let _ = tree.take_parameter_automation_events(automation_token)?;

    tree.remove_parameter_observer(value_token);
    tree.remove_parameter_observer(recording_token);
    tree.remove_parameter_observer(automation_token);
    Ok(())
}

#[test]
fn au_audio_unit_music_device_schedule_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let unit = AuAudioUnit::instantiate(
        support::dls_synth_description(),
        InstantiationOptions::InProcess,
    )?;
    unit.allocate_render_resources()?;
    unit.schedule_midi_event(AU_EVENT_SAMPLE_TIME_IMMEDIATE, 0, &[0x90, 60, 100])?;
    let _ = unit.profile_state_for_cable_channel(0, 0)?;
    let _ = unit.message_channel("rust.missing-channel")?;
    unit.deallocate_render_resources();
    Ok(())
}
