//! `AUParameterTree` wrappers.

use core::ffi::c_void;

use serde::Deserialize;

use crate::au_parameter::{AuParameter, AuParameterInfo};
use crate::au_parameter_group::AuParameterGroup;
use crate::error::AuError;
use crate::ffi;
use crate::util::{take_json, take_string};

/// Recursive snapshot of an `AUParameterNode` / tree node.
#[derive(Debug, Clone, Deserialize)]
pub struct AuParameterNodeInfo {
    pub identifier: String,
    #[serde(rename = "keyPath")]
    pub key_path: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub kind: String,
    #[serde(default)]
    pub children: Vec<Self>,
    #[serde(rename = "allParameters", default)]
    pub all_parameters: Vec<AuParameterInfo>,
    #[serde(rename = "minValue")]
    pub min_value: Option<f32>,
    #[serde(rename = "maxValue")]
    pub max_value: Option<f32>,
    pub unit: Option<u32>,
    pub address: Option<u64>,
    pub value: Option<f32>,
}

/// Owned handle to an `AUParameterTree`.
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

    /// Returns the full tree as JSON.
    pub fn to_json(&self) -> String {
        unsafe { take_string(ffi::au_parameter_tree_snapshot_json(self.ptr)).unwrap_or_default() }
    }

    /// Returns a typed snapshot of the tree.
    pub fn info(&self) -> Result<AuParameterNodeInfo, AuError> {
        let ptr = unsafe { ffi::au_parameter_tree_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Look up a parameter by address.
    pub fn parameter_with_address(&self, address: u64) -> Option<AuParameter> {
        let ptr = unsafe { ffi::au_parameter_tree_parameter_with_address(self.ptr, address) };
        if ptr.is_null() {
            None
        } else {
            Some(AuParameter::from_raw(ptr))
        }
    }

    /// Look up a v2 parameter by `(id, scope, element)`.
    pub fn parameter_with_id(
        &self,
        parameter_id: u32,
        scope: u32,
        element: u32,
    ) -> Option<AuParameter> {
        let ptr = unsafe {
            ffi::au_parameter_tree_parameter_with_id(self.ptr, parameter_id, scope, element)
        };
        if ptr.is_null() {
            None
        } else {
            Some(AuParameter::from_raw(ptr))
        }
    }

    /// Access the root group view of the tree.
    pub fn root_group(&self) -> AuParameterGroup {
        let ptr = unsafe { ffi::au_parameter_tree_root_group(self.ptr) };
        AuParameterGroup::from_raw(ptr)
    }
}
