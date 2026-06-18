use std::collections::{BTreeMap, BTreeSet};

use either::Either;
use sinterpreter::{RunError, default_state::DefaultStateError};
use smodel::attrs::{List, Variable};
use svalue::{SNumber, SValue};

use crate::{
    CategoryBLevelMessages, Text,
    messages::{Message, MessageAdder, MessageKind},
    simulation::{
        ActionsState, CaseBuilderStatusSet, CaseBuilderUnspecifiedStatus, Category, Simulation,
        TestCase, TestCaseBuilder, TestCaseStatus, TestCriterion, cases::RunAnalysis,
        error_transform::RunningError,
    },
};

impl TestCaseBuilder<CaseBuilderStatusSet> {
    pub fn build(self) -> (TestCase, CategoryBLevelMessages, RunAnalysis) {
        let test = TestCase {
            criterion: self.criterion,
            status: self.status,
            inputs: self.inputs,
            randoms: self.randoms,
            received: self.received,
            error_code: self.error_code,
            messages: self.messages.0,
        };

        (test, self.messages.1, self.analysis)
    }
}

impl From<TestCaseBuilder<CaseBuilderUnspecifiedStatus>> for TestCaseBuilder<CaseBuilderStatusSet> {
    fn from(value: TestCaseBuilder<CaseBuilderUnspecifiedStatus>) -> Self {
        value.derived_status()
    }
}

impl<S> MessageAdder<TestCase> for TestCaseBuilder<S> {
    fn notify(&mut self, msg: Message<TestCase>) {
        self.messages.0.notify(msg);
    }
}

impl<S> MessageAdder<Category> for TestCaseBuilder<S> {
    fn notify(&mut self, msg: Message<Category>) {
        self.messages.1.0.notify(msg);
    }
}

impl<S> MessageAdder<Simulation> for TestCaseBuilder<S> {
    fn notify(&mut self, msg: Message<Simulation>) {
        self.messages.1.1.notify(msg);
    }
}

impl TestCaseBuilder<CaseBuilderUnspecifiedStatus> {
    pub fn derived_status(self) -> TestCaseBuilder<CaseBuilderStatusSet> {
        let status = if self.messages.0.has_kind(&MessageKind::Error)
            || self.error_code.is_some()
            || self.criterion.as_ref().is_some_and(|c| !c.is_successful())
        {
            TestCaseStatus::Failure
        } else if self.messages.0.has_kind(&MessageKind::Warning) {
            TestCaseStatus::SuccessButWarnings
        } else {
            TestCaseStatus::CompleteSucess
        };

        TestCaseBuilder {
            status,
            inputs: self.inputs,
            randoms: self.randoms,
            received: self.received,
            error_code: self.error_code,
            criterion: self.criterion,
            analysis: self.analysis,
            messages: self.messages,
            phantom: Default::default(),
        }
    }
}
impl<S> TestCaseBuilder<S> {
    // TODO: Think about distinguishing provided and actually used inputs
    /// Set the program inputs
    pub fn inputs(&mut self, inputs: Vec<SValue>) -> &mut Self {
        self.inputs = inputs;
        self
    }
    /// Set the program inputs
    pub fn inputs_from(&mut self, inputs: impl IntoIterator<Item = SValue>) -> &mut Self {
        self.inputs = inputs.into_iter().collect();
        self
    }
    /// Add another program input at the end
    pub fn add_input(&mut self, input: SValue) -> &mut Self {
        self.inputs.push(input);
        self
    }
    /// Set the random numbers, the program requested and got
    pub fn randoms(&mut self, randoms: Vec<SNumber>) -> &mut Self {
        *self.randoms.get_or_insert_default() = randoms;
        self
    }
    /// Set the random numbers, the program requested and got
    pub fn randoms_from(&mut self, randoms: impl IntoIterator<Item = SNumber>) -> &mut Self {
        *self.randoms.get_or_insert_default() = randoms.into_iter().collect();
        self
    }
    /// Add another requested random number at the end
    pub fn add_random(&mut self, random: SNumber) -> &mut Self {
        self.randoms.get_or_insert_default().push(random);
        self
    }
    /// Set the output, the program produced
    pub fn received_output(&mut self, output: Vec<SValue>) -> &mut Self {
        *self.received.output_mut() = output;
        self
    }
    /// Set the output, the program produced
    pub fn received_output_from(&mut self, output: impl IntoIterator<Item = SValue>) -> &mut Self {
        *self.received.output_mut() = output.into_iter().collect();
        self
    }
    /// Add another output line at the end
    pub fn add_received_output(&mut self, output: SValue) -> &mut Self {
        self.received.output_mut().push(output);
        self
    }
    /// Set the lists the program had when exiting
    pub fn received_lists(&mut self, lists: BTreeMap<Text, Vec<SValue>>) -> &mut Self {
        *self.received.data_mut().lists_mut() = lists;
        self
    }
    /// Set the variables the program had when exiting
    pub fn received_variables(&mut self, variables: BTreeMap<Text, SValue>) -> &mut Self {
        *self.received.data_mut().variables_mut() = variables;
        self
    }
    pub fn uninitialized_usages(&mut self, data: BTreeSet<Either<Variable, List>>) -> &mut Self {
        self.analysis.uninitialized_data = data.into_iter().map(|s| s.into()).collect();
        self
    }
    pub fn criterion(&mut self, crit: TestCriterion) -> &mut Self {
        self.criterion = Some(crit);
        self
    }
    pub fn error_code(&mut self, error_code: Option<RunError<DefaultStateError>>) -> &mut Self {
        self.error_code = error_code.and_then(RunningError::from_interpreter_error);
        self
    }

    pub fn received_(&self) -> &ActionsState {
        &self.received
    }

    pub fn flag_hardcoding(&mut self, message: impl Into<String>) -> &mut Self {
        self.analysis.hardcoding = Some(Err(Message::info(message.into())));
        self
    }
}
