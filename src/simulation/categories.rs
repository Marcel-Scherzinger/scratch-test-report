mod category;
mod category_builder;
mod implementations;

use derive_getters::Getters;

use crate::{
    CategoryBLevelMessages, Text,
    messages::Messages,
    simulation::{CategoryStatus, FinishedCaseBuilder, TestCase},
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Getters)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub struct Category {
    /// An optional _short_ title describing the category
    title: Option<Text>,
    /// An optional description giving a hint on what the contained
    /// test cases have in common i. e. what property they test
    description: Option<Text>,
    status: CategoryStatus,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::is_default")
    )]
    messages: Messages<Category>,
    /// All test cases of this category
    cases: Vec<TestCase>,
    // TODO: Is this needed?
    // /// Random numbers specified here were provided to every test case
    // /// in this category. This is mostly for saving bandwidth if a category
    // /// executes a test multiple times with different inputs but the same randoms
    // randoms: Option<Vec<svalue::SNumber>>,
}
pub struct CategoryBuilder {
    title: Option<Text>,
    messages: CategoryBLevelMessages,
    description: Option<Text>,
    cases: Vec<FinishedCaseBuilder>,
}
