use svalue::SNumber;

use crate::messages::MessageAdder;

use super::*;

impl Simulation {
    pub fn add_category(&mut self, category: Category) -> &mut Self {
        self.categories.push(category);
        self
    }
}

impl Category {
    pub fn compute_from(description: Option<impl Into<Text>>, cases: Vec<TestCase>) -> Category {
        let description = description.map(Into::into);
        let (mut success, mut with_warn, mut fail) = (0, 0, 0);

        for case in cases.iter() {
            match case.status() {
                TestCaseStatus::Failure => fail += 1,
                TestCaseStatus::SuccessButWarnings => with_warn += 1,
                TestCaseStatus::CompleteSucess => success += 1,
            }
        }

        Category {
            description,
            status: CategoryStatus {
                complete_success: success,
                success_but_warnings: with_warn,
                failure: fail,
            },
            cases,
        }
    }
}

impl MessageAdder<Simulation> for Simulation {
    fn notify(&mut self, msg: crate::messages::Message<Simulation>) {
        self.messages.notify(msg);
    }
}

impl TestCase {
    pub fn new(
        status: TestCaseStatus,
        inputs: Vec<SValue>,
        randoms: Option<Vec<SNumber>>,
        received: ActionsState,
        criterion: Option<TestCriterion>,
    ) -> Self {
        Self {
            status,
            inputs,
            randoms,
            received,
            criterion,
        }
    }
}

impl ActionsState {
    pub fn new(output: impl Into<Vec<SValue>>) -> Self {
        Self::from_parts(output.into(), None, None)
    }
    pub fn from_parts(
        output: Vec<SValue>,
        lists: Option<BTreeMap<Text, Vec<SValue>>>,
        variables: Option<BTreeMap<Text, SValue>>,
    ) -> Self {
        Self {
            output,
            lists,
            variables,
        }
    }
    pub fn set_variable(&mut self, name: Text, value: impl Into<SValue>) -> &mut Self {
        self.variables
            .get_or_insert_default()
            .insert(name, value.into());
        self
    }
    pub fn set_list(&mut self, name: Text, value: impl Into<Vec<SValue>>) -> &mut Self {
        self.lists
            .get_or_insert_default()
            .insert(name, value.into());
        self
    }
}
