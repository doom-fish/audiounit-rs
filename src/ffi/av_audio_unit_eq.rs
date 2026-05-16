use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_av_eq_create(
        number_of_bands: usize,
        out_unit: *mut *mut c_void,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_av_eq_release(ptr: *mut c_void);
    pub fn au_av_eq_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_av_eq_get_global_gain(ptr: *mut c_void) -> f32;
    pub fn au_av_eq_set_global_gain(ptr: *mut c_void, global_gain: f32);
    pub fn au_av_eq_band_count(ptr: *mut c_void) -> usize;
    pub fn au_av_eq_band_at(ptr: *mut c_void, index: usize) -> *mut c_void;
    pub fn au_av_eq_as_effect(ptr: *mut c_void) -> *mut c_void;

    pub fn au_av_eq_band_release(ptr: *mut c_void);
    pub fn au_av_eq_band_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_av_eq_band_get_filter_type(ptr: *mut c_void) -> i64;
    pub fn au_av_eq_band_set_filter_type(ptr: *mut c_void, filter_type: i64);
    pub fn au_av_eq_band_get_frequency(ptr: *mut c_void) -> f32;
    pub fn au_av_eq_band_set_frequency(ptr: *mut c_void, frequency: f32);
    pub fn au_av_eq_band_get_bandwidth(ptr: *mut c_void) -> f32;
    pub fn au_av_eq_band_set_bandwidth(ptr: *mut c_void, bandwidth: f32);
    pub fn au_av_eq_band_get_gain(ptr: *mut c_void) -> f32;
    pub fn au_av_eq_band_set_gain(ptr: *mut c_void, gain: f32);
    pub fn au_av_eq_band_get_bypass(ptr: *mut c_void) -> bool;
    pub fn au_av_eq_band_set_bypass(ptr: *mut c_void, bypass: bool);
}
