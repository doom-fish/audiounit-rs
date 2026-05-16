use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_bus_release(ptr: *mut c_void);
    pub fn au_bus_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_bus_set_standard_format(ptr: *mut c_void, sample_rate: f64, channel_count: u32, out_error_msg: *mut *mut c_char) -> i32;
    pub fn au_bus_set_should_allocate_buffer(ptr: *mut c_void, value: bool);
    pub fn au_bus_set_enabled(ptr: *mut c_void, value: bool);
    pub fn au_bus_set_name(ptr: *mut c_void, value: *const c_char);
    pub fn au_bus_set_context_presentation_latency(ptr: *mut c_void, value: f64);
    pub fn au_bus_owner_audio_unit(ptr: *mut c_void) -> *mut c_void;
}
