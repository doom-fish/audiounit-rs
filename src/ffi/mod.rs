//! Raw FFI declarations matching the Swift bridge.

#![allow(missing_docs, non_camel_case_types)]

pub mod au_audio_unit;
pub mod au_audio_unit_bus;
pub mod au_audio_unit_bus_array;
pub mod au_audio_unit_factory;
pub mod au_audio_unit_v2_bridge;
pub mod au_parameter;
pub mod au_parameter_group;
pub mod au_parameter_tree;
pub mod au_voice_io;
pub mod av_audio_unit;
pub mod av_audio_unit_delay;
pub mod av_audio_unit_distortion;
pub mod av_audio_unit_effect;
pub mod av_audio_unit_eq;
pub mod av_audio_unit_generator;
pub mod av_audio_unit_instrument;
pub mod av_audio_unit_midi_instrument;
pub mod av_audio_unit_reverb;
pub mod av_audio_unit_sampler;
pub mod av_audio_unit_time_effect;
pub mod av_audio_unit_time_pitch;
pub mod av_audio_unit_varispeed;
pub mod component;

pub use au_audio_unit::*;
pub use au_audio_unit_bus::*;
pub use au_audio_unit_bus_array::*;
pub use au_audio_unit_factory::*;
pub use au_audio_unit_v2_bridge::*;
pub use au_parameter::*;
pub use au_parameter_group::*;
pub use au_parameter_tree::*;
pub use au_voice_io::*;
pub use av_audio_unit::*;
pub use av_audio_unit_delay::*;
pub use av_audio_unit_distortion::*;
pub use av_audio_unit_effect::*;
pub use av_audio_unit_eq::*;
pub use av_audio_unit_generator::*;
pub use av_audio_unit_instrument::*;
pub use av_audio_unit_midi_instrument::*;
pub use av_audio_unit_reverb::*;
pub use av_audio_unit_sampler::*;
pub use av_audio_unit_time_effect::*;
pub use av_audio_unit_time_pitch::*;
pub use av_audio_unit_varispeed::*;
pub use component::*;

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const INSTANTIATE_FAILED: i32 = -2;
    pub const TIMED_OUT: i32 = -3;
    pub const PROPERTY_ERROR: i32 = -4;
    pub const UNAVAILABLE: i32 = -5;
    pub const UNKNOWN: i32 = -99;
}
