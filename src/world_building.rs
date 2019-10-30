use itertools::Itertools;
use log_derive::{logfn, logfn_inputs};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct World {
    pub locations: HashMap<String, Room>,
    pub player_location: String,
}

impl World {
    pub fn add_location(&mut self, location: Room) {
        self.locations.insert(location.id.clone(), location);
    }

    #[logfn(Info)]
    #[logfn_inputs(Info)]
    pub fn move_player(&mut self, direction: &String) -> Result<String, String> {
        // Check whether our current location has an exit that matches direction.
        // If so, set the payers location to the pointed direction.
        // returns a result with Ok(new_location), Err(No Exit)

        let current_location = self.locations.get(&self.player_location).unwrap();
        warn!("Found location: {}", current_location.id);

        match current_location.exits.get(direction) {
            Some(room_id) => {
                self.player_location = room_id.clone();
                Ok(format!("You have moved {}", direction))
            }
            None => Err(format!("{} is not a valid direction", direction)),
        }
    }

    pub fn get_player_room(&mut self) -> Option<&Room> {
        self.locations.get(&self.player_location)
    }
}

#[derive(Debug, new)]
pub struct Room {
    pub id: String,
    pub description: String,
    #[new(default)]
    exits: HashMap<String, String>,
    #[new(default)]
    items: Vec<Item>,
}

impl<'a> Room {
    pub fn add_exit(&mut self, command: String, exit_id: String) {
        self.exits.insert(command, exit_id);
    }

    pub fn get_exits(&self) -> impl Iterator<Item = &String> {
        self.exits.keys()
    }

    pub fn get_item_names(&self) -> impl Iterator<Item = String> + '_ {
        self.items.iter().map(|i| i.name.to_string())
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn has_items(&self) -> bool {
        self.items.len() > 0
    }

    pub fn get_full_description(&self) -> String {
        let mut output = String::new();
        let description = format!(
            "\n{location_description}",
            location_description = self.description
        );
        output.push_str(&description);

        let exits = format!("\nExits are {exits}", exits = self.get_exits().join(", "));
        output.push_str(&exits);

        if self.has_items() {
            let items = format!(
                "\nItems are {item_names}",
                item_names = self.get_item_names().join(", ")
            );
            output.push_str(&items);
        }

        output
    }

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
}

#[derive(Default, Debug, new)]
pub struct Player {
    pub name: String,
    #[new(default)]
    pub inventory: Vec<Item>,
}

#[derive(Debug, new)]
pub struct Item {
    pub name: String,
}

macro_rules! shaper_of_worlds {
    (
        location = $player_location:expr,
        rooms = [
            $([
                $room_name:expr,
                $room_description:expr,
                items=[$($item:expr$(,)*)*],
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
                    let item = Item::new($item.to_string());
                    room.add_item(item);
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
