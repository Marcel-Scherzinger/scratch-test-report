mod merge_parts_of_level;
pub mod messages;
pub mod report;
mod report_creation;
pub mod simulation;
pub mod utils;

use derive_more::Debug;
use derive_more::Deref;
use derive_more::Display;
use derive_more::From;
use derive_more::Into;
pub use report_creation::Exercises;
pub use report_creation::ReportGenerator;

use crate::{
    messages::Messages,
    simulation::{Category, Simulation, TestCase},
};

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Deref, From, Into)]
#[debug("{_0:?}")]
#[display("{_0}")]
pub struct Text(std::borrow::Cow<'static, str>);

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl From<&'static str> for Text {
    fn from(value: &'static str) -> Self {
        Self(value.into())
    }
}

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
