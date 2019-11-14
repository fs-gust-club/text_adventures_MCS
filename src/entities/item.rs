use serde::{Deserialize, Serialize};

/// An item
#[derive(Debug, new, Serialize, Deserialize)]
pub struct Item {
    /// The item name
    pub name: String,
}