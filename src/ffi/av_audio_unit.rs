use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_instantiate_sync(
        component_type: u32,
        component_subtype: u32,
        component_manufacturer: u32,
        component_flags: u32,
        component_flags_mask: u32,
        options: u32,
        out_unit: *mut *mut c_void,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_avunit_release(ptr: *mut c_void);
    pub fn au_avunit_audio_unit(ptr: *mut c_void) -> *mut c_void;
    pub fn au_avunit_auaudiounit(ptr: *mut c_void) -> *mut c_void;
    pub fn au_avunit_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_avunit_load_audio_unit_preset(
        ptr: *mut c_void,
        path: *const c_char,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
}
