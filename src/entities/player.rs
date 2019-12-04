use serde::{Deserialize, Serialize};

use crate::entities::Item;

/// Represents a player
#[derive(Default, Debug, new, Serialize, Deserialize)]
pub struct Player {
    /// The player name
    pub name: String,

    /// The items the player is carrying
    #[new(default)]
    pub inventory: Vec<Item>,
}

/// Lists the inventory currently carried by the player
impl Player {
    /// List the items carried by the player
    pub fn list_inventory(&self) -> String {
        self.inventory.iter().fold(String::new(), |mut agg, item| {
            agg.push_str(&*item.name);
            agg.push_str("\n");
            agg
        })
    }

    /// Checks if the player has an item of that name
    pub fn has_item(&self, item_name: &str) -> bool {
        let lower = item_name.to_lowercase();
        self.inventory.iter().any(|i| i.name == lower)
    }
}
