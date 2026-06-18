use derive_getters::Getters;

use crate::{
    Text,
    messages::{Message, Messages},
    simulation::Simulation,
};

/// A report is the top-level data structure for responses when checking programs
#[derive(Debug, PartialEq, PartialOrd, Clone, Getters)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Report {
    /// Messages for the whole report
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::is_default")
    )]
    messages: Messages<Report>,
    form: Option<Formality>,
    simulation: Option<Simulation>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Getters)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub struct Formality {
    #[cfg_attr(feature = "utoipa", schema(value_type= Result<String, InitialBlockAmbiguity>))]
    initial_block: Result<svalue::ARc<str>, InitialBlockAmbiguity>,
    /// if the block pointers form a cycle (true), the file is malicious or at least invalid
    cyclic_graph: bool,
    max_blocks_exceeded: Option<MaxBlocksExceeded>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Getters)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MaxBlocksExceeded {
    used: usize,
    allowed: usize,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::is_default")
    )]
    msg: Option<Message<Formality>>,
}
impl MaxBlocksExceeded {
    pub fn new_with_msg(used: usize, allowed: usize, msg: impl Into<Text>) -> Self {
        Self {
            used,
            allowed,
            msg: Some(Message::error(msg)),
        }
    }
    pub fn new(used: usize, allowed: usize) -> Self {
        Self {
            used,
            allowed,
            msg: None,
        }
    }
}

impl Formality {
    pub fn new(
        initial_block: Result<svalue::ARc<str>, InitialBlockAmbiguity>,
        cyclic_graph: bool,
        max_blocks_exceeded: Result<(), MaxBlocksExceeded>,
    ) -> Self {
        Self {
            initial_block,
            cyclic_graph,
            max_blocks_exceeded: max_blocks_exceeded.err(),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum InitialBlockAmbiguity {
    No,
    Multiple,
}

impl Report {
    pub fn new(form: Option<Formality>, simulation: Option<Simulation>) -> Self {
        Self {
            messages: Default::default(),
            form,
            simulation,
        }
    }
}
