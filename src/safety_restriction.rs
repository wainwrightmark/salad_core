use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIs, EnumString};

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumIs,
    Serialize,
    Deserialize,
    Display,
    EnumString,
    AsRefStr,
)]
pub enum SafetyRestriction {
    #[default]
    None,
    UnambiguousFirstLetters,
    Paper,
    PaperAndArizona,
}

impl SafetyRestriction {
    pub const fn has_paper_restriction(self) -> bool {
        self.is_paper() || self.is_paper_and_arizona()
    }
}
