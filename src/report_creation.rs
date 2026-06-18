use std::{collections::BTreeMap, fmt::Debug, ops::Deref, sync::Arc};

use itertools::Itertools;
use smodel::{
    ProjectDoc,
    blocks::{BlockKindUnit, EventBlockKindUnit},
};

use crate::report::{Formality, InitialBlockAmbiguity, MaxBlocksExceeded};

const GREEN_FLAG: BlockKindUnit = BlockKindUnit::Event(EventBlockKindUnit::EventWhenflagclicked);

pub trait ReportGenerator {
    fn create(&self, doc: &ProjectDoc) -> crate::report::Report {
        let (initial_block, mut form) = self.create_formality(doc);
        if let Some(initial_block) = initial_block
            && !form.cyclic_graph()
        {
            let simulation = self.create_simulation(&mut form, initial_block, doc);
            crate::report::Report::new(Some(form), Some(simulation))
        } else {
            crate::report::Report::new(Some(form), None)
        }
    }

    fn create_simulation(
        &self,
        form: &mut Formality,
        initial_block: &smodel::Id,
        doc: &ProjectDoc,
    ) -> crate::simulation::Simulation;

    fn max_allowed_total_block_number(&self) -> Option<usize> {
        None
    }

    fn form_check_max_total_blocks(&self, doc: &ProjectDoc) -> Result<(), MaxBlocksExceeded> {
        if let Some(allowed) = self.max_allowed_total_block_number() {
            let used = doc.targets().iter().flat_map(|t| t.blocks().iter()).count();
            if used > allowed {
                Err(MaxBlocksExceeded::new(used, allowed))
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    fn create_formality<'a>(&self, doc: &'a ProjectDoc) -> (Option<&'a smodel::Id>, Formality) {
        let initial_block = self.form_find_initial_block(doc);
        let cyclic_graph = self.form_is_cyclic_graph(doc);
        let block_ref = initial_block.clone().ok();
        let max_total_blocks = self.form_check_max_total_blocks(doc);
        (
            block_ref,
            Formality::new(initial_block.cloned(), cyclic_graph, max_total_blocks),
        )
    }

    fn form_is_cyclic_graph(&self, doc: &ProjectDoc) -> bool {
        if let Err(err) = sgraph::BlockGraph::new(doc).check_no_cycles_in_next_or_param_edges() {
            log::warn!("[report/form] submission has cyclic graph: {err}");
            true
        } else {
            false
        }
    }

    fn form_is_initial_block(&self, opcode: &BlockKindUnit) -> bool {
        opcode == &GREEN_FLAG
    }

    fn form_find_initial_block<'a>(
        &self,
        doc: &'a ProjectDoc,
    ) -> Result<&'a smodel::Id, InitialBlockAmbiguity> {
        let mut green_flags = doc
            .ids_with_opcodes()
            .filter_map(|(id, opcode)| self.form_is_initial_block(&opcode).then_some(id));
        let first = green_flags.next();
        if green_flags.next().is_some() {
            Err(InitialBlockAmbiguity::Multiple)
        } else {
            first.ok_or(InitialBlockAmbiguity::No)
        }
    }
}

#[derive(Default)]
pub struct Exercises(BTreeMap<String, Arc<dyn ReportGenerator + Send + Sync>>);

impl Exercises {
    pub fn insert<T: ReportGenerator + 'static + Send + Sync>(
        &mut self,
        identifier: impl Into<String>,
        generator: T,
    ) -> &mut Self {
        self.0.insert(identifier.into(), Arc::new(generator));
        self
    }

    pub fn with<T: ReportGenerator + 'static + Send + Sync>(
        mut self,
        identifier: impl Into<String>,
        generator: T,
    ) -> Self {
        self.insert(identifier, generator);
        self
    }
    pub fn get(&self, identifier: &str) -> Option<Arc<dyn ReportGenerator + Send + Sync>> {
        self.0.get(identifier).cloned()
    }
    pub fn get_ref(&self, identifier: &str) -> Option<&(dyn ReportGenerator + Send + Sync)> {
        self.0.get(identifier).map(|a| &**a)
    }
    pub fn keys(&self) -> impl ExactSizeIterator<Item = &str> {
        self.0.keys().map(|x| x.deref())
    }
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = (&str, &Arc<dyn ReportGenerator + Send + Sync>)> {
        self.0.iter().map(|(x, y)| (x.deref(), y))
    }
}

impl Debug for Exercises {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Exercises {{ {} }}",
            self.0.keys().map(|a| format!("{a:?}: ...")).join(", ")
        ))
    }
}

impl IntoIterator for Exercises {
    type Item = (String, Arc<dyn ReportGenerator + Send + Sync>);
    type IntoIter =
        std::collections::btree_map::IntoIter<String, Arc<dyn ReportGenerator + Send + Sync>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<S: Into<String>> FromIterator<(S, Arc<dyn ReportGenerator + Send + Sync>)> for Exercises {
    fn from_iter<T: IntoIterator<Item = (S, Arc<dyn ReportGenerator + Send + Sync>)>>(
        iter: T,
    ) -> Self {
        let x = iter.into_iter().map(|(k, v)| (k.into(), v));
        Self(x.collect())
    }
}
