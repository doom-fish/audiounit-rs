//! Backwards-compatible re-exports for the original `parameter` module.

pub use crate::au_parameter::{AuParameter, AuParameterAutomationEventType, AuParameterInfo};
pub use crate::au_parameter_group::{AuParameterGroup, AuParameterGroupInfo};
pub use crate::au_parameter_tree::{AuParameterNodeInfo, AuParameterTree};

/// Backwards-compatible alias for parameter-node snapshots.
pub type AuParameterNode = AuParameterNodeInfo;
