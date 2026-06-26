use std::collections::BTreeSet;

use derive_more::{Eq, PartialEq};

use crate::{
    merge_parts_of_level::MergePartsOfLevel,
    messages::{MessageKind, msg::Message},
};

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
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Messages<Level>(BTreeSet<Message<Level>>);

impl<Level> Messages<Level> {
    pub fn has_kind(&self, kind: &MessageKind) -> bool {
        self.iter().any(|m| m.kind() == kind)
    }
}

impl<Level> Default for Messages<Level> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<Level> Messages<Level> {
    pub fn iter(&self) -> std::collections::btree_set::Iter<'_, Message<Level>> {
        self.0.iter()
    }
}

impl<Level> MessageAdder<Level> for Messages<Level> {
    fn notify(&mut self, msg: Message<Level>) {
        self.0.insert(msg);
    }
}

impl<Level> Extend<Message<Level>> for Messages<Level> {
    fn extend<T: IntoIterator<Item = Message<Level>>>(&mut self, iter: T) {
        for i in iter {
            self.0.insert(i);
        }
    }
}

impl<Level> FromIterator<Message<Level>> for Messages<Level> {
    fn from_iter<T: IntoIterator<Item = Message<Level>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<Level> IntoIterator for Messages<Level> {
    type Item = Message<Level>;
    type IntoIter = <BTreeSet<Message<Level>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<Level> MergePartsOfLevel for Messages<Level> {
    fn merge_parts_ref(&mut self, other: Self) -> &mut Self {
        self.extend(other);
        self
    }
}
