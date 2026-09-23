use core::ffi::{c_char, c_void};

pub type ParameterValueCallback = unsafe extern "C" fn(*mut c_void, u64, f32);
pub type ParameterRecordingCallback = unsafe extern "C" fn(*mut c_void, u64, u64, f32);
pub type ParameterAutomationCallback = unsafe extern "C" fn(*mut c_void, u64, u64, f32, u32);

unsafe extern "C" {
    pub fn au_parameter_tree_release(ptr: *mut c_void);
    pub fn au_parameter_tree_retain(ptr: *mut c_void) -> *mut c_void;
    pub fn au_parameter_tree_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_parameter_tree_parameter_with_address(tree: *mut c_void, address: u64)
        -> *mut c_void;
    pub fn au_parameter_tree_parameter_with_id(
        tree: *mut c_void,
        parameter_id: u32,
        scope: u32,
        element: u32,
    ) -> *mut c_void;
    pub fn au_parameter_tree_root_group(tree: *mut c_void) -> *mut c_void;
    pub fn au_parameter_tree_add_value_observer(
        ptr: *mut c_void,
        callback: ParameterValueCallback,
        context: *mut c_void,
        release: super::ContextRelease,
    ) -> *mut c_void;
    pub fn au_parameter_tree_add_recording_observer(
        ptr: *mut c_void,
        callback: ParameterRecordingCallback,
        context: *mut c_void,
        release: super::ContextRelease,
    ) -> *mut c_void;
    pub fn au_parameter_tree_add_automation_observer(
        ptr: *mut c_void,
        callback: ParameterAutomationCallback,
        context: *mut c_void,
        release: super::ContextRelease,
    ) -> *mut c_void;
    pub fn au_parameter_tree_remove_parameter_observer(ptr: *mut c_void, token: *mut c_void);
}
