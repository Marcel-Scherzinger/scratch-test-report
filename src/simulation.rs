mod cases;
mod categories;
mod criterion;
mod error_transform;
mod implementations;
mod storage;

pub use cases::{CaseBuilderStatusSet, CaseBuilderUnspecifiedStatus, TestCase, TestCaseBuilder};
pub use categories::{Category, CategoryBuilder};
pub use criterion::TestCriterion;
pub use error_transform::RunningError;
pub use storage::{ActionsState, DataStorage};

pub type FinishedCaseBuilder = TestCaseBuilder<CaseBuilderStatusSet>;

use derive_getters::Getters;

use crate::{messages::Messages, simulation::cases::RunAnalysis};

/// Runs programs on different inputs and observes the results
///
/// Each simulation consists of multiple [categories](Category) that group
/// similar [`TestCase`]s
#[derive(Debug, PartialEq, PartialOrd, Clone, Getters, Default)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Simulation {
    /// Messages that apply to the whole simulation-part of a report.
    /// This could for example be a missing data structure in the code
    /// or a general issue that was detected by the simulation.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::is_default")
    )]
    messages: Messages<Simulation>,

    categories: Vec<Category>,

    analysis: RunAnalysis,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Getters)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub struct CategoryStatus {
    complete_success: usize,
    success_but_warnings: usize,
    failure: usize,
}
#[derive(Debug, PartialEq, PartialOrd, Clone, derive_more::Display)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum TestCaseStatus {
    #[display("success")]
    CompleteSucess,
    #[display("success-but-warnings")]
    SuccessButWarnings,
    #[display("failure")]
    Failure,
}
