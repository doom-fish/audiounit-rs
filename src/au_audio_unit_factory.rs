//! `AUAudioUnitFactory` compatibility wrapper.

use core::ffi::c_void;

use crate::au_audio_unit::AuAudioUnit;
use crate::component_description::AudioComponentDescription;
use crate::error::AuError;
use crate::ffi;
use crate::util::status_result;

/// Bridge-backed helper that mirrors `AUAudioUnitFactory` creation semantics.
pub struct AuAudioUnitFactory {
    ptr: *mut c_void,
}

unsafe impl Send for AuAudioUnitFactory {}

impl Drop for AuAudioUnitFactory {
    fn drop(&mut self) {
        unsafe { ffi::au_factory_release(self.ptr) };
    }
}

impl AuAudioUnitFactory {
    /// Create a new bridge-backed factory instance.
    pub fn new() -> Self {
        Self {
            ptr: unsafe { ffi::au_factory_create() },
        }
    }

    /// Create an `AUAudioUnit` for the given description.
    pub fn create_audio_unit(
        &self,
        description: AudioComponentDescription,
    ) -> Result<AuAudioUnit, AuError> {
        let mut unit_ptr = core::ptr::null_mut();
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_factory_create_audio_unit(
                self.ptr,
                description.component_type,
                description.component_subtype,
                description.component_manufacturer,
                description.component_flags,
                description.component_flags_mask,
                &mut unit_ptr,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        Ok(AuAudioUnit::from_raw(unit_ptr))
    }
}

impl Default for AuAudioUnitFactory {
    fn default() -> Self {
        Self::new()
    }
}
