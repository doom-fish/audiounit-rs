#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use audiounit::prelude::*;

pub const fn default_output_description() -> AudioComponentDescription {
    AudioComponentDescription::apple(AUDIO_UNIT_TYPE_OUTPUT, AUDIO_UNIT_SUBTYPE_DEFAULT_OUTPUT)
}

pub const fn peak_limiter_description() -> AudioComponentDescription {
    AudioComponentDescription::apple(AUDIO_UNIT_TYPE_EFFECT, AUDIO_UNIT_SUBTYPE_PEAK_LIMITER)
}

pub const fn speech_synthesis_description() -> AudioComponentDescription {
    AudioComponentDescription::apple(
        AUDIO_UNIT_TYPE_GENERATOR,
        AUDIO_UNIT_SUBTYPE_SPEECH_SYNTHESIS,
    )
}

pub const fn dls_synth_description() -> AudioComponentDescription {
    AudioComponentDescription::apple(AUDIO_UNIT_TYPE_MUSIC_DEVICE, AUDIO_UNIT_SUBTYPE_DLS_SYNTH)
}

pub const fn varispeed_description() -> AudioComponentDescription {
    AudioComponentDescription::apple(
        AUDIO_UNIT_TYPE_FORMAT_CONVERTER,
        AUDIO_UNIT_SUBTYPE_VARISPEED,
    )
}

pub const fn new_time_pitch_description() -> AudioComponentDescription {
    AudioComponentDescription::apple(
        AUDIO_UNIT_TYPE_FORMAT_CONVERTER,
        AUDIO_UNIT_SUBTYPE_NEW_TIME_PITCH,
    )
}

pub const SYSTEM_SOUND_BANK_PATH: &str =
    "/System/Library/Components/CoreAudio.component/Contents/Resources/gs_instruments.dls";

pub fn system_sound_bank_path() -> &'static Path {
    Path::new(SYSTEM_SOUND_BANK_PATH)
}

pub fn write_full_state_preset(
    unit: &AvAudioUnit,
    name: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let plist = unit
        .au_audio_unit()
        .info()?
        .full_state_plist
        .ok_or_else(|| AuError::Unavailable("unit did not expose fullState".to_owned()))?;
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("test-artifacts");
    fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{name}-{}.aupreset", std::process::id()));
    fs::write(&path, plist)?;
    Ok(path)
}

pub fn system_aupreset_candidates(
) -> Result<Vec<(PathBuf, AudioComponentDescription)>, Box<dyn std::error::Error>> {
    let presets =
        find_aupresets("/System/Library/Components/AudioDSP.component/Contents/Resources/Tunings")
            .or_else(|_| find_aupresets("/System/Library/Components"))?;
    let mut candidates = Vec::with_capacity(presets.len());
    for preset in presets {
        let description = AudioComponentDescription::with_flags(
            plutil_extract_u32(&preset, "type")?,
            plutil_extract_u32(&preset, "subtype")?,
            plutil_extract_u32(&preset, "manufacturer")?,
            0,
            0,
        );
        candidates.push((preset, description));
    }
    Ok(candidates)
}

fn find_aupresets(root: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let output = Command::new("find")
        .args([root, "-name", "*.aupreset"])
        .output()?;
    if !output.status.success() {
        return Err(format!("find failed for {root}").into());
    }
    let paths = String::from_utf8(output.stdout)?
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(PathBuf::from)
        .collect::<Vec<_>>();
    if paths.is_empty() {
        return Err(format!("no .aupreset files found under {root}").into());
    }
    Ok(paths)
}

fn plutil_extract_u32(path: &Path, key: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let path_str = path
        .to_str()
        .ok_or_else(|| format!("preset path was not valid UTF-8: {}", path.display()))?;
    let output = Command::new("plutil")
        .args(["-extract", key, "raw", "-o", "-", path_str])
        .output()?;
    if !output.status.success() {
        return Err(format!("plutil failed extracting {key} from {path_str}").into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().parse()?)
}

pub fn first_parameter(unit: &AuAudioUnit) -> Result<AuParameter, AuError> {
    let tree = unit
        .parameter_tree()
        .ok_or_else(|| AuError::Unavailable("unit has no parameter tree".to_owned()))?;
    let group = tree.root_group();
    let parameters = group.all_parameters()?;
    let address = parameters
        .first()
        .ok_or_else(|| AuError::Unavailable("parameter tree was empty".to_owned()))?
        .address;
    tree.parameter_with_address(address).ok_or_else(|| {
        AuError::Unavailable("failed to resolve first parameter by address".to_owned())
    })
}

pub const fn ducking_level_from_raw(raw: u32) -> AuVoiceIoOtherAudioDuckingLevel {
    match raw {
        10 => AuVoiceIoOtherAudioDuckingLevel::Min,
        20 => AuVoiceIoOtherAudioDuckingLevel::Mid,
        30 => AuVoiceIoOtherAudioDuckingLevel::Max,
        _ => AuVoiceIoOtherAudioDuckingLevel::Default,
    }
}
