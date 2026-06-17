use crate::{
    Text,
    messages::Messages,
    simulation::{Category, CategoryBuilder, CategoryStatus, TestCase, TestCaseStatus},
};

impl Category {
    pub fn create_with_desc(description: impl Into<Text>) -> CategoryBuilder {
        CategoryBuilder {
            messages: Default::default(),
            description: Some(description.into()),
            cases: vec![],
        }
    }
    pub fn create() -> CategoryBuilder {
        CategoryBuilder {
            messages: Default::default(),
            description: None,
            cases: vec![],
        }
    }

    pub fn compute_from(
        description: Option<impl Into<Text>>,
        messages: Messages<Category>,
        cases: Vec<TestCase>,
    ) -> Category {
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
            messages,
            cases,
        }
    }
}
