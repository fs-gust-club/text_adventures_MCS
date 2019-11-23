#![warn(missing_docs)]

//! The `world_building` module handles the types that
//! describe the world and its current state.

use log_derive::{logfn, logfn_inputs};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::entities::{interactive, Player, Room};

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

        let new_room = self
            .locations
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
            Ok(contents) => match serde_json::from_str::<World>(&contents) {
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
            None => Err("Can't find the room the player is in.".to_string()),
        }
    }
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
                let mut room = crate::entities::Room::new($room_name.to_lowercase().to_string(), $room_description.to_string());
                $(
                    let item = crate::entities::Item::new($item.to_string());
                    room.add_item(item);
                )*
                $(
                    let feature = crate::entities::Feature::new($feature.to_string());
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
