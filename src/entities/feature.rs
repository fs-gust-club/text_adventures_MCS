// use serde::{Deserialize, Serialize};
// use std::fs;

/// Describes an attribute of a room, which will eventually hold behaviour
#[derive(Debug, new)]
pub struct Feature {
    pub name: String,
}

impl<'a> Feature {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
