mod support;

use audiounit::prelude::*;

fn main() -> Result<(), AuError> {
    let instrument = AvAudioUnitMidiInstrument::new(support::dls_synth_description())?;
    let info = instrument.info()?;
    println!(
        "midi instrument {} event-list={}",
        info.av_audio_unit.name, info.supports_midi_event_list
    );
    instrument.send_controller(7, 100, 0);
    instrument.send_pitch_bend(0x2000, 0);
    instrument.start_note(64, 100, 0);
    instrument.stop_note(64, 0);
    instrument.send_sysex(&[0xF0, 0x7D, 0x01, 0xF7]);
    Ok(())
}
