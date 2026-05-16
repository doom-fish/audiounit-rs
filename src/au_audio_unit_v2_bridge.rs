//! `AUAudioUnitV2Bridge` wrappers.

use core::ffi::c_void;

use serde::Deserialize;

use crate::au_audio_unit::AuAudioUnit;
use crate::component_description::AudioComponentDescription;
use crate::ffi;
use crate::util::take_json;

/// Snapshot of an `AUAudioUnitV2Bridge`.
#[derive(Debug, Clone, Deserialize)]
pub struct AuAudioUnitV2BridgeInfo {
    #[serde(rename = "audioUnitPointer")]
    pub audio_unit_pointer: usize,
    #[serde(rename = "componentDescription")]
    pub component_description: AudioComponentDescription,
}

/// Owned handle to an `AUAudioUnitV2Bridge`.
pub struct AuAudioUnitV2Bridge {
    ptr: *mut c_void,
}

unsafe impl Send for AuAudioUnitV2Bridge {}

impl Drop for AuAudioUnitV2Bridge {
    fn drop(&mut self) {
        unsafe { ffi::au_v2_bridge_release(self.ptr) };
    }
}

impl AuAudioUnitV2Bridge {
    pub(crate) fn from_au_audio_unit(unit: &AuAudioUnit) -> Option<Self> {
        let ptr = unsafe { ffi::au_v2_bridge_from_auaudiounit(unit.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Snapshot the bridge metadata.
    pub fn info(&self) -> Result<AuAudioUnitV2BridgeInfo, crate::error::AuError> {
        let ptr = unsafe { ffi::au_v2_bridge_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Access the underlying v2 `AudioUnit` pointer.
    pub fn audio_unit_ptr(&self) -> *mut c_void {
        unsafe { ffi::au_v2_bridge_audio_unit(self.ptr) }
    }
}
