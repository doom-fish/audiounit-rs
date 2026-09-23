//! `AUParameterTree` wrappers.

use core::ffi::c_void;

use serde::Deserialize;

use crate::au_parameter::{AuParameter, AuParameterInfo};
use crate::au_parameter_group::AuParameterGroup;
use crate::capture::{
    push_parameter_automation, push_parameter_value, push_recorded_parameter, Capture,
    CaptureContext,
};
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
pub struct AuParameterObserverToken<T: Send + 'static> {
    tree: AuParameterTree,
    raw: *mut c_void,
    capture: Capture<T>,
}

unsafe impl<T: Send + 'static> Send for AuParameterObserverToken<T> {}

impl<T: Send + 'static> AuParameterObserverToken<T> {
    pub fn take_events(&self) -> Vec<T> {
        self.capture.drain()
    }

    pub fn dropped_events(&self) -> u64 {
        self.capture.dropped()
    }
}

impl<T: Send + 'static> Drop for AuParameterObserverToken<T> {
    fn drop(&mut self) {
        self.capture.close();
        unsafe { ffi::au_parameter_tree_remove_parameter_observer(self.tree.ptr, self.raw) };
    }
}

impl<T: Send + 'static> std::fmt::Debug for AuParameterObserverToken<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuParameterObserverToken")
            .field("raw", &self.raw)
            .field("capacity", &self.capture.capacity())
            .field("dropped_events", &self.dropped_events())
            .finish_non_exhaustive()
    }
}

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
    pub(crate) ptr: *mut c_void,
}

crate::handle::handle_type!(
    AuParameterTree,
    field = ptr,
    release = ffi::au_parameter_tree_release,
    sync
);

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

    fn add_observer<T: Send + 'static>(
        &self,
        capacity: usize,
        register: impl FnOnce(*mut c_void, *mut c_void) -> *mut c_void,
        what: &str,
    ) -> Result<AuParameterObserverToken<T>, AuError> {
        let capture = Capture::new(capacity)?;
        let tree = self.retained()?;
        let raw = register(tree.ptr, capture.retained_context());
        if raw.is_null() {
            return Err(AuError::Unavailable(format!(
                "AUParameter {what} token was null"
            )));
        }
        Ok(AuParameterObserverToken { tree, raw, capture })
    }

    /// Install a capture-backed value observer on the tree root.
    pub fn add_parameter_observer_capture(
        &self,
        capacity: usize,
    ) -> Result<AuParameterObserverToken<AuParameterValueEvent>, AuError> {
        self.add_observer(
            capacity,
            |tree, context| unsafe {
                ffi::au_parameter_tree_add_value_observer(
                    tree,
                    push_parameter_value,
                    context,
                    CaptureContext::<AuParameterValueEvent>::RELEASE,
                )
            },
            "observer",
        )
    }

    /// Install a capture-backed recording observer on the tree root.
    pub fn add_parameter_recording_observer_capture(
        &self,
        capacity: usize,
    ) -> Result<AuParameterObserverToken<AuRecordedParameterEventInfo>, AuError> {
        self.add_observer(
            capacity,
            |tree, context| unsafe {
                ffi::au_parameter_tree_add_recording_observer(
                    tree,
                    push_recorded_parameter,
                    context,
                    CaptureContext::<AuRecordedParameterEventInfo>::RELEASE,
                )
            },
            "recording observer",
        )
    }

    /// Install a capture-backed automation observer on the tree root.
    pub fn add_parameter_automation_observer_capture(
        &self,
        capacity: usize,
    ) -> Result<AuParameterObserverToken<AuParameterAutomationEventInfo>, AuError> {
        self.add_observer(
            capacity,
            |tree, context| unsafe {
                ffi::au_parameter_tree_add_automation_observer(
                    tree,
                    push_parameter_automation,
                    context,
                    CaptureContext::<AuParameterAutomationEventInfo>::RELEASE,
                )
            },
            "automation observer",
        )
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

#[cfg(test)]
mod tests {
    use core::ffi::c_void;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    use doom_fish_utils::callback_context::CallbackContext;

    use crate::au_audio_unit::AuAudioUnit;
    use crate::av_audio_unit::InstantiationOptions;
    use crate::component_description::constants::{
        AUDIO_UNIT_SUBTYPE_DELAY, AUDIO_UNIT_TYPE_EFFECT,
    };
    use crate::component_description::AudioComponentDescription;
    use crate::ffi;

    struct ReleaseFlag(Arc<AtomicBool>);

    impl Drop for ReleaseFlag {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    unsafe extern "C" fn ignore_value(_: *mut c_void, _: u64, _: f32) {}

    #[test]
    fn removing_a_parameter_observer_releases_its_context() {
        let unit = AuAudioUnit::instantiate(
            AudioComponentDescription::apple(AUDIO_UNIT_TYPE_EFFECT, AUDIO_UNIT_SUBTYPE_DELAY),
            InstantiationOptions::InProcess,
        )
        .expect("delay unit");
        let tree = unit.parameter_tree().expect("parameter tree");
        let released = Arc::new(AtomicBool::new(false));
        let context = CallbackContext::new(ReleaseFlag(Arc::clone(&released)));
        let token = unsafe {
            ffi::au_parameter_tree_add_value_observer(
                tree.ptr,
                ignore_value,
                context.retained_ptr(),
                CallbackContext::<ReleaseFlag>::RELEASE,
            )
        };
        assert!(!token.is_null());
        drop(context);
        assert!(!released.load(Ordering::SeqCst));
        unsafe { ffi::au_parameter_tree_remove_parameter_observer(tree.ptr, token) };
        let deadline = Instant::now() + Duration::from_secs(2);
        while !released.load(Ordering::SeqCst) && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(5));
        }
        assert!(released.load(Ordering::SeqCst));
    }
}
