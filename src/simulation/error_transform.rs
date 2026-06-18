use sinterpreter::error;
use sinterpreter::{RunError, default_state::DefaultStateError};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RunningError {
    // tell that something is wrong with file, log details
    File(error::InvalidFileError),
    User(error::UserError),
    State(DefaultStateError),
    Limit(error::LimitError),
    Internal(error::InternalError),
    Unsupported(error::UnsupportedError),
}

// TODO: think about granularity

impl RunningError {
    pub(crate) fn from_interpreter_error(value: RunError<DefaultStateError>) -> Option<Self> {
        Some(match value {
            RunError::File(err) => Self::File(err),
            RunError::User(err) => Self::User(err),
            RunError::State(err) => Self::State(err),
            RunError::Limit(err) => Self::Limit(err),
            RunError::Internal(err) => Self::Internal(err),
            RunError::Unsupported(err) => Self::Unsupported(err),
            RunError::TerminatedByControlStop => return None,
        })
    }
}
