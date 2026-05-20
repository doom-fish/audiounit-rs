use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_auaudiounit_instantiate_sync(
        component_type: u32,
        component_subtype: u32,
        component_manufacturer: u32,
        component_flags: u32,
        component_flags_mask: u32,
        options: u32,
        out_unit: *mut *mut c_void,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_auaudiounit_release(ptr: *mut c_void);
    pub fn au_auaudiounit_retain(ptr: *mut c_void) -> *mut c_void;
    pub fn au_auaudiounit_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_auaudiounit_component(ptr: *mut c_void) -> *mut c_void;
    pub fn au_auaudiounit_allocate_render_resources(
        ptr: *mut c_void,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_auaudiounit_deallocate_render_resources(ptr: *mut c_void);
    pub fn au_auaudiounit_reset(ptr: *mut c_void);
    pub fn au_auaudiounit_input_busses(ptr: *mut c_void) -> *mut c_void;
    pub fn au_auaudiounit_output_busses(ptr: *mut c_void) -> *mut c_void;
    pub fn au_auaudiounit_parameter_tree(ptr: *mut c_void) -> *mut c_void;
    pub fn au_auaudiounit_parameters_for_overview_json(
        ptr: *mut c_void,
        count: usize,
    ) -> *mut c_char;
    pub fn au_auaudiounit_render(
        ptr: *mut c_void,
        action_flags: *mut u32,
        time_stamp: *const c_void,
        frame_count: u32,
        output_bus_number: isize,
        output_data: *mut c_void,
    ) -> i32;
    pub fn au_auaudiounit_schedule_parameter(
        ptr: *mut c_void,
        event_sample_time: i64,
        ramp_duration_sample_frames: u32,
        parameter_address: u64,
        value: f32,
    ) -> i32;
    pub fn au_auaudiounit_add_render_observer_capture(
        ptr: *mut c_void,
        out_token: *mut isize,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_auaudiounit_take_render_observer_events_json(
        ptr: *mut c_void,
        token: isize,
    ) -> *mut c_char;
    pub fn au_auaudiounit_remove_render_observer_capture(ptr: *mut c_void, token: isize);
    pub fn au_auaudiounit_schedule_midi_event(
        ptr: *mut c_void,
        event_sample_time: i64,
        cable: u8,
        bytes: *const u8,
        length: usize,
    ) -> i32;
    pub fn au_auaudiounit_schedule_midi_event_list(
        ptr: *mut c_void,
        event_sample_time: i64,
        cable: u8,
        event_list: *const c_void,
    ) -> i32;
    pub fn au_auaudiounit_set_midi_output_event_capture_enabled(ptr: *mut c_void, enabled: bool);
    pub fn au_auaudiounit_take_midi_output_events_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_auaudiounit_set_midi_output_event_list_capture_enabled(
        ptr: *mut c_void,
        enabled: bool,
    );
    pub fn au_auaudiounit_take_midi_output_event_lists_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_auaudiounit_set_musical_context_json(
        ptr: *mut c_void,
        value: *const c_char,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_auaudiounit_musical_context_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_auaudiounit_set_transport_state_json(
        ptr: *mut c_void,
        value: *const c_char,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_auaudiounit_transport_state_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_auaudiounit_profile_state_for_cable_channel_json(
        ptr: *mut c_void,
        cable: u8,
        channel: u8,
        out_json: *mut *mut c_char,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_auaudiounit_enable_profile(
        ptr: *mut c_void,
        profile_id: *const u8,
        length: usize,
        name: *const c_char,
        cable: u8,
        channel: u8,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_auaudiounit_disable_profile(
        ptr: *mut c_void,
        profile_id: *const u8,
        length: usize,
        name: *const c_char,
        cable: u8,
        channel: u8,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_auaudiounit_message_channel(
        ptr: *mut c_void,
        name: *const c_char,
        out_channel: *mut *mut c_void,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_message_channel_release(ptr: *mut c_void);
    pub fn au_message_channel_call_audio_unit_json(
        ptr: *mut c_void,
        message_json: *const c_char,
        out_json: *mut *mut c_char,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_auaudiounit_can_perform_input(ptr: *mut c_void) -> bool;
    pub fn au_auaudiounit_can_perform_output(ptr: *mut c_void) -> bool;
    pub fn au_auaudiounit_set_device_id(
        ptr: *mut c_void,
        device_id: u32,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_auaudiounit_start_hardware(ptr: *mut c_void, out_error_msg: *mut *mut c_char) -> i32;
    pub fn au_auaudiounit_stop_hardware(ptr: *mut c_void);
    pub fn au_auaudiounit_set_maximum_frames_to_render(ptr: *mut c_void, value: u32);
    pub fn au_auaudiounit_set_render_quality(ptr: *mut c_void, value: isize);
    pub fn au_auaudiounit_set_should_bypass_effect(ptr: *mut c_void, value: bool);
    pub fn au_auaudiounit_set_rendering_offline(ptr: *mut c_void, value: bool);
    pub fn au_auaudiounit_set_context_name(ptr: *mut c_void, value: *const c_char);
    pub fn au_auaudiounit_set_current_preset(ptr: *mut c_void, number: isize, name: *const c_char);
    pub fn au_auaudiounit_set_channel_map(ptr: *mut c_void, values: *const i32, count: usize);
}
