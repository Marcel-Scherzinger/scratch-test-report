use crate::{
    messages::MessageAdder,
    simulation::{Category, CategoryBuilder, FinishedCaseBuilder, Simulation},
};

impl MessageAdder<Category> for CategoryBuilder {
    fn notify(&mut self, msg: crate::messages::Message<Category>) {
        self.messages.0.notify(msg);
    }
}
impl MessageAdder<Simulation> for CategoryBuilder {
    fn notify(&mut self, msg: crate::messages::Message<Simulation>) {
        self.messages.1.notify(msg);
    }
}
impl Extend<FinishedCaseBuilder> for CategoryBuilder {
    fn extend<T: IntoIterator<Item = FinishedCaseBuilder>>(&mut self, iter: T) {
        for i in iter {
            self.add_case(i);
        }
    }
}

impl FromIterator<FinishedCaseBuilder> for CategoryBuilder {
    fn from_iter<T: IntoIterator<Item = FinishedCaseBuilder>>(iter: T) -> Self {
        Self {
            title: None,
            messages: Default::default(),
            description: None,
            cases: iter.into_iter().collect(),
        }
    }
}
