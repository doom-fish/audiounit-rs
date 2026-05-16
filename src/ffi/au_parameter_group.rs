use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_parameter_group_release(ptr: *mut c_void);
    pub fn au_parameter_group_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_parameter_group_children_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_parameter_group_all_parameters_json(ptr: *mut c_void) -> *mut c_char;
}
