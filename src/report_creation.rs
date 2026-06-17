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
