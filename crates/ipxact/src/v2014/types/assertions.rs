//! Assertions container type

use serde::{Deserialize, Serialize};

use super::assertion::Assertion;

/// Container for assertions
///
/// Maps to XML schema `assertionsType` complex type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Assertions {
    /// List of assertions
    #[serde(rename = "assertion", default)]
    pub assertion: Vec<Assertion>,
}

impl Assertions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, assertion: Assertion) {
        self.assertion.push(assertion);
    }
}
