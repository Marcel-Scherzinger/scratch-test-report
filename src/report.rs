use derive_getters::Getters;

use crate::{messages::Messages, simulation::Simulation};

/// A report is the top-level data structure for responses when checking programs
#[derive(Debug, PartialEq, PartialOrd, Clone, Getters)]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Formality {
    initial_block: Result<svalue::ARc<str>, InitialBlockAmbiguity>,
    /// if the block pointers form a cycle (true), the file is malicious or at least invalid
    cyclic_graph: bool,
}

impl Formality {
    pub fn new(
        initial_block: Result<svalue::ARc<str>, InitialBlockAmbiguity>,
        cyclic_graph: bool,
    ) -> Self {
        Self {
            initial_block,
            cyclic_graph,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
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
