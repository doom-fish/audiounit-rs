use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_factory_create() -> *mut c_void;
    pub fn au_factory_release(ptr: *mut c_void);
    pub fn au_factory_create_audio_unit(
        ptr: *mut c_void,
        component_type: u32,
        component_subtype: u32,
        component_manufacturer: u32,
        component_flags: u32,
        component_flags_mask: u32,
        out_unit: *mut *mut c_void,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
}
