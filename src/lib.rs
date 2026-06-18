mod merge_parts_of_level;
pub mod messages;
pub mod report;
mod report_creation;
pub mod simulation;

pub use report_creation::Exercises;
pub use report_creation::ReportGenerator;

use crate::{
    messages::Messages,
    simulation::{Category, Simulation, TestCase},
};

pub type Text = std::borrow::Cow<'static, str>;

pub mod prelude {
    // make `notify` method available without extra import
    pub use crate::ReportGenerator;
    pub use crate::messages::Message;
    pub use crate::messages::MessageAdder as _;
    pub use crate::report::{Formality, Report};
    pub use crate::simulation::*;
}

#[allow(unused)]
/// This is used by `serde` for checking if values should be
/// serialized. This is not detected but still used and needed.
fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

pub(crate) type SimulationBLevelMessages = Messages<Simulation>;
pub(crate) type CategoryBLevelMessages = (Messages<Category>, SimulationBLevelMessages);
pub(crate) type CaseLevelBMessages = (Messages<TestCase>, CategoryBLevelMessages);
