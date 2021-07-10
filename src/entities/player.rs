// use serde::{Deserialize, Serialize};
use std::cell::RefCell;

use crate::entities::Item;

/// Represents a player
#[derive(Default, Debug, new)]
pub struct Player {
    /// The player name
    pub name: String,

    /// The items the player is carrying
    #[new(default)]
    pub inventory: RefCell<Vec<Item>>,
}

/// Lists the inventory currently carried by the player
impl Player {
    /// List the items carried by the player
    pub fn list_inventory(&self) -> String {
        self.inventory.borrow().iter().fold(String::new(), |mut agg, item| {
            agg.push_str(&*item.name);
            agg.push_str("\n");
            agg
        })
    }

    /// Checks if the player has an item of that name
    pub fn has_item(&self, item_name: &str) -> bool {
        let lower = item_name.to_lowercase();
        self.inventory.borrow().iter().any(|i| i.name == lower)
    }
}
