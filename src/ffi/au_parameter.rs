use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_parameter_release(ptr: *mut c_void);
    pub fn au_parameter_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_parameter_get_value(ptr: *mut c_void) -> f32;
    pub fn au_parameter_set_value(ptr: *mut c_void, value: f32);
    pub fn au_parameter_identifier(ptr: *mut c_void) -> *mut c_char;
    pub fn au_parameter_display_name(ptr: *mut c_void) -> *mut c_char;
    pub fn au_parameter_display_name_with_length(ptr: *mut c_void, length: isize) -> *mut c_char;
    pub fn au_parameter_address(ptr: *mut c_void) -> u64;
    pub fn au_parameter_min_value(ptr: *mut c_void) -> f32;
    pub fn au_parameter_max_value(ptr: *mut c_void) -> f32;
    pub fn au_parameter_unit(ptr: *mut c_void) -> u32;
    pub fn au_parameter_string_from_value(ptr: *mut c_void, value: f32) -> *mut c_char;
    pub fn au_parameter_value_from_string(ptr: *mut c_void, value: *const c_char) -> f32;
    pub fn au_parameter_set_value_at_host_time(ptr: *mut c_void, value: f32, host_time: u64);
    pub fn au_parameter_set_value_with_event(
        ptr: *mut c_void,
        value: f32,
        host_time: u64,
        event_type: u32,
    );
}
