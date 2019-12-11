#![warn(missing_docs)]

//! The `world_building` module handles the types that
//! describe the world and its current state.

// use log_derive::{logfn, logfn_inputs};
use log_derive::logfn;
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use std::fs;
// use std::cell::RefCell;

// use crate::entities::{interactive, Player, Room};
use crate::entities::{Player, Room};

// #[derive(Default, Debug)]
#[derive(Debug)]
pub struct World<'a> {
    pub player_location: &'a Room<'a>,
    pub player: Player,
}

impl<'a> World<'a> {
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
    // #[logfn_inputs(Info)]
    pub fn move_player(&mut self, direction: &String) -> Result<String, String> {
        // Check whether our current location has an exit that matches direction.
        // If so, set the payers location to the pointed direction.
        // returns a result with Ok(new_location), Err(No Exit)

        let x = self.player_location.exits.borrow(); // assign here so that borrowed hashmap won't go OOS before end of function
        let new_room = x.get(direction);

        match new_room {
            Some(room) => {
                self.player_location = room;
                Ok(format!("You have moved {}", direction))
            }
            None => Err(format!("{} is not a valid direction", direction)),
        }
    }

    /// Save the state of the game to a local file
    ///
    /// # Errors
    /// Could not save the game
    #[logfn(Info)]
    pub fn save_state(&self) -> Result<String, String> {
        /* match serde_json::to_string(self) {
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
        }*/
        Err("Save not implimented".to_string())
    }

    /// Load the state of the game from a local file
    ///
    /// # Errors
    /// Could not load the game
    //#[logfn(Info)]
    pub fn load_state(&mut self) -> Result<String, String> {
        /*match fs::read_to_string("savedata.json") {
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
        }*/
        Err("Load not implemented".to_owned())
    }

    /// Take the specified item from the players current location
    ///
    /// # Arguments
    ///
    /// * `item_name` - the name of the item to take
    pub fn take_item(&self, item_name: &str) -> Result<String, String> {
        let mut items = self.player_location.items.borrow_mut();

        match items
            .iter()
            .position(|i| i.name.to_lowercase() == item_name.to_lowercase())
        {
            Some(index) => {
                let item = items.remove(index);
                self.player.inventory.borrow_mut().push(item);
                return Ok(format!("Picked up {}", item_name));
            }
            None => Err(format!("No item of type {} is present", item_name)),
        }
    }

    pub fn use_item(&mut self, subject: &str, target: &str) -> Result<String, String> {
        Ok("Not Implemented".to_owned())
    }
}

/// Create a `World`
macro_rules! shaper_of_worlds {
    (
        arena = $arena:expr,
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
            // let room_arena = typed_arena::Arena::new();
            let mut room_lookup: std::collections::HashMap<String, &crate::entities::Room> = std::collections::HashMap::new();
            let mut destination_lookup = std::collections::HashMap::new();
            $(
                let room = $arena.alloc(entities::Room::new($room_name.to_lowercase().to_string(), $room_description.to_string()));
                $(
                    let item = crate::entities::Item::new($item.to_string());
                    room.add_item(item);
                )*
                $(
                    let feature = crate::entities::Feature::new($feature.to_string());
                    room.add_feature(feature);
                )*
                $(
                    destination_lookup.entry($room_name.clone()).or_insert(Vec::new()).push(($dir, $dest.to_lowercase()));
                )*
                room_lookup.insert($room_name.to_string(), room);
            )+
            for (room_name, destinations) in destination_lookup.iter() {
                let room = room_lookup.get(&room_name.to_string()).unwrap();
                for (direction, destination_name) in destinations {
                    let other_room = room_lookup.get(&destination_name.to_string()).unwrap();
                    room.add_exit(direction.to_string(), other_room);
                }
            }

            let initial_location = room_lookup.get($player_location).unwrap();

            World {
                player_location: initial_location,
                player: Player::default()
            }
        }
    };
}

#[cfg(test)]
#[path = "./world_building_tests.rs"]
mod world_building_tests;
