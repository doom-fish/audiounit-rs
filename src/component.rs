//! `AVAudioUnitComponentManager` and `AVAudioUnitComponent` wrappers.

use core::ffi::c_void;
use std::ffi::CStr;
use std::mem::ManuallyDrop;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

use serde::Serialize;
use serde_json::Value;

use crate::component_description::AudioComponentDescription;
use crate::error::AuError;
use crate::ffi;
use crate::util::{json_cstring, status_result, take_json};

/// Structured predicate DSL for
/// `AVAudioUnitComponentManager.componentsMatchingPredicate:`.
///
/// The Rust API intentionally avoids raw `NSPredicate` format strings so the
/// bridge stays memory-safe and cannot throw Objective-C exceptions on invalid
/// user input.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ComponentPredicate {
    /// Match all components.
    True,
    /// Match components whose display name contains `value`.
    NameContains { value: String },
    /// Match components whose type name contains `value`.
    TypeNameContains { value: String },
    /// Match components whose manufacturer name contains `value`.
    ManufacturerNameContains { value: String },
    /// Match components whose user tags contain `value`.
    UserTagContains { value: String },
    /// Match components whose user or system tags contain `value`.
    AllTagContains { value: String },
    /// Match components by custom-view support.
    HasCustomView { value: bool },
    /// Match components by sandbox-safety.
    SandboxSafe { value: bool },
    /// Logical AND of child predicates.
    All { predicates: Vec<Self> },
    /// Logical OR of child predicates.
    Any { predicates: Vec<Self> },
    /// Logical negation of a child predicate.
    Not { predicate: Box<Self> },
}

impl ComponentPredicate {
    /// Match all components.
    #[must_use]
    pub fn all_components() -> Self {
        Self::True
    }

    /// Match on the component's display name.
    #[must_use]
    pub fn name_contains(value: impl Into<String>) -> Self {
        Self::NameContains {
            value: value.into(),
        }
    }

    /// Match on the component's type name.
    #[must_use]
    pub fn type_name_contains(value: impl Into<String>) -> Self {
        Self::TypeNameContains {
            value: value.into(),
        }
    }

    /// Match on the component's manufacturer name.
    #[must_use]
    pub fn manufacturer_name_contains(value: impl Into<String>) -> Self {
        Self::ManufacturerNameContains {
            value: value.into(),
        }
    }

    /// Match on user-defined tag names.
    #[must_use]
    pub fn user_tag_contains(value: impl Into<String>) -> Self {
        Self::UserTagContains {
            value: value.into(),
        }
    }

    /// Match on any tag name, including system tags.
    #[must_use]
    pub fn all_tag_contains(value: impl Into<String>) -> Self {
        Self::AllTagContains {
            value: value.into(),
        }
    }

    /// Match on custom-view availability.
    #[must_use]
    pub const fn has_custom_view(value: bool) -> Self {
        Self::HasCustomView { value }
    }

    /// Match on sandbox safety.
    #[must_use]
    pub const fn sandbox_safe(value: bool) -> Self {
        Self::SandboxSafe { value }
    }

    /// Combine predicates with logical AND.
    #[must_use]
    pub fn all(predicates: Vec<Self>) -> Self {
        Self::All { predicates }
    }

    /// Combine predicates with logical OR.
    #[must_use]
    pub fn any(predicates: Vec<Self>) -> Self {
        Self::Any { predicates }
    }

    /// Negate a predicate.
    #[must_use]
    pub fn negate(predicate: Self) -> Self {
        Self::Not {
            predicate: Box::new(predicate),
        }
    }
}

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

    /// User-defined tag names.
    pub fn user_tag_names(&self) -> Result<Vec<String>, AuError> {
        let ptr = unsafe { ffi::au_avc_component_user_tag_names_json(self.ptr) };
        take_json(ptr)
    }

    /// Replace the component's user-defined tag names.
    pub fn set_user_tag_names<I, S>(&self, tags: I) -> Result<(), AuError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let tags = tags
            .into_iter()
            .map(|tag| tag.as_ref().to_owned())
            .collect::<Vec<_>>();
        let tags = json_cstring(&tags)?;
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_avc_component_set_user_tag_names_json(self.ptr, tags.as_ptr(), &mut error_ptr)
        };
        status_result(status, error_ptr)
    }

    /// All tag names visible to the current user, including system tags.
    pub fn all_tag_names(&self) -> Result<Vec<String>, AuError> {
        let ptr = unsafe { ffi::au_avc_component_all_tag_names_json(self.ptr) };
        take_json(ptr)
    }

    /// Available Mach-O architectures for the component.
    pub fn available_architectures(&self) -> Result<Vec<i64>, AuError> {
        let ptr = unsafe { ffi::au_avc_component_available_architectures_json(self.ptr) };
        take_json(ptr)
    }

    /// `configurationDictionary` converted to a JSON-compatible value.
    pub fn configuration_dictionary(&self) -> Result<Value, AuError> {
        let ptr = unsafe { ffi::au_avc_component_configuration_dictionary_json(self.ptr) };
        take_json(ptr)
    }

    /// Probe whether the component supports a specific input/output channel layout.
    pub fn supports_number_input_channels(
        &self,
        input_channels: isize,
        output_channels: isize,
    ) -> bool {
        unsafe {
            ffi::au_avc_component_supports_number_input_channels(
                self.ptr,
                input_channels,
                output_channels,
            )
        }
    }

    /// Compatibility helper returning all visible tags.
    pub fn tags(&self) -> Vec<String> {
        self.all_tag_names().unwrap_or_default()
    }
}

