// use serde::{Deserialize, Serialize};

/// An item
#[derive(Debug, new)]
pub struct Item {
    /// The item name
    pub name: String,
}
