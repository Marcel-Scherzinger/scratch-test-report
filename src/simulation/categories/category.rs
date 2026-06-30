use crate::{
    Text,
    messages::Messages,
    prelude::Simulation,
    report::Report,
    simulation::{Category, CategoryBuilder, CategoryStatus, TestCase, TestCaseStatus},
};

impl CategoryStatus {
    pub(crate) fn total_count(&self) -> usize {
        self.failure() + self.success_but_warnings() + self.complete_success()
    }
}

impl Category {
    pub(crate) fn limit_tests(&mut self) {
        if self.status.total_count() > 30 {
            let mut failures_to_remove = self.status().failure().saturating_sub(10);
            let mut warnings_to_remove = self.status().success_but_warnings().saturating_sub(10);
            let mut ok_to_remove = self.status().complete_success().saturating_sub(10);

            // TODO: it can happen that test count < 30 if not all categories occur

            let cases: Vec<TestCase> = std::mem::take(&mut self.cases);
            for case in cases.into_iter().rev() {
                match case.status() {
                    TestCaseStatus::Failure => {
                        if failures_to_remove == 0 {
                            self.cases.push(case);
                        } else {
                            failures_to_remove -= 1;
                        }
                    }
                    TestCaseStatus::SuccessButWarnings => {
                        if warnings_to_remove == 0 {
                            self.cases.push(case);
                        } else {
                            warnings_to_remove -= 1;
                        }
                    }
                    TestCaseStatus::CompleteSucess => {
                        if ok_to_remove == 0 {
                            self.cases.push(case);
                        } else {
                            ok_to_remove -= 1;
                        }
                    }
                }
            }
        }
    }

    pub fn create_with_title(title: impl Into<Text>) -> CategoryBuilder {
        CategoryBuilder {
            title: Some(title.into()),
            messages: Default::default(),
            description: None,
            cases: vec![],
        }
    }
    pub fn create_with_desc(description: impl Into<Text>) -> CategoryBuilder {
        CategoryBuilder {
            title: None,
            messages: Default::default(),
            description: Some(description.into()),
            cases: vec![],
        }
    }
    pub fn create() -> CategoryBuilder {
        CategoryBuilder {
            title: None,
            messages: Default::default(),
            description: None,
            cases: vec![],
        }
    }

    pub(crate) fn add_extra_messages(
        &mut self,
        report_msgs: &mut Messages<Report>,
        _simulation_msgs: &mut Messages<Simulation>,
    ) {
        for test in self.cases.iter_mut() {
            test.add_extra_messages(report_msgs);
        }
    }

    pub fn compute_from(
        title: Option<impl Into<Text>>,
        description: Option<impl Into<Text>>,
        messages: Messages<Category>,
        cases: Vec<TestCase>,
    ) -> Category {
        let description = description.map(Into::into);
        let title = title.map(Into::into);
        let (mut success, mut with_warn, mut fail) = (0, 0, 0);

        for case in cases.iter() {
            match case.status() {
                TestCaseStatus::Failure => fail += 1,
                TestCaseStatus::SuccessButWarnings => with_warn += 1,
                TestCaseStatus::CompleteSucess => success += 1,
            }
        }

        Category {
            title,
            description,
            status: CategoryStatus {
                complete_success: success,
                success_but_warnings: with_warn,
                failure: fail,
            },
            messages,
            cases,
        }
    }
}
