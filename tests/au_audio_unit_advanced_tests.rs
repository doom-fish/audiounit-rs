mod support;

use std::time::{Duration, Instant};

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

    let render_token = unit.add_render_observer_capture(16)?;
    assert_eq!(
        render_token.take_events(),
        [] as [audiounit::AuRenderObserverEvent; 0]
    );
    drop(render_token);
    assert!(unit.add_render_observer_capture(0).is_err());

    let first = unit.add_midi_output_capture(16)?;
    let second = unit.add_midi_output_capture(16)?;
    assert_eq!(first.take_events(), [] as [audiounit::AuMidiOutputEvent; 0]);
    drop(first);
    assert_eq!(
        second.take_events(),
        [] as [audiounit::AuMidiOutputEvent; 0]
    );
    assert_eq!(second.dropped_events(), 0);
    drop(second);

    let event_lists = unit.add_midi_output_event_list_capture(16)?;
    assert_eq!(
        event_lists.take_events(),
        [] as [audiounit::AuMidiOutputEventListSummary; 0]
    );
    drop(event_lists);

    Ok(())
}

#[test]
fn au_parameter_observer_capture_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let unit = AuAudioUnit::instantiate(
        support::default_output_description(),
        InstantiationOptions::InProcess,
    )?;
    let tree = unit.parameter_tree().expect("missing parameter tree");
    let value_token = tree.add_parameter_observer_capture(64)?;
    let recording_token = tree.add_parameter_recording_observer_capture(64)?;
    let automation_token = tree.add_parameter_automation_observer_capture(64)?;
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

    let mut values = Vec::new();
    let mut recordings = Vec::new();
    let mut automations = Vec::new();
    let deadline = Instant::now() + Duration::from_secs(5);
    while (values.is_empty() || recordings.is_empty() || automations.is_empty())
        && Instant::now() < deadline
    {
        values.extend(value_token.take_events());
        recordings.extend(recording_token.take_events());
        automations.extend(automation_token.take_events());
        std::thread::sleep(Duration::from_millis(5));
    }

    assert_ne!(values, [] as [audiounit::AuParameterValueEvent; 0]);
    assert_ne!(
        recordings,
        [] as [audiounit::AuRecordedParameterEventInfo; 0]
    );
    assert_ne!(
        automations,
        [] as [audiounit::AuParameterAutomationEventInfo; 0]
    );
    assert!(values.iter().all(
        |event| event.address == info.address && (event.value - new_value).abs() < f32::EPSILON
    ));
    assert!(recordings.iter().all(
        |event| event.address == info.address && (event.value - new_value).abs() < f32::EPSILON
    ));
    assert!(automations.iter().all(
        |event| event.address == info.address && (event.value - new_value).abs() < f32::EPSILON
    ));
    assert_eq!(value_token.dropped_events(), 0);
    assert!(tree.add_parameter_observer_capture(0).is_err());
    assert!(tree
        .add_parameter_observer_capture(CAPTURE_CAPACITY_MAX + 1)
        .is_err());

    drop(value_token);
    drop(recording_token);
    drop(automation_token);
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
