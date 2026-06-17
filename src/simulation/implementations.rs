use crate::{merge_parts_of_level::MergePartsOfLevel, messages::MessageAdder};

use super::*;

impl Simulation {
    pub fn new() -> Self {
        Self {
            messages: Default::default(),
            categories: Default::default(),
            analysis: Default::default(),
        }
    }
    pub fn add_category(&mut self, category_builder: CategoryBuilder) -> &mut Self {
        let (category, messages, extra) = category_builder.build();
        self.messages.merge_parts_ref(messages);
        if let Some(analysis) = extra {
            self.analysis.merge_parts_ref(analysis);
        }
        self.categories.push(category);
        self
    }
    pub fn with_category(mut self, category_builder: CategoryBuilder) -> Self {
        self.add_category(category_builder);
        self
    }
    pub fn verbalize(&mut self) {
        self.analysis.verbalize_uninitialized(&mut self.messages);
    }
}

impl MessageAdder<Simulation> for Simulation {
    fn notify(&mut self, msg: crate::messages::Message<Simulation>) {
        self.messages.notify(msg);
    }
}

impl<'a, I: Iterator<Item = (&'a sinterpreter::OutputKind, &'a svalue::SValue)>> From<I>
    for ActionsState
{
    fn from(value: I) -> Self {
        Self::new_output(value.map(|(_, o)| o.clone()).collect())
    }
}

impl<'a> FromIterator<(&'a sinterpreter::OutputKind, &'a svalue::SValue)> for ActionsState {
    fn from_iter<T: IntoIterator<Item = (&'a sinterpreter::OutputKind, &'a svalue::SValue)>>(
        iter: T,
    ) -> Self {
        Self::from(iter.into_iter())
    }
}
