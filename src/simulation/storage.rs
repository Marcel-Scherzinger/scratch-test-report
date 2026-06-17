use std::{borrow::Cow, collections::BTreeMap};

use derive_getters::Getters;
use svalue::SValue;

use crate::Text;

#[derive(Debug, PartialEq, PartialOrd, Clone, Getters, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActionsState {
    output: Vec<SValue>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::is_default")
    )]
    data: DataStorage,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Getters, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DataStorage {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::is_default")
    )]
    lists: BTreeMap<Text, Vec<SValue>>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::is_default")
    )]
    variables: BTreeMap<Text, SValue>,
}

impl ActionsState {
    pub fn new_output(output: Vec<SValue>) -> Self {
        Self {
            output,
            data: DataStorage::default(),
        }
    }
    pub fn new_output_from(output: impl Into<Vec<SValue>>) -> Self {
        Self {
            output: output.into(),
            data: Default::default(),
        }
    }

    pub fn output_mut(&mut self) -> &mut Vec<SValue> {
        &mut self.output
    }
    pub fn data_mut(&mut self) -> &mut DataStorage {
        &mut self.data
    }

    pub fn last_output<'a>(&'a self) -> Option<Cow<'a, str>> {
        self.output().last().map(|x| x.as_text())
    }

    pub fn from_parts(
        output: Vec<SValue>,
        lists: BTreeMap<Text, Vec<SValue>>,
        variables: BTreeMap<Text, SValue>,
    ) -> Self {
        Self {
            output,
            data: DataStorage::new(variables, lists),
        }
    }
}

impl FromIterator<SValue> for ActionsState {
    fn from_iter<T: IntoIterator<Item = SValue>>(iter: T) -> Self {
        Self::new_output(iter.into_iter().collect())
    }
}

impl DataStorage {
    pub fn new(variables: BTreeMap<Text, SValue>, lists: BTreeMap<Text, Vec<SValue>>) -> Self {
        Self { lists, variables }
    }
    pub fn lists_mut(&mut self) -> &mut BTreeMap<Text, Vec<SValue>> {
        &mut self.lists
    }
    pub fn variables_mut(&mut self) -> &mut BTreeMap<Text, SValue> {
        &mut self.variables
    }
}
