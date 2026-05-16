//! `AUParameterTree`, `AUParameterNode`, and `AUParameter` wrappers.

use core::ffi::c_void;
use std::ffi::CStr;

use crate::ffi;

/// A live reference to an `AUParameterTree`.
pub struct AuParameterTree {
    ptr: *mut c_void,
}

unsafe impl Send for AuParameterTree {}
unsafe impl Sync for AuParameterTree {}

impl Drop for AuParameterTree {
    fn drop(&mut self) {
        unsafe { ffi::au_parameter_tree_release(self.ptr) };
    }
}

impl AuParameterTree {
    pub(crate) fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    /// Returns a JSON description of the full parameter tree.
    pub fn to_json(&self) -> String {
        let ptr = unsafe { ffi::au_parameter_tree_json(self.ptr) };
        if ptr.is_null() {
            return "{}".to_owned();
        }
        unsafe {
            let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            ffi::au_string_free(ptr);
            s
        }
    }

    /// Look up a parameter by its address.
    pub fn parameter_with_address(&self, address: u64) -> Option<AuParameter> {
        let ptr =
            unsafe { ffi::au_parameter_tree_parameter_with_address(self.ptr, address) };
        if ptr.is_null() {
            None
        } else {
            Some(AuParameter { ptr })
        }
    }
}

/// A single `AUParameter` within a tree.
pub struct AuParameter {
    ptr: *mut c_void,
}

unsafe impl Send for AuParameter {}
unsafe impl Sync for AuParameter {}

impl Drop for AuParameter {
    fn drop(&mut self) {
        unsafe { ffi::au_parameter_release(self.ptr) };
    }
}

impl AuParameter {
    /// Short identifier string.
    pub fn identifier(&self) -> String {
        unsafe { take_string(ffi::au_parameter_identifier(self.ptr)).unwrap_or_default() }
    }

    /// Localized display name.
    pub fn display_name(&self) -> String {
        unsafe { take_string(ffi::au_parameter_display_name(self.ptr)).unwrap_or_default() }
    }

    /// Address used to identify the parameter within its tree.
    pub fn address(&self) -> u64 {
        unsafe { ffi::au_parameter_address(self.ptr) }
    }

    /// Minimum value.
    pub fn min_value(&self) -> f32 {
        unsafe { ffi::au_parameter_min_value(self.ptr) }
    }

    /// Maximum value.
    pub fn max_value(&self) -> f32 {
        unsafe { ffi::au_parameter_max_value(self.ptr) }
    }

    /// Raw `AUParameterUnit` enum value.
    pub fn unit(&self) -> u32 {
        unsafe { ffi::au_parameter_unit(self.ptr) }
    }

    /// Current value.
    pub fn value(&self) -> f32 {
        unsafe { ffi::au_parameter_get_value(self.ptr) }
    }

    /// Set current value.
    pub fn set_value(&self, value: f32) {
        unsafe { ffi::au_parameter_set_value(self.ptr, value) };
    }

    /// Localized string representation of `value`.
    pub fn string_from_value(&self, value: f32) -> String {
        unsafe {
            take_string(ffi::au_parameter_string_from_value(self.ptr, value)).unwrap_or_default()
        }
    }
}

/// A node in the `AUParameterTree` — either a group or a leaf parameter.
/// Use `AuParameterTree::to_json()` for full tree traversal; this type
/// is a placeholder for future typed walking.
pub struct AuParameterNode;

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
