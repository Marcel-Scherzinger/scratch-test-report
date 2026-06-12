pub mod messages;
pub mod report;
pub mod simulation;

pub type Text = std::borrow::Cow<'static, str>;

pub mod prelude {
    // make `notify` method available without extra import
    pub use crate::messages::MessageAdder as _;
}
