use derive_getters::Getters;

use crate::{messages::Messages, simulation::Simulation};

/// A report is the top-level data structure for responses when checking programs
#[derive(Debug, PartialEq, PartialOrd, Clone, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Report {
    /// Messages for the whole report
    messages: Messages<Report>,
    simulation: Simulation,
}
