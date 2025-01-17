use super::super::Repetition;

#[cfg(feature = "serde_types")]
use serde_derive::{Deserialize, Serialize};

/// Common type information.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde_types", derive(Deserialize, Serialize))]
pub struct FieldInfo {
    /// The field name
    pub name: String,
    /// The repetition
    pub repetition: Repetition,
    /// the optional id, to select fields by id
    pub id: Option<i32>,
}
