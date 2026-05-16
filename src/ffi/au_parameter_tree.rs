use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_parameter_tree_release(ptr: *mut c_void);
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
    pub fn au_parameter_tree_add_parameter_observer_capture(ptr: *mut c_void) -> *mut c_void;
    pub fn au_parameter_tree_add_parameter_recording_observer_capture(
        ptr: *mut c_void,
    ) -> *mut c_void;
    pub fn au_parameter_tree_add_parameter_automation_observer_capture(
        ptr: *mut c_void,
    ) -> *mut c_void;
    pub fn au_parameter_tree_take_parameter_observer_events_json(
        ptr: *mut c_void,
        token: *mut c_void,
    ) -> *mut c_char;
    pub fn au_parameter_tree_take_parameter_recording_events_json(
        ptr: *mut c_void,
        token: *mut c_void,
    ) -> *mut c_char;
    pub fn au_parameter_tree_take_parameter_automation_events_json(
        ptr: *mut c_void,
        token: *mut c_void,
    ) -> *mut c_char;
    pub fn au_parameter_tree_remove_parameter_observer(ptr: *mut c_void, token: *mut c_void);
}
