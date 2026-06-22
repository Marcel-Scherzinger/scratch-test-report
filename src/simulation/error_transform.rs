use sinterpreter::error;
use sinterpreter::{RunError, default_state::DefaultStateError};

use crate::prelude::{Message, TestCase};
use crate::report::Report;

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
    pub fn to_message(&self) -> RunningErrorMessage {
        match self {
            Self::File(error) => {
                log::info!("[report/form] Submission of invalid file: {error:?}");
                RunningErrorMessage::Report(Message::error(format!(
                    "Irgendetwas stimmt nicht mit der von Ihnen abgegebenen Datei. Sie ist in sich seltsam. Der technische Fehler hierzu (falls Sie die Aufgabensteller informieren wollen) lautet: {error}"
                )))
            }
            Self::Internal(error) => {
                log::error!("[report/critical] Error in interpreter: {error:?}");
                RunningErrorMessage::Report(Message::error(format!(
                    "Ich glaube Ihr Programm hat einen Fehler in mir gefunden, der noch keinem vorher aufgefallen ist. Bitte informieren Sie die Aufgabensteller. Der technische Fehler hierzu lautet: {error}"
                )))
            }
            Self::Unsupported(error) => {
                log::info!("[report/form] Usage of unsupported block: {error:?}");
                RunningErrorMessage::Report(Message::error(format!(
                    "Ihr Programm nutzt Blöcke, die ich nicht verstehe. Das bedeutet sehr wahrscheinlich, dass Sie sie nicht für die Aufgabe benötigen. Sie müssen sie entfernen, damit ich Ihr Programm verarbeiten kann. Der technische Fehler hierzu lautet: {error}"
                )))
            }
            Self::Limit(error) => {
                log::info!("[report/user] Limitations exceeded: {error:?}");
                RunningErrorMessage::Case(match error {
                    error::LimitError::StringExceededLengthLimit(_) => Message::error(
                        "Ihr Programm hat die erlaubten Ressourcen überschritten. Es hat zu lange Texte erzeugt. Um mich selbst zu schützen, musste ich leider aufhören, Ihr Programm auszuführen.",
                    ),
                    _ => Message::error(
                        "Ihr Programm hat die erlaubten Ressourcen überschritten. Das könnte bedeuten, dass Sie eine Endlosschleife programmiert haben oder anderweitig unnötige Befehle ausführen. Um mich selbst zu schützen, musste ich leider aufhören, Ihr Programm auszuführen.",
                    ),
                })
            }
            Self::User(error) => RunningErrorMessage::Report(match error {
                error::UserError::InfiniteLoopWithoutBodyNeverStops => Message::error(
                    "Sie haben einen Endlosschleifen-Block benutzt, aber keinen Schleifenkörper hinzugefügt. Somit kann nie etwas passieren, was die Schleife abbricht. Ihr Programm würde also ewig laufen ohne etwas zu tun, deshalb habe ich es abgebrochen.",
                ),
                error::UserError::ConditionLoopWithoutBodyNeverStops => Message::error(
                    "Sie haben einen Schleifenblock mit Bedingung benutzt, aber keinen Schleifenkörper hinzugefügt. Somit kann nie etwas passieren, was die Schleifenbedingung falsch werden lässt. Ihr Programm würde also ewig laufen ohne etwas zu tun, deshalb habe ich es abgebrochen.",
                ),
                error::UserError::WaitUntilFalseNeverTerminatesInSingleThreadded => Message::error(
                    "Sie haben einen Warte-bis-Block benutzt, die Bedingung auf die gewartet wird ist jedoch falsch. Da gewartet wird und kein anderer Programmteil läuft kann nie etwas passieren, was die Bedingnung wahr werden lässt. Ihr Programm würde also ewig laufen ohne etwas zu tun, deshalb habe ich es abgebrochen.",
                ),
            }),
            Self::State(error) => match error {
                DefaultStateError::NoMoreAnswers => RunningErrorMessage::Case(Message::error(
                    "Ihr Programm hat mehr Fragen gestellt, als die Fragensteller für diese Aufgabe Antworten zur Verfügung stellen. Vielleicht haben Sie die Aufgabenstellung falsch verstanden?",
                )),
                DefaultStateError::RandomsDisabled => RunningErrorMessage::Report(Message::error(
                    "Ihr Programm hat um eine Zufallszahl gebeten, doch die Aufgabensteller haben diese Funktion für diese Aufgabe deaktiviert. Sie müssen also ohne Zufallszahlen auskommen.",
                )),
                DefaultStateError::ListNotFound(id) => {
                    RunningErrorMessage::Report(Message::error(format!(
                        "Ihr Programm wollte eine Liste namens {:?} benutzen, die gibt es aber nicht. Das bedeutet sehr wahrscheinlich, dass die Aufgabensteller bestimmte Listennamen erzwingen (siehe Aufgabenstellung) und Sie davon abgewichen sind.",
                        id.name()
                    )))
                }
                DefaultStateError::VariableNotFound(id) => {
                    RunningErrorMessage::Report(Message::error(format!(
                        "Ihr Programm wollte eine Variable namens {:?} benutzen, die gibt es aber nicht. Das bedeutet sehr wahrscheinlich, dass die Aufgabensteller bestimmte Variablennamen erzwingen (siehe Aufgabenstellung) und Sie davon abgewichen sind.",
                        id.name()
                    )))
                }
                DefaultStateError::ListFull { id } => {
                    log::info!("[report/user] Limitations exceeded: ListFull {{ id: {id:?} }}");
                    RunningErrorMessage::Case(Message::error(format!(
                        "Ihr Programm hat die erlaubten Ressourcen überschritten. Es hat zu lange Listen erzeugt (im Speziellen ist die Liste {:?} zu lang geworden). Um mich selbst zu schützen, musste ich leider aufhören, Ihr Programm auszuführen.",
                        id.name(),
                    )))
                }
            },
        }
    }
}

pub enum RunningErrorMessage {
    Report(Message<Report>),
    Case(Message<TestCase>),
}
