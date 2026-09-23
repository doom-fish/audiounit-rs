mod support;

use std::time::Duration;

use audiounit::prelude::*;

#[test]
fn au_audio_unit_timeout_is_configurable() {
    let zero = AuAudioUnit::instantiate_with_timeout(
        support::peak_limiter_description(),
        InstantiationOptions::InProcess,
        Duration::ZERO,
    );
    assert!(matches!(zero, Err(AuError::InvalidArgument(_))));
    assert!(AuAudioUnit::instantiate_with_timeout(
        support::peak_limiter_description(),
        InstantiationOptions::InProcess,
        Duration::from_secs(30),
    )
    .is_ok());
    assert!(AuAudioUnit::instantiate_with_timeout(
        support::peak_limiter_description(),
        InstantiationOptions::InProcess,
        Duration::MAX,
    )
    .is_ok());
}

#[test]
fn av_audio_unit_timeout_is_configurable() {
    let zero = AvAudioUnit::instantiate_with_timeout(
        support::peak_limiter_description(),
        InstantiationOptions::InProcess,
        Duration::ZERO,
    );
    assert!(matches!(zero, Err(AuError::InvalidArgument(_))));
    assert!(AvAudioUnit::instantiate_with_timeout(
        support::peak_limiter_description(),
        InstantiationOptions::InProcess,
        Duration::from_secs(30),
    )
    .is_ok());
}

#[test]
fn voice_io_rejects_a_zero_timeout() {
    assert!(matches!(
        AuVoiceIo::new_with_timeout(InstantiationOptions::InProcess, Duration::ZERO),
        Err(AuError::InvalidArgument(_))
    ));
}

#[test]
fn default_timeout_is_fifteen_seconds() {
    assert_eq!(DEFAULT_INSTANTIATION_TIMEOUT, Duration::from_secs(15));
}
