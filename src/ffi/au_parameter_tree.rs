use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_parameter_tree_release(ptr: *mut c_void);
    pub fn au_parameter_tree_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_parameter_tree_parameter_with_address(tree: *mut c_void, address: u64) -> *mut c_void;
    pub fn au_parameter_tree_parameter_with_id(tree: *mut c_void, parameter_id: u32, scope: u32, element: u32) -> *mut c_void;
    pub fn au_parameter_tree_root_group(tree: *mut c_void) -> *mut c_void;
}
