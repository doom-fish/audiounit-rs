use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_v2_bridge_from_auaudiounit(ptr: *mut c_void) -> *mut c_void;
    pub fn au_v2_bridge_release(ptr: *mut c_void);
    pub fn au_v2_bridge_audio_unit(ptr: *mut c_void) -> *mut c_void;
    pub fn au_v2_bridge_snapshot_json(ptr: *mut c_void) -> *mut c_char;
}
