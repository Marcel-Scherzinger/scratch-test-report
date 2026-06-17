mod merge_parts_of_level;
pub mod messages;
pub mod report;
mod report_creation;
pub mod simulation;

pub use report_creation::ReportGenerator;

pub type Text = std::borrow::Cow<'static, str>;

pub mod prelude {
    // make `notify` method available without extra import
    pub use crate::messages::MessageAdder as _;
}

#[allow(unused)]
/// This is used by `serde` for checking if values should be
/// serialized. This is not detected but still used and needed.
fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}
