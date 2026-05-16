//! `AVAudioUnitComponentManager` and `AVAudioUnitComponent` wrappers.

use core::ffi::c_void;
use std::ffi::CStr;

use crate::component_description::AudioComponentDescription;
use crate::error::AuError;
use crate::ffi;

/// Metadata about a single audio plug-in component, obtained via
/// [`ComponentManager`].
pub struct AudioUnitComponent {
    ptr: *mut c_void,
}

unsafe impl Send for AudioUnitComponent {}
unsafe impl Sync for AudioUnitComponent {}

impl Drop for AudioUnitComponent {
    fn drop(&mut self) {
        unsafe { ffi::au_avc_component_release(self.ptr) };
    }
}

impl AudioUnitComponent {
    fn new(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    /// Display name of the component, e.g. `"AUPeakLimiter"`.
    pub fn name(&self) -> String {
        unsafe {
            let ptr = ffi::au_avc_component_name(self.ptr);
            take_string(ptr).unwrap_or_default()
        }
    }

    /// Human-readable type string, e.g. `"Effect"`.
    pub fn type_name(&self) -> String {
        unsafe {
            let ptr = ffi::au_avc_component_type_name(self.ptr);
            take_string(ptr).unwrap_or_default()
        }
    }

    /// Manufacturer display name, e.g. `"Apple"`.
    pub fn manufacturer_name(&self) -> String {
        unsafe {
            let ptr = ffi::au_avc_component_manufacturer_name(self.ptr);
            take_string(ptr).unwrap_or_default()
        }
    }

    /// Numeric version (packed BCD: `0x00MMmmpp`).
    pub fn version(&self) -> u32 {
        unsafe { ffi::au_avc_component_version(self.ptr) }
    }

    /// Human-readable version string, e.g. `"1.0.0"`.
    pub fn version_string(&self) -> String {
        unsafe {
            let ptr = ffi::au_avc_component_version_string(self.ptr);
            take_string(ptr).unwrap_or_default()
        }
    }

    /// Whether the component provides a custom view.
    pub fn has_custom_view(&self) -> bool {
        unsafe { ffi::au_avc_component_has_custom_view(self.ptr) }
    }

    /// Whether the component is sandbox-safe.
    pub fn is_sandbox_safe(&self) -> bool {
        unsafe { ffi::au_avc_component_sandbox_safe(self.ptr) }
    }

    /// The `AudioComponentDescription` identifying this component.
    pub fn audio_component_description(&self) -> AudioComponentDescription {
        let mut t: u32 = 0;
        let mut st: u32 = 0;
        let mut mfr: u32 = 0;
        let mut fl: u32 = 0;
        let mut flm: u32 = 0;
        unsafe {
            ffi::au_avc_component_audio_component_description(
                self.ptr, &mut t, &mut st, &mut mfr, &mut fl, &mut flm,
            );
        };
        AudioComponentDescription {
            component_type: t,
            component_subtype: st,
            component_manufacturer: mfr,
            component_flags: fl,
            component_flags_mask: flm,
        }
    }

    /// Tags associated with the component (always empty; tags property
    /// removed from `AVAudioUnitComponent` public API in recent SDK).
    pub fn tags(&self) -> Vec<String> {
        Vec::new()
    }
}

/// Access to `AVAudioUnitComponentManager.sharedAudioUnitComponentManager`.
pub struct ComponentManager;

impl ComponentManager {
    /// Enumerate all installed components matching `description`.
    /// Pass `AudioComponentDescription::any()` to list everything.
    ///
    /// # Errors
    /// Returns `AuError::Unknown` if the bridge returns a null without a count
    /// of zero.
    pub fn components_matching(
        description: AudioComponentDescription,
    ) -> Result<Vec<AudioUnitComponent>, AuError> {
        let d = description;
        let mut count: usize = 0;
        let buf = unsafe {
            ffi::au_avc_manager_components_matching(
                d.component_type,
                d.component_subtype,
                d.component_manufacturer,
                d.component_flags,
                d.component_flags_mask,
                &mut count,
            )
        };
        if count == 0 {
            return Ok(Vec::new());
        }
        if buf.is_null() {
            return Err(AuError::Unknown {
                code: -1,
                message: "au_avc_manager_components_matching returned null for count > 0"
                    .to_owned(),
            });
        }
        let components = (0..count)
            .map(|i| {
                let ptr = unsafe { *buf.add(i) };
                AudioUnitComponent::new(ptr)
            })
            .collect();
        // The array itself is freed here; individual component pointers were
        // already retained by the Swift side and wrapped above.
        unsafe { ffi::au_avc_component_array_free(buf, count) };
        Ok(components)
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

unsafe fn take_string(ptr: *mut core::ffi::c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    ffi::au_string_free(ptr);
    Some(s)
}
