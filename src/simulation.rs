mod implementations;

use std::collections::BTreeMap;

use derive_getters::Getters;
use svalue::SValue;

use crate::{Text, messages::Messages};

/// Runs programs on different inputs and observes the results
///
/// Each simulation consists of multiple [categories](Category) that group
/// similar [`TestCase`]s
#[derive(Debug, PartialEq, PartialOrd, Clone, Getters)]
pub struct Simulation {
    /// Messages that apply to the whole simulation-part of a report.
    /// This could for example be a missing data structure in the code
    /// or a general issue that was detected by the simulation.
    messages: Messages<Simulation>,

    categories: Vec<Category>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Getters)]
pub struct CategoryStatus {
    complete_success: usize,
    success_but_warnings: usize,
    failure: usize,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum TestCaseStatus {
    CompleteSucess,
    SuccessButWarnings,
    Failure,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Getters)]
pub struct Category {
    /// An optional description giving a hint on what the contained
    /// test cases have in common i. e. what property they test
    description: Option<Text>,
    status: CategoryStatus,
    /// All test cases of this category
    cases: Vec<TestCase>,
    // TODO: Is this needed?
    // /// Random numbers specified here were provided to every test case
    // /// in this category. This is mostly for saving bandwidth if a category
    // /// executes a test multiple times with different inputs but the same randoms
    // randoms: Option<Vec<svalue::SNumber>>,
}

/// Encodes what a test checked and what is the difference between expected and
/// received results
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum TestCriterion {
    /// The last line the program output should have an `expected` value.
    /// Any other (slightly different) value will be counted as failure.
    LastOutputExact { expected: Text },
    /// The last line the program output encodes a decision or value it was asked
    /// for. This value could be formatted in different ways so a special layer
    /// was used to extract the needed parts and interpret what the decision is.
    /// This layer could guess wrongly.
    LastOutputInterpreted {
        /// One example that would have leaded to the expected interpretation.
        /// This can be shown to the user for comparison
        sample_expected: Text,
        /// The expected interpretation (decision/value)
        iexpected: Text,
        /// The interpretation of the program output, `None` if there was no output
        ireceived: Option<Text>,
    },
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Getters)]
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
    criterion: Option<TestCriterion>,
}

/// Results of linting that is performed on test case level
#[derive(Debug, PartialEq, PartialOrd, Clone, Getters)]
pub struct TestCaseLints {
    /// The test succeeded (with or without warnings) when running
    /// it with initially empty lists and variables, but failed
    /// when the values were set to _some other_ values.
    /// This indicates that the program doesn't initialize its storage.
    uninitialized_data: Option<TestCaseLintUninitialized>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Getters)]
pub struct TestCaseLintUninitialized {
    /// An initial setting of variables and lists that lets the
    /// program misbehave even if it worked well for clean state
    problematic_values: DataStorage,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Getters)]
pub struct ActionsState {
    output: Vec<SValue>,
    lists: Option<BTreeMap<Text, Vec<SValue>>>,
    variables: Option<BTreeMap<Text, SValue>>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Getters)]
pub struct DataStorage {
    lists: BTreeMap<Text, Vec<SValue>>,
    variables: BTreeMap<Text, SValue>,
}
