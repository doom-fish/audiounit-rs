mod support;

use audiounit::prelude::*;

#[test]
fn au_audio_unit_v2_bridge_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let unit = AuAudioUnit::instantiate(
        support::peak_limiter_description(),
        InstantiationOptions::InProcess,
    )?;
    let bridge = unit.as_v2_bridge().expect("expected v2 bridge");
    assert_ne!(bridge.audio_unit_ptr(), std::ptr::null_mut());
    Ok(())
}
