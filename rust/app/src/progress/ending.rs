use serde::{Deserialize, Serialize};

/// How the true ending went, once it has been reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Ending {
    /// The wound was closed: the portal sleeps, the Scientist is silent.
    Closed,
    /// It was left open, for the Scientist's work.
    LeftOpen,
}
