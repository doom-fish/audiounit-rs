mod support;

use audiounit::prelude::*;

fn main() -> Result<(), AuError> {
    let unit = AuAudioUnit::instantiate(
        support::peak_limiter_description(),
        InstantiationOptions::InProcess,
    )?;
    let bridge = unit.as_v2_bridge().ok_or_else(|| {
        AuError::Unavailable("unit was not backed by AUAudioUnitV2Bridge".to_owned())
    })?;
    let info = bridge.info()?;
    println!("v2 bridge raw audio unit: 0x{:x}", info.audio_unit_pointer);
    Ok(())
}