/// Access to `AVAudioUnitComponentManager.sharedAudioUnitComponentManager`.
pub struct ComponentManager;

impl ComponentManager {
    /// Enumerate all installed components matching `description`.
    /// Pass `AudioComponentDescription::any()` to list everything.
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
        components_from_buffer(
            buf,
            count,
            "au_avc_manager_components_matching returned null for count > 0",
        )
    }

    /// Enumerate components using a safe predicate wrapper around
    /// `componentsMatchingPredicate:`.
    pub fn components_matching_predicate(
        predicate: &ComponentPredicate,
    ) -> Result<Vec<AudioUnitComponent>, AuError> {
        let predicate = json_cstring(predicate)?;
        let mut buf = core::ptr::null_mut();
        let mut count: usize = 0;
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_avc_manager_components_matching_predicate(
                predicate.as_ptr(),
                &mut buf,
                &mut count,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        components_from_buffer(
            buf,
            count,
            "au_avc_manager_components_matching_predicate returned null for count > 0",
        )
    }

    /// Enumerate components using a safe closure wrapper around
    /// `componentsPassingTest:`.
    ///
    /// Set `*stop = true` inside the callback to stop the manager's search early.
    pub fn components_passing_test<F>(test: F) -> Result<Vec<AudioUnitComponent>, AuError>
    where
        F: FnMut(&AudioUnitComponent, &mut bool) -> bool,
    {
        let mut context = ComponentTestContext {
            callback: test,
            panic: None,
        };
        let mut buf = core::ptr::null_mut();
        let mut count: usize = 0;
        let mut error_ptr = core::ptr::null_mut();
        let status = unsafe {
            ffi::au_avc_manager_components_passing_test(
                Some(component_test_trampoline::<F>),
                std::ptr::addr_of_mut!(context).cast(),
                &mut buf,
                &mut count,
                &mut error_ptr,
            )
        };
        if let Some(payload) = context.panic.take() {
            resume_unwind(payload);
        }
        status_result(status, error_ptr)?;
        components_from_buffer(
            buf,
            count,
            "au_avc_manager_components_passing_test returned null for count > 0",
        )
    }
}

struct ComponentTestContext<F> {
    callback: F,
    panic: Option<Box<dyn std::any::Any + Send>>,
}

unsafe extern "C" fn component_test_trampoline<F>(
    component_ptr: *mut c_void,
    stop: *mut bool,
    context: *mut c_void,
) -> bool
where
    F: FnMut(&AudioUnitComponent, &mut bool) -> bool,
{
    let context = unsafe { &mut *context.cast::<ComponentTestContext<F>>() };
    match catch_unwind(AssertUnwindSafe(|| {
        let component = ManuallyDrop::new(AudioUnitComponent::new(component_ptr));
        let component_ref = unsafe { &*std::ptr::addr_of!(component).cast::<AudioUnitComponent>() };
        let mut should_stop = false;
        let matches = (context.callback)(component_ref, &mut should_stop);
        if !stop.is_null() {
            unsafe { *stop = should_stop };
        }
        matches
    })) {
        Ok(matches) => matches,
        Err(payload) => {
            context.panic = Some(payload);
            if !stop.is_null() {
                unsafe { *stop = true };
            }
            false
        }
    }
}

fn components_from_buffer(
    buf: *mut *mut c_void,
    count: usize,
    null_message: &str,
) -> Result<Vec<AudioUnitComponent>, AuError> {
    if count == 0 {
        if !buf.is_null() {
            unsafe { ffi::au_avc_component_array_free(buf, count) };
        }
        return Ok(Vec::new());
    }
    if buf.is_null() {
        return Err(AuError::Unknown {
            code: -1,
            message: null_message.to_owned(),
        });
    }
    let components = (0..count)
        .map(|index| {
            let ptr = unsafe { *buf.add(index) };
            AudioUnitComponent::new(ptr)
        })
        .collect();
    unsafe { ffi::au_avc_component_array_free(buf, count) };
    Ok(components)
}

unsafe fn take_string(ptr: *mut core::ffi::c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let s = unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    unsafe { ffi::au_string_free(ptr) };
    Some(s)
}
