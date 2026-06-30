use itertools::Itertools;

use crate::{
    SimulationBLevelMessages, Text,
    merge_parts_of_level::MergePartsOfLevel,
    simulation::{Category, CategoryBuilder, FinishedCaseBuilder, cases::RunAnalysis},
};

impl CategoryBuilder {
    pub fn title(&mut self, title: impl Into<Text>) -> &mut Self {
        self.title = Some(title.into());
        self
    }
    pub fn description(&mut self, description: impl Into<Text>) -> &mut Self {
        self.description = Some(description.into());
        self
    }
    pub fn add_case(&mut self, case: FinishedCaseBuilder) -> &mut Self {
        self.cases.push(case);
        self
    }
    pub fn build(self) -> (Category, SimulationBLevelMessages, Option<RunAnalysis>) {
        let (cases, messages, extras): (_, Vec<_>, Vec<_>) =
            self.cases.into_iter().map(|t| t.build()).multiunzip();

        let messages = messages
            .into_iter()
            .reduce(MergePartsOfLevel::merge_parts)
            .unwrap_or_default();
        let cat = Category::compute_from(self.title, self.description, messages.0, cases);

        let extra = extras.into_iter().reduce(MergePartsOfLevel::merge_parts);
        (cat, messages.1, extra)
    }
}
