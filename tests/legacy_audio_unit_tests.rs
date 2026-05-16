mod support;

use std::ptr;

use audiounit::legacy;
use audiounit::prelude::*;

fn status_err(label: &str, status: i32) -> String {
    format!("{label} failed with OSStatus {status}")
}

#[allow(clippy::missing_const_for_fn)]
unsafe extern "C" fn stream_format_listener(
    _ref_con: *mut core::ffi::c_void,
    _unit: *mut core::ffi::c_void,
    _property_id: u32,
    _scope: u32,
    _element: u32,
) {
}

#[test]
fn legacy_output_unit_lifecycle_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let unit = AuAudioUnit::instantiate(
        support::default_output_description(),
        InstantiationOptions::InProcess,
    )?;
    assert!(!unit.component_ptr().is_null());

    let raw_unit = unsafe {
        legacy::audio_component_instance_new(unit.component_ptr())
            .map_err(|status| status_err("AudioComponentInstanceNew", status))?
    };

    unsafe {
        let (size, _writable) = legacy::audio_unit_get_property_info(
            raw_unit,
            legacy::property_id::STREAM_FORMAT,
            legacy::scope::INPUT,
            0,
        )
        .map_err(|status| status_err("AudioUnitGetPropertyInfo", status))?;
        assert!(size > 0);

        legacy::audio_unit_add_property_listener(
            raw_unit,
            legacy::property_id::STREAM_FORMAT,
            stream_format_listener,
            ptr::null_mut(),
        )
        .map_err(|status| status_err("AudioUnitAddPropertyListener", status))?;

        legacy::audio_unit_remove_property_listener_with_user_data(
            raw_unit,
            legacy::property_id::STREAM_FORMAT,
            stream_format_listener,
            ptr::null_mut(),
        )
        .map_err(|status| status_err("AudioUnitRemovePropertyListenerWithUserData", status))?;

        legacy::audio_unit_initialize(raw_unit)
            .map_err(|status| status_err("AudioUnitInitialize", status))?;

        let events: [legacy::AudioUnitParameterEvent; 0] = [];
        legacy::audio_unit_schedule_parameters(raw_unit, events.as_ptr(), 0)
            .map_err(|status| status_err("AudioUnitScheduleParameters", status))?;

        legacy::audio_output_unit_start(raw_unit)
            .map_err(|status| status_err("AudioOutputUnitStart", status))?;
        legacy::audio_output_unit_stop(raw_unit)
            .map_err(|status| status_err("AudioOutputUnitStop", status))?;
        legacy::audio_unit_uninitialize(raw_unit)
            .map_err(|status| status_err("AudioUnitUninitialize", status))?;
        legacy::audio_component_instance_dispose(raw_unit)
            .map_err(|status| status_err("AudioComponentInstanceDispose", status))?;
    }

    Ok(())
}

#[test]
fn legacy_music_device_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let unit = AuAudioUnit::instantiate(
        support::dls_synth_description(),
        InstantiationOptions::InProcess,
    )?;
    let raw_unit = unsafe {
        legacy::audio_component_instance_new(unit.component_ptr())
            .map_err(|status| status_err("AudioComponentInstanceNew", status))?
    };

    unsafe {
        legacy::audio_unit_initialize(raw_unit)
            .map_err(|status| status_err("AudioUnitInitialize", status))?;
        legacy::music_device_midi_event(raw_unit, 0xC0, 0, 0, 0)
            .map_err(|status| status_err("MusicDeviceMIDIEvent", status))?;
        let note = legacy::music_device_start_note_std(
            raw_unit,
            legacy::music_note_event::UNUSED,
            0,
            0,
            60.0,
            96.0,
        )
        .map_err(|status| status_err("MusicDeviceStartNote", status))?;
        legacy::music_device_stop_note(raw_unit, 0, note, 0)
            .map_err(|status| status_err("MusicDeviceStopNote", status))?;
        legacy::music_device_sysex(raw_unit, &[0xF0, 0x7D, 0x01, 0xF7])
            .map_err(|status| status_err("MusicDeviceSysEx", status))?;
        legacy::audio_unit_uninitialize(raw_unit)
            .map_err(|status| status_err("AudioUnitUninitialize", status))?;
        legacy::audio_component_instance_dispose(raw_unit)
            .map_err(|status| status_err("AudioComponentInstanceDispose", status))?;
    }

    Ok(())
}
