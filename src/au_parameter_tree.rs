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

/// Opaque token returned by `AUParameterNode` observer registration methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuParameterObserverToken {
    raw: *mut c_void,
}

unsafe impl Send for AuParameterObserverToken {}
unsafe impl Sync for AuParameterObserverToken {}

/// One captured `AUParameterObserver` callback.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AuParameterValueEvent {
    pub address: u64,
    pub value: f32,
}

/// One captured `AURecordedParameterEvent`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AuRecordedParameterEventInfo {
    #[serde(rename = "hostTime")]
    pub host_time: u64,
    pub address: u64,
    pub value: f32,
}

/// One captured `AUParameterAutomationEvent`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AuParameterAutomationEventInfo {
    #[serde(rename = "hostTime")]
    pub host_time: u64,
    pub address: u64,
    pub value: f32,
    #[serde(rename = "eventType")]
    pub event_type: u32,
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

    pub(crate) fn retained(&self) -> Result<Self, AuError> {
        let ptr = unsafe { ffi::au_parameter_tree_retain(self.ptr) };
        if ptr.is_null() {
            Err(AuError::Unavailable(
                "AUParameterTree retain returned null".to_owned(),
            ))
        } else {
            Ok(Self { ptr })
        }
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

    /// Install a capture-backed value observer on the tree root.
    pub fn add_parameter_observer_capture(&self) -> Result<AuParameterObserverToken, AuError> {
        let token = unsafe { ffi::au_parameter_tree_add_parameter_observer_capture(self.ptr) };
        if token.is_null() {
            Err(AuError::Unavailable(
                "AUParameter observer token was null".to_owned(),
            ))
        } else {
            Ok(AuParameterObserverToken { raw: token })
        }
    }

    /// Install a capture-backed recording observer on the tree root.
    pub fn add_parameter_recording_observer_capture(
        &self,
    ) -> Result<AuParameterObserverToken, AuError> {
        let token =
            unsafe { ffi::au_parameter_tree_add_parameter_recording_observer_capture(self.ptr) };
        if token.is_null() {
            Err(AuError::Unavailable(
                "AUParameter recording observer token was null".to_owned(),
            ))
        } else {
            Ok(AuParameterObserverToken { raw: token })
        }
    }

    /// Install a capture-backed automation observer on the tree root.
    pub fn add_parameter_automation_observer_capture(
        &self,
    ) -> Result<AuParameterObserverToken, AuError> {
        let token =
            unsafe { ffi::au_parameter_tree_add_parameter_automation_observer_capture(self.ptr) };
        if token.is_null() {
            Err(AuError::Unavailable(
                "AUParameter automation observer token was null".to_owned(),
            ))
        } else {
            Ok(AuParameterObserverToken { raw: token })
        }
    }

    /// Drain the captured value-observer events for a token.
    pub fn take_parameter_observer_events(
        &self,
        token: AuParameterObserverToken,
    ) -> Result<Vec<AuParameterValueEvent>, AuError> {
        let ptr = unsafe {
            ffi::au_parameter_tree_take_parameter_observer_events_json(self.ptr, token.raw)
        };
        take_json(ptr)
    }

    /// Drain the captured recording-observer events for a token.
    pub fn take_parameter_recording_events(
        &self,
        token: AuParameterObserverToken,
    ) -> Result<Vec<AuRecordedParameterEventInfo>, AuError> {
        let ptr = unsafe {
            ffi::au_parameter_tree_take_parameter_recording_events_json(self.ptr, token.raw)
        };
        take_json(ptr)
    }

    /// Drain the captured automation-observer events for a token.
    pub fn take_parameter_automation_events(
        &self,
        token: AuParameterObserverToken,
    ) -> Result<Vec<AuParameterAutomationEventInfo>, AuError> {
        let ptr = unsafe {
            ffi::au_parameter_tree_take_parameter_automation_events_json(self.ptr, token.raw)
        };
        take_json(ptr)
    }

    /// Remove a previously registered tree observer token.
    pub fn remove_parameter_observer(&self, token: AuParameterObserverToken) {
        unsafe { ffi::au_parameter_tree_remove_parameter_observer(self.ptr, token.raw) };
    }

    /// Subscribe to async value-observer capture events.
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub fn parameter_observer_stream(
        &self,
        capacity: usize,
    ) -> Result<crate::async_api::AuParameterObserverStream, AuError> {
        crate::async_api::AuParameterObserverStream::subscribe(self, capacity)
    }

    /// Subscribe to async recording-observer capture events.
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub fn parameter_recording_stream(
        &self,
        capacity: usize,
    ) -> Result<crate::async_api::AuParameterRecordingStream, AuError> {
        crate::async_api::AuParameterRecordingStream::subscribe(self, capacity)
    }

    /// Subscribe to async automation-observer capture events.
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub fn parameter_automation_stream(
        &self,
        capacity: usize,
    ) -> Result<crate::async_api::AuParameterAutomationStream, AuError> {
        crate::async_api::AuParameterAutomationStream::subscribe(self, capacity)
    }
}
