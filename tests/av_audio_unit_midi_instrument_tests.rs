mod support;

use audiounit::prelude::*;

#[test]
fn av_audio_unit_midi_instrument_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let instrument = AvAudioUnitMidiInstrument::new(support::dls_synth_description())?;
    instrument.send_controller(7, 100, 0);
    instrument.send_pitch_bend(0x2000, 0);
    instrument.send_pressure(64, 0);
    instrument.send_pressure_for_key(60, 64, 0);
    instrument.send_program_change_bank(0, 0, 0, 0);
    instrument.send_midi_event2(0x90, 60, 100);
    instrument.send_sysex(&[0xF0, 0x7D, 0x02, 0xF7]);
    assert!(instrument.info()?.supports_midi_event_list);
    Ok(())
}
