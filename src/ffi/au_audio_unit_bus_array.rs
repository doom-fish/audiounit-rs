use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_bus_array_release(ptr: *mut c_void);
    pub fn au_bus_array_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_bus_array_bus_at(ptr: *mut c_void, index: usize) -> *mut c_void;
    pub fn au_bus_array_set_bus_count(
        ptr: *mut c_void,
        count: usize,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
}
