use itertools::Itertools;

use crate::{
    merge_parts_of_level::MergePartsOfLevel,
    messages::{Message, MessageAdder, Messages},
    simulation::{Simulation, cases::RunAnalysis},
};

impl RunAnalysis {
    pub(crate) fn verbalize_uninitialized(&self, msgs: &mut Messages<Simulation>) {
        self.verbalize_uninitialized_lists(msgs);
        self.verbalize_uninitialized_variables(msgs);
    }

    fn verbalize_uninitialized_lists(&self, msgs: &mut Messages<Simulation>) -> bool {
        let mut lists = self
            .uninitialized_data()
            .iter()
            .flat_map(|e| e.as_ref().right().cloned())
            .peekable();
        if lists.peek().is_none() {
            return false;
        }
        let names = lists.map(|l| format!("{:?}", l.name())).join(", ");
        msgs.notify(Message::warning(format!("Ich habe erkannt, dass Sie mindestens eine Liste nutzen, bevor Sie sie initialisiert haben. ({names}) Listen werden darüber initialisiert, dass alle Elemente auf einmal mit dem dafür vorgesehenen Block gelöscht werden.")));
        true
    }
    fn verbalize_uninitialized_variables(&self, msgs: &mut Messages<Simulation>) -> bool {
        let mut variables = self
            .uninitialized_data()
            .iter()
            .flat_map(|e| e.as_ref().left().cloned())
            .peekable();
        if variables.peek().is_none() {
            return false;
        }
        let names = variables.map(|l| format!("{:?}", l.name())).join(", ");
        msgs.notify(Message::warning(format!("Ich habe erkannt, dass Sie mindestens eine Variable nutzen, bevor Sie sie initialisiert haben. ({names}) Sie sollten Variablen immer zuerst setzen, bevor Sie daraus lesen.")));
        true
    }
}

impl MergePartsOfLevel for RunAnalysis {
    fn merge_parts_ref(&mut self, other: Self) -> &mut Self {
        if self
            .hardcoding
            .as_ref()
            .is_none_or(|r| r.is_ok() && other.hardcoding.is_some())
        {
            self.hardcoding = other.hardcoding;
        }
        self.uninitialized_data
            .merge_parts_ref(other.uninitialized_data);
        self
    }
}
