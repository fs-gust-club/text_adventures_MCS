#![warn(missing_docs)]

//! The `world_building` module handles the types that
//! describe the world and its current state.

use itertools::Itertools;
use log_derive::{logfn, logfn_inputs};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::entities::item::{Item};
use crate::entities::interactive;

/// Holds the state of the world
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct World {
    /// The rooms making up the world, stored by Room.id
    pub locations: HashMap<String, Room>,
    /// The id of the current player's location
    pub player_location: String,
    pub player: Player,
}

impl World {
    /// Adds a location to the world
    pub fn add_location(&mut self, location: Room) {
        self.locations.insert(location.id.clone(), location);
    }

    /// Move the player to a new location
    ///
    /// # Arguments
    ///     
    /// * `direction` - A name that corresponds to the name of an exit from the
    /// players current location
    ///
    /// # Errors
    /// The location is invalid
    #[logfn(Info)]
    #[logfn_inputs(Info)]
    pub fn move_player(&mut self, direction: &String) -> Result<String, String> {
        // Check whether our current location has an exit that matches direction.
        // If so, set the payers location to the pointed direction.
        // returns a result with Ok(new_location), Err(No Exit)

        let new_room = self.locations
            .get(&self.player_location)
            .and_then(|cl| cl.exits.get(direction));

        match new_room {
            Some(room_id) => {
                self.player_location = room_id.clone();
                Ok(format!("You have moved {}", direction))
            }
            None => Err(format!("{} is not a valid direction", direction)),
        }
    }

    /// Gets a mutable reference to the room the player is currently in    
    pub fn get_player_room(&mut self) -> Option<&mut Room> {
        self.locations.get_mut(&self.player_location)
    }

    /// Save the state of the game to a local file
    ///
    /// # Errors
    /// Could not save the game
    #[logfn(Info)]
    pub fn save_state(&self) -> Result<String, String> {
        match serde_json::to_string(self) {
            Ok(json) => match fs::write("savedata.json", json) {
                Ok(_msg) => Ok("game saved".to_string()),
                Err(err) => {
                    error!("Error saving game {:?}", err);
                    Err("could not save game".to_string())
                }
            },
            Err(err) => {
                error!("Error serializing game state {:?}", err);
                Err("could not save game".to_string())
            }
        }
    }

    /// Load the state of the game from a local file
    ///
    /// # Errors
    /// Could not load the game
    #[logfn(Info)]
    pub fn load_state(&mut self) -> Result<String, String> {
        match fs::read_to_string("savedata.json") {
            Ok(contents) => match serde_json::from_str::<World>(&*contents) {
                Ok(new_world) => {
                    self.locations = new_world.locations;
                    self.player = new_world.player;
                    self.player_location = new_world.player_location;
                    Ok("game loaded".to_string())
                }
                Err(err) => {
                    error!("Error loading game {:?}", err);
                    Err("could not load game".to_string())
                }
            },
            Err(err) => {
                error!("Error deserializing game state {:?}", err);
                Err(format!("{:?}", err))
            }
        }
    }

    /// Take the specified item from the players current location
    ///  
    /// # Arguments
    ///
    /// * `item_name` - the name of the item to take
    pub fn take_item(&mut self, item_name: &str) -> Result<String, String> {
        match self.get_player_room() {
            Some(room) => {
                match room
                    .items
                    .iter()
                    .position(|i| i.name.to_lowercase() == item_name.to_lowercase())
                {
                    Some(index) => {
                        let item = room.items.remove(index);
                        self.player.inventory.push(item);
                        return Ok(format!("Picked up {}", item_name));
                    }
                    None => Err(format!("No item of type {} is present", item_name)),
                }
            }
            None => Err("Room does not exist".to_string()),
        }
    }


    pub fn use_item(&mut self, subject: &str, target: &str) -> Result<String, String> {
        match self.get_player_room() {
            Some(room) => {
                // ####### THIS ISNT FINISHED YET!!!!
                if room.has_feature(target) && self.player.has_item(subject) {
                    let mut feature = &room.features.get(target);
                    let item = &self.player.inventory.get(subject);
                    interactive::use_item(item, &mut feature, room);
                    Ok("Done".to_owned())
                } else {
                    Err("You cannot do that here".to_string())
                }
            }
            None => Err("Can't find the room the player is in.".to_string())
        }
        
    }
}

