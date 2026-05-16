mod support;

use audiounit::prelude::*;

#[test]
fn av_audio_unit_instrument_smoke() -> Result<(), Box<dyn std::error::Error>> {
    assert!(!audiounit::av_audio_unit_instrument::public_api_available());
    let instrument = AvAudioUnitInstrument::new(support::dls_synth_description())?;
    instrument.send_program_change(0, 0);
    instrument.start_note(60, 96, 0);
    instrument.stop_note(60, 0);
    assert!(!instrument.info()?.av_audio_unit.name.is_empty());
    Ok(())
}
