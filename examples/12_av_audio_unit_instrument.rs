mod support;

use audiounit::prelude::*;

fn main() -> Result<(), AuError> {
    println!(
        "public AVAudioUnitInstrument API available: {}",
        audiounit::av_audio_unit_instrument::public_api_available()
    );
    let instrument = AvAudioUnitInstrument::new(support::dls_synth_description())?;
    let info = instrument.info()?;
    println!("instrument {}", info.av_audio_unit.name);
    instrument.send_program_change(0, 0);
    instrument.start_note(60, 96, 0);
    instrument.stop_note(60, 0);
    Ok(())
}
