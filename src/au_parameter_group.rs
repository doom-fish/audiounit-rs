//! `AUParameterGroup` wrappers.

use core::ffi::c_void;

use crate::au_parameter::AuParameterInfo;
use crate::au_parameter_tree::AuParameterNodeInfo;
use crate::error::AuError;
use crate::ffi;
use crate::util::take_json;

/// Typed snapshot of an `AUParameterGroup`.
pub type AuParameterGroupInfo = AuParameterNodeInfo;

/// Owned handle to an `AUParameterGroup`.
pub struct AuParameterGroup {
    ptr: *mut c_void,
}

unsafe impl Send for AuParameterGroup {}
unsafe impl Sync for AuParameterGroup {}

impl Drop for AuParameterGroup {
    fn drop(&mut self) {
        unsafe { ffi::au_parameter_group_release(self.ptr) };
    }
}

impl AuParameterGroup {
    pub(crate) fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    /// Snapshot the group metadata.
    pub fn info(&self) -> Result<AuParameterGroupInfo, AuError> {
        let ptr = unsafe { ffi::au_parameter_group_snapshot_json(self.ptr) };
        take_json(ptr)
    }

    /// Snapshot the group's direct children.
    pub fn children(&self) -> Result<Vec<AuParameterNodeInfo>, AuError> {
        let ptr = unsafe { ffi::au_parameter_group_children_json(self.ptr) };
        take_json(ptr)
    }

    /// Snapshot all parameters contained by the group.
    pub fn all_parameters(&self) -> Result<Vec<AuParameterInfo>, AuError> {
        let ptr = unsafe { ffi::au_parameter_group_all_parameters_json(self.ptr) };
        take_json(ptr)
    }
}
