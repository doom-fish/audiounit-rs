#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
)]

pub mod component;
pub mod component_description;
pub mod error;
pub mod ffi;
pub mod legacy;
pub mod parameter;
pub mod unit;

pub use component::{AudioUnitComponent, ComponentManager};
pub use component_description::AudioComponentDescription;
pub use error::AuError;
pub use parameter::{AuParameter, AuParameterNode, AuParameterTree};
pub use unit::AvAudioUnit;

pub mod prelude {
    pub use crate::component::{AudioUnitComponent, ComponentManager};
    pub use crate::component_description::AudioComponentDescription;
    pub use crate::error::AuError;
    pub use crate::parameter::{AuParameter, AuParameterNode, AuParameterTree};
    pub use crate::unit::{AvAudioUnit, InstantiationOptions};
    pub use crate::legacy::*;
    pub use crate::component_description::constants::*;
}
