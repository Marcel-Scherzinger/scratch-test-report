use std::collections::BTreeSet;

use derive_more::{Eq, PartialEq};

use crate::messages::msg::Message;

pub trait MessageAdder<Level> {
    fn notify(&mut self, msg: Message<Level>);
    fn with_messages(&mut self, msgs: impl IntoIterator<Item = Message<Level>>) -> &mut Self {
        for msg in msgs {
            self.notify(msg);
        }
        self
    }
}

/// Collection of multiple messages of a level
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub struct Messages<Level> {
    messages: BTreeSet<Message<Level>>,
}

impl<Level> Messages<Level> {
    pub fn iter(&self) -> std::collections::btree_set::Iter<'_, Message<Level>> {
        self.messages.iter()
    }
}

impl<Level> MessageAdder<Level> for Messages<Level> {
    fn notify(&mut self, msg: Message<Level>) {
        self.messages.insert(msg);
    }
}

impl<Level> Extend<Message<Level>> for Messages<Level> {
    fn extend<T: IntoIterator<Item = Message<Level>>>(&mut self, iter: T) {
        for i in iter {
            self.messages.insert(i);
        }
    }
}

impl<Level> IntoIterator for Messages<Level> {
    type Item = Message<Level>;
    type IntoIter = <BTreeSet<Message<Level>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.messages.into_iter()
    }
}
