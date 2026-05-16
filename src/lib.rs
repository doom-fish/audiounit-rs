#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
)]

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
pub mod av_audio_unit_effect;
pub mod av_audio_unit_generator;
pub mod av_audio_unit_instrument;
pub mod av_audio_unit_midi_instrument;
pub mod component;
pub mod component_description;
pub mod error;
pub mod ffi;
pub mod legacy;
pub mod parameter;
pub mod unit;
mod util;

pub use au_audio_unit::{AuAudioUnit, AuAudioUnitInfo, AuAudioUnitPreset};
pub use au_audio_unit_bus::{AudioFormatInfo, AuAudioUnitBus, AuAudioUnitBusInfo};
pub use au_audio_unit_bus_array::{AuAudioUnitBusArray, AuAudioUnitBusArrayInfo};
pub use au_audio_unit_factory::AuAudioUnitFactory;
pub use au_audio_unit_v2_bridge::{AuAudioUnitV2Bridge, AuAudioUnitV2BridgeInfo};
pub use au_parameter::{AuParameter, AuParameterAutomationEventType, AuParameterInfo};
pub use au_parameter_group::{AuParameterGroup, AuParameterGroupInfo};
pub use au_parameter_tree::{AuParameterNodeInfo, AuParameterTree};
pub use au_voice_io::{AuVoiceIo, AuVoiceIoInfo, AuVoiceIoOtherAudioDuckingConfiguration, AuVoiceIoOtherAudioDuckingLevel, AuVoiceIO};
pub use av_audio_unit::{AvAudioUnit, AvAudioUnitInfo, InstantiationOptions};
pub use av_audio_unit_effect::{AvAudioUnitEffect, AvAudioUnitEffectInfo};
pub use av_audio_unit_generator::{AvAudioUnitGenerator, AvAudioUnitGeneratorInfo};
pub use av_audio_unit_instrument::{AvAudioUnitInstrument, AvAudioUnitInstrumentInfo};
pub use av_audio_unit_midi_instrument::{AvAudioUnitMidiInstrument, AvAudioUnitMidiInstrumentInfo, AvAudioUnitMIDIInstrument};
pub use component::{AudioUnitComponent, ComponentManager};
pub use component_description::AudioComponentDescription;
pub use error::AuError;

pub mod prelude {
    pub use crate::au_audio_unit::{AuAudioUnit, AuAudioUnitInfo, AuAudioUnitPreset};
    pub use crate::au_audio_unit_bus::{AudioFormatInfo, AuAudioUnitBus, AuAudioUnitBusInfo};
    pub use crate::au_audio_unit_bus_array::{AuAudioUnitBusArray, AuAudioUnitBusArrayInfo};
    pub use crate::au_audio_unit_factory::AuAudioUnitFactory;
    pub use crate::au_audio_unit_v2_bridge::{AuAudioUnitV2Bridge, AuAudioUnitV2BridgeInfo};
    pub use crate::au_parameter::{AuParameter, AuParameterAutomationEventType, AuParameterInfo};
    pub use crate::au_parameter_group::{AuParameterGroup, AuParameterGroupInfo};
    pub use crate::au_parameter_tree::{AuParameterNodeInfo, AuParameterTree};
    pub use crate::au_voice_io::{AuVoiceIo, AuVoiceIoInfo, AuVoiceIoOtherAudioDuckingConfiguration, AuVoiceIoOtherAudioDuckingLevel, AuVoiceIO};
    pub use crate::av_audio_unit::{AvAudioUnit, AvAudioUnitInfo, InstantiationOptions};
    pub use crate::av_audio_unit_effect::{AvAudioUnitEffect, AvAudioUnitEffectInfo};
    pub use crate::av_audio_unit_generator::{AvAudioUnitGenerator, AvAudioUnitGeneratorInfo};
    pub use crate::av_audio_unit_instrument::{AvAudioUnitInstrument, AvAudioUnitInstrumentInfo};
    pub use crate::av_audio_unit_midi_instrument::{AvAudioUnitMidiInstrument, AvAudioUnitMidiInstrumentInfo, AvAudioUnitMIDIInstrument};
    pub use crate::component::{AudioUnitComponent, ComponentManager};
    pub use crate::component_description::constants::*;
    pub use crate::component_description::AudioComponentDescription;
    pub use crate::error::AuError;
    pub use crate::legacy::*;
}
