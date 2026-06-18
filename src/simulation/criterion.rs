use crate::Text;

/// Encodes what a test checked and what is the difference between expected and
/// received results
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case", tag = "type"))]
#[allow(clippy::enum_variant_names)]
pub enum TestCriterion {
    /// The last line the program output should have an `expected` value.
    /// Any other (slightly different) value will be counted as failure.
    #[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
    LastOutputExact {
        expected: Text,
        output_matches: Option<bool>,
    },
    /// The last line the program output encodes a decision or value it was asked
    /// for. This value could be formatted in different ways so a special layer
    /// was used to extract the needed parts and interpret what the decision is.
    /// This layer could guess wrongly.
    #[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
    LastOutputInterpreted {
        /// One example that would have leaded to the expected interpretation.
        /// This can be shown to the user for comparison
        sample_expected: Text,
        /// The expected interpretation (decision/value)
        iexpected: Text,
        /// The interpretation of the program output, `None` if there was no output
        ireceived: Option<Text>,
        interpretations_match: Option<bool>,
    },
    /// The last line the program output should contain an `expected` value.
    /// Any other value not containing this will be counted as failure.
    #[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
    LastOutputContains {
        /// One example that contains the expected content.
        /// This can be shown to the user for comparison
        sample_expected: Text,
        /// An expected string part
        expected: Text,
        /// If the received output contains the expected string.
        /// None if there is no output
        received_contains: Option<bool>,
    },
}

impl TestCriterion {
    pub fn is_successful(&self) -> bool {
        match self {
            Self::LastOutputExact {
                output_matches: happy,
                ..
            }
            | Self::LastOutputContains {
                received_contains: happy,
                ..
            }
            | Self::LastOutputInterpreted {
                interpretations_match: happy,
                ..
            } => happy.unwrap_or(false),
        }
    }
}
