//! `AVAudioUnit` instantiation and handles.

use core::ffi::c_void;

use crate::component_description::AudioComponentDescription;
use crate::error::{self, AuError};
use crate::ffi;
use crate::parameter::AuParameterTree;

/// Options for `AVAudioUnit` instantiation (mirrors `AudioComponentInstantiationOptions`).
#[derive(Debug, Clone, Copy, Default)]
#[repr(u32)]
pub enum InstantiationOptions {
    /// Instantiate in-process (default).
    #[default]
    InProcess = 0,
    /// Instantiate out-of-process (sandboxed).
    OutOfProcess = 1,
}

/// A live `AVAudioUnit` instance, wrapping both the modern `AUAudioUnit`
/// and the legacy `AudioUnit` handle.
pub struct AvAudioUnit {
    ptr: *mut c_void,
}

unsafe impl Send for AvAudioUnit {}

impl Drop for AvAudioUnit {
    fn drop(&mut self) {
        unsafe { ffi::au_avunit_release(self.ptr) };
    }
}

impl AvAudioUnit {
    /// Instantiate an `AVAudioUnit` for the given component description,
    /// blocking until the completion handler fires (up to 15 s).
    ///
    /// # Errors
    /// Returns an error if instantiation fails or times out.
    pub fn instantiate(
        description: AudioComponentDescription,
        options: InstantiationOptions,
    ) -> Result<Self, AuError> {
        let d = description;
        let mut unit_ptr: *mut c_void = core::ptr::null_mut();
        let mut err_ptr: *mut core::ffi::c_char = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_instantiate_sync(
                d.component_type,
                d.component_subtype,
                d.component_manufacturer,
                d.component_flags,
                d.component_flags_mask,
                options as u32,
                &mut unit_ptr,
                &mut err_ptr,
            )
        };
        if status != ffi::status::OK {
            return Err(error::from_status(status, err_ptr));
        }
        Ok(Self { ptr: unit_ptr })
    }

    /// The legacy `AudioUnit` opaque pointer.
    ///
    /// # Safety
    /// The returned pointer is valid only while this `AvAudioUnit` is alive.
    /// It is not reference-counted — do not retain or release it.
    pub fn audio_unit_ptr(&self) -> *mut c_void {
        unsafe { ffi::au_avunit_audio_unit(self.ptr) }
    }

    /// Returns a reference to the underlying `AUAudioUnit` (modern API).
    /// The returned handle is separately reference-counted.
    pub fn au_audio_unit(&self) -> AuAudioUnitHandle {
        let inner = unsafe { ffi::au_avunit_auaudiounit(self.ptr) };
        AuAudioUnitHandle { ptr: inner }
    }

    /// Returns the `AUParameterTree` for this unit, if it has one.
    pub fn parameter_tree(&self) -> Option<AuParameterTree> {
        let au = self.au_audio_unit();
        au.parameter_tree()
    }
}

/// A handle to the modern `AUAudioUnit` interface.
pub struct AuAudioUnitHandle {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AuAudioUnitHandle {}

impl Drop for AuAudioUnitHandle {
    fn drop(&mut self) {
        unsafe { ffi::au_auaudiounit_release(self.ptr) };
    }
}

impl AuAudioUnitHandle {
    /// Returns the parameter tree, or `None` if the unit has no parameters.
    pub fn parameter_tree(&self) -> Option<AuParameterTree> {
        let raw = unsafe { ffi::au_auaudiounit_parameter_tree(self.ptr) };
        if raw.is_null() {
            None
        } else {
            Some(AuParameterTree::from_raw(raw))
        }
    }
}
