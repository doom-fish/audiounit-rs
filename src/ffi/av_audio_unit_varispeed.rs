use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_av_varispeed_create(
        out_unit: *mut *mut c_void,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_av_varispeed_release(ptr: *mut c_void);
    pub fn au_av_varispeed_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_av_varispeed_get_rate(ptr: *mut c_void) -> f32;
    pub fn au_av_varispeed_set_rate(ptr: *mut c_void, rate: f32);
    pub fn au_av_varispeed_as_time_effect(ptr: *mut c_void) -> *mut c_void;
}
