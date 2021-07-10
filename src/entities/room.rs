use itertools::Itertools;
// use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

use crate::entities::{Feature, Item, Player};

/// Describes a location and its contents
#[derive(Debug, new)]
pub struct Room<'a> {
    /// Unique ID of the Room
    pub id: String,

    /// The description of the Room the player will see
    pub description: String,
    #[new(default)]
    pub exits: RefCell<HashMap<String, &'a Room<'a>>>,
    #[new(default)]
    pub items: RefCell<Vec<Item>>,
    #[new(default)]
    pub features: Vec<Feature>,
}

impl<'a> Room<'a> {
    /// Add an exit to the room
    ///
    /// # Arguments
    ///
    /// * `direction` - the name of the direction e.g. 'North'
    /// * `exit_id` - the name of the exit, must match the id of a Room.
    pub fn add_exit(&self, direction: String, exit: &'a Room<'a>) {
        self.exits.borrow_mut().insert(direction, exit);
    }

    /// Gets the directions of all the exits
    pub fn get_exits(&self) -> impl Iterator<Item = String> {
        self.exits
            .borrow()
            .keys()
            .map(|key| key.clone())
            .collect::<Vec<String>>()
            .into_iter()
    }

    /// Determines if the direction is valid
    ///
    /// # Arguments
    ///
    /// * `direction` - the direction to check
    pub fn has_exit(&self, direction: &str) -> bool {
        let lower = direction.to_lowercase();
        self.exits.borrow().contains_key(&*lower)
    }

    /// Checks if the room has a feature
    pub fn has_feature(&self, feature_name: &str) -> bool {
        let lower = feature_name.to_lowercase();
        self.features.iter().any(|f| f.name == lower)
    }
    pub fn get_item_names(&self) -> impl Iterator<Item = String> {
        self.items.borrow().iter().map(|i| i.name.to_string()).collect::<Vec<String>>().into_iter()
    }

    pub fn get_feature_names(&self) -> impl Iterator<Item = String> + '_ {
        self.features.iter().map(|i| i.name.to_string())
    }

    /// Adds the item to the room's inventory
    ///
    /// # Arguments
    ///
    /// * `item` - the item to add
    pub fn add_item(&mut self, item: Item) {
        self.items.borrow_mut().push(item);
    }

    /// Returns true if the room has any items, false otherwise.
    pub fn has_items(&self) -> bool {
        self.items.borrow().len() > 0
    }

    /// Creates a complete description of the location including
    /// exits and items.
    pub fn get_full_description(&self) -> String {
        let mut output = String::new();
        let description = format!(
            "\n{location_description}",
            location_description = self.description
        );
        output.push_str(&description);

        let exits = format!("\nExits are {exits}", exits = self.get_exits().join(", "));
        output.push_str(&exits);

        if self.features.len() > 0 {
            let features = format!(
                "\nThere is {feature_names}",
                feature_names = self.get_feature_names().join(", ")
            );
            output.push_str(&features);
        }

        if self.has_items() {
            let items = format!(
                "\nItems are {item_names}",
                item_names = self.get_item_names().join(", ")
            );
            output.push_str(&items);
        }

        output
    }

    /// Takes an item from the room and adds it the players inventory
    ///
    /// # Arguments
    ///
    /// * `player` - The player who will receive the item
    /// * `item_name` - The name of the item to be take
    ///
    /// # Errors
    /// The item does not exist in the player's current location
    pub fn take_item(&mut self, player: &mut Player, item_name: String) -> Result<String, String> {
        match self.items.borrow().iter().position(|i| i.name == item_name) {
            Some(index) => {
                let temp = self.items.borrow_mut().remove(index);
                player.inventory.borrow_mut().push(temp);
                Ok(format!("Picked up {}", item_name))
            }
            None => Err(format!("No item of type {} is present", item_name)),
        }
    }

    pub fn add_feature(&mut self, feature: Feature) {
        self.features.push(feature);
    }

    pub fn remove_feature(&mut self, feature_name: String) -> Result<String, String> {
        match self.features.iter().position(|i| i.name == feature_name) {
            Some(index) => {
                self.features.remove(index);
                Ok(feature_name)
            }
            None => Err(format!("No feature of type {} is present", feature_name)),
        }
    }
}
