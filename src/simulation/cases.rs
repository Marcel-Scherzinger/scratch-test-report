mod case;
mod case_builder;
mod run_analysis;

use std::collections::BTreeSet;

use derive_getters::Getters;
use derive_more::From;
use either::Either;
use smodel::attrs::{List, Variable};
use svalue::{SNumber, SValue};

use crate::{
    CaseLevelBMessages,
    messages::{Message, Messages},
    report::SchemaResult,
    simulation::{
        ActionsState, Simulation, TestCaseStatus, TestCriterion, error_transform::RunningError,
    },
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Getters)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TestCase {
    /// If the test:
    /// - [succeeded entirely](TestCaseStatus::CompleteSucess)
    /// - [succeeded but produced warnings](TestCaseStatus::SuccessButWarnings)
    /// - [failed](TestCaseStatus::Failure)
    status: TestCaseStatus,
    /// The inputs the program got
    inputs: Vec<svalue::SValue>,
    /// The random numbers the program got on request,
    ///
    /// **TODO**: Decide if this can also contain randoms that would have been
    /// provided but weren't needed due to unexpected program behaviour
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::is_default")
    )]
    randoms: Option<Vec<svalue::SNumber>>,
    /// The state (outputs, data) the program produced/wrote while running.
    ///
    /// - Outputs
    /// - Lists
    /// - Varaibles
    received: ActionsState,
    /// The difference between expected and received results.
    /// This describes the reason why a program output/data has a value
    /// that caused the test case to fail.
    ///
    /// Should *always* exist for failed tests and tests with warnings.
    /// This _could_ exist on successful tests as well!
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::is_default")
    )]
    criterion: Option<TestCriterion>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::is_default")
    )]
    error_code: Option<RunningError>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::is_default")
    )]
    messages: Messages<TestCase>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct CaseBuilderUnspecifiedStatus;
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct CaseBuilderStatusSet;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct TestCaseBuilder<S> {
    status: TestCaseStatus,
    inputs: Vec<SValue>,
    randoms: Option<Vec<SNumber>>,
    received: ActionsState,
    error_code: Option<RunningError>,
    criterion: Option<TestCriterion>,
    analysis: RunAnalysis,
    messages: CaseLevelBMessages,
    phantom: std::marker::PhantomData<S>,
}

/// Results of linting that is performed on test case level
#[derive(Debug, PartialEq, PartialOrd, Clone, Getters, Default)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunAnalysis {
    #[cfg_attr(feature = "utoipa", schema(value_type=Alias))]
    hardcoding: Option<Result<(), Message<Simulation>>>,
    uninitialized_data: BTreeSet<VarOrList>,
}
type Alias = SchemaResult<(), Message<Simulation>>;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, From)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum VarOrList {
    Var { id: Variable },
    List { id: List },
}
impl From<Either<Variable, List>> for VarOrList {
    fn from(value: Either<Variable, List>) -> Self {
        match value {
            Either::Left(v) => v.into(),
            Either::Right(v) => v.into(),
        }
    }
}