/// Describes an attribute of a room, which will eventually hold behaviour
#[derive(Debug, new, Serialize, Deserialize)]
pub struct Feature {
    name: String,
}

impl<'a> Feature {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn save_state(&self) -> Result<String, String> {
        match serde_json::to_string(self) {
            Ok(json) => match fs::write("savedata.json", json) {
                Ok(_msg) => Ok("game saved".to_string()),
                Err(err) => {
                    error!("Error saving game {:?}", err);
                    Err("could not save game".to_string())
                }
            },
            Err(err) => {
                error!("Error serializing game state {:?}", err);
                Err("could not save game".to_string())
            }
        }
    }

    pub fn load_state(&mut self) -> Result<String, String> {
        match fs::read_to_string("savedata.json") {
            Ok(contents) => match serde_json::from_str::<World>(&*contents) {
                Ok(new_world) => { 
                    self.locations = new_world.locations;
                    self.player = new_world.player;
                    self.player_location = new_world.player_location;
                    Ok("game loaded".to_string())
                },
                Err(err) => { 
                    error!("Error loading game {:?}", err);
                    Err("could not load game".to_string())
                }
            },
            Err(err) => { 
                error!("Error deserializing game state {:?}", err);
                Err(format!("{:?}", err))
            }
        }
    }
}

/// Describes a location and its contents
#[derive(Debug, new, Serialize, Deserialize)]
pub struct Room {
    /// Unique ID of the Room
    pub id: String,

    /// The description of the Room the player will see
    pub description: String,
    #[new(default)]
    exits: HashMap<String, String>,
    #[new(default)]
    items: Vec<Item>,
    #[new(default)]
    features: Vec<Feature>,
}

impl<'a> Room {
    /// Add an exit to the room
    ///
    /// # Arguments
    ///
    /// * `direction` - the name of the direction e.g. 'North'
    /// * `exit_id` - the name of the exit, must match the id of a Room.
    pub fn add_exit(&mut self, direction: String, exit_id: String) {
        self.exits.insert(direction, exit_id);
    }

    /// Gets the directions of all the exits
    pub fn get_exits(&self) -> impl Iterator<Item = &String> {
        self.exits.keys()
    }

    /// Determines if the direction is valid
    ///
    /// # Arguments
    ///
    /// * `direction` - the direction to check
    pub fn has_exit(&self, direction: &str) -> bool {
        let lower = direction.to_lowercase();
        self.exits.contains_key(&*lower)
    }

    /// Checks if the room has a feature
    pub fn has_feature(&self, feature_name: &str) -> bool {
        let lower = feature_name.to_lowercase();
        self.features.iter().any(|f| f.name == lower)
    }
    pub fn get_item_names(&self) -> impl Iterator<Item = String> + '_ {
        self.items.iter().map(|i| i.name.to_string())
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
        self.items.push(item);
    }

    /// Returns true if the room has any items, false otherwise.
    pub fn has_items(&self) -> bool {
        self.items.len() > 0
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
        match self.items.iter().position(|i| i.name == item_name) {
            Some(index) => {
                let temp = self.items.remove(index);
                player.inventory.push(temp);
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

#[derive(Debug, new, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
}


/// Create a `World`
macro_rules! shaper_of_worlds {
    (
        location = $player_location:expr,
        rooms = [
            $([
                $room_name:expr,
                $room_description:expr,
                items=[$($item:expr$(,)*)*],
                features=[$($feature:expr$(,)*)*],
                exits=[$($dir:expr => $dest:expr)*]
            ])+
        ]
    ) => {
        {
            let mut world = World::default();
            world.player_location = $player_location.to_lowercase().to_string();
            $(
                let mut room = Room::new($room_name.to_lowercase().to_string(), $room_description.to_string());
                $(
                    let item = crate::entities::item::Item::new($item.to_string());
                    room.add_item(item);
                )*
                $(
                    let feature = Feature::new($feature.to_string());
                    room.add_feature(feature);
                )*
                $(
                    room.add_exit($dir.to_string(), $dest.to_lowercase().to_string());
                )*
                world.add_location(room);
            )+
            world
        }
    };
}

#[cfg(test)]
#[path = "./world_building_tests.rs"]
mod world_building_tests;
}
