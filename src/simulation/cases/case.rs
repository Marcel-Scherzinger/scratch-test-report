use sinterpreter::default_state::DefaultState;

use crate::{
    messages::{MessageAdder, Messages},
    report::Report,
    simulation::{
        CaseBuilderUnspecifiedStatus, TestCase, TestCaseBuilder, TestCaseStatus,
        error_transform::RunningErrorMessage,
    },
};

impl TestCase {
    pub fn create() -> TestCaseBuilder<CaseBuilderUnspecifiedStatus> {
        TestCaseBuilder {
            // the type state encodes that this
            // value has to be overwritten at some
            // time so it is save to use this as default
            status: TestCaseStatus::Failure,
            inputs: Default::default(),
            randoms: Default::default(),
            received: Default::default(),
            error_code: Default::default(),
            criterion: Default::default(),
            analysis: Default::default(),
            messages: Default::default(),
            phantom: Default::default(),
        }
    }

    pub(crate) fn add_extra_messages(&mut self, report_msgs: &mut Messages<Report>) {
        match self.error_code.as_ref().map(|e| e.to_message()) {
            Some(RunningErrorMessage::Report(r)) => report_msgs.notify(r),
            Some(RunningErrorMessage::Case(c)) => self.messages.notify(c),
            None => {}
        }
    }

    pub fn create_from_run_ref(
        run_report: &sinterpreter::Report<'_, DefaultState>,
    ) -> TestCaseBuilder<CaseBuilderUnspecifiedStatus> {
        let mut b = Self::create_from_state_ref(run_report.state());
        b.error_code(run_report.error_code().clone());
        b
    }
    pub fn create_from_state_ref(
        state: &DefaultState,
    ) -> TestCaseBuilder<CaseBuilderUnspecifiedStatus> {
        let mut b = Self::create();

        b.inputs_from(state.answer_inputs().iter().map(|x| x.clone().into()))
            .received_output_from(state.output_lines().cloned())
            .received_lists(
                state
                    .lists()
                    .iter()
                    .map(|(id, value)| {
                        (id.name().to_owned().into(), value.iter().cloned().collect())
                    })
                    .collect(),
            )
            .received_variables(
                state
                    .variables()
                    .iter()
                    .map(|(id, value)| (id.name().to_owned().into(), value.clone()))
                    .collect(),
            )
            .uninitialized_usages(state.uninitialized_usages().clone());
        b
    }
}
