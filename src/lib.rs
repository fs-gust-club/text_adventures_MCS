#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate log;

#[macro_use]
pub mod world_building;
pub mod arena;
pub mod entities;
pub mod parser;

use entities::Player;
use parser::Action;
use std::io;
use world_building::World;

/// Create the world and start the main loop
pub fn start() {
    let mut world = let_there_be_light();

    println!("Welcome");

    loop {
        // Write output
        match world.get_player_room() {
            None => {
                println!("Location does not exist");
                break;
            }
            Some(room) => println!("{}", room.get_full_description()),
        }

        // Get input
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Err(error) => {
                println!("error: {}", error);
                break;
            }
            _ => {}
        }

        // Perform actions
        match perform_action(&mut world, &user_input) {
            Err(err) => {
                println!("{}", err);
                break;
            }
            Ok(output) => println!("{}", output),
        }
    }
}

/// Create the initial world state
fn let_there_be_light() -> World {
    let world = shaper_of_worlds!(
        location = "entrance",
        rooms = [
            [
                "entrance",
                "The dungeon entrance",
                items = [],
                features = [],
                exits = ["north" => "corridor"]
            ]
            [
                "corridor",
                "A long corridor",
                items = [],
                features = ["north door"],
                exits = ["south" => "entrance" "west" => "storeroom"]
            ]
            [
                "storeroom",
                "An old dusty storeroom",
                items = ["key", "tinderbox"],
                features = [],
                exits = ["east" => "corridor"]
            ]
            [
                "dark room",
                "A dimly lit room with unlit torches on the walls",
                items = [],
                features = [],
                exits = ["south" => "corridor"]
            ]
            [
                "treasure room",
                "A room full of shiney things",
                items = ["phat loot"],
                features = [],
                exits = ["south" => "dark room"]
            ]
        ]
    );

    let player = Player::new("Bob".to_string());
    world.player.replace(player);
    world
}

/// Parse the user input and perform the action if possible
///
/// # Arguments
///
/// * `world` - the current world state
/// * `user_input` - the user command
///
/// # Errors
///
/// The user command is not valid
fn perform_action(world: &World, user_input: &str) -> Result<String, String> {
    let action = parser::parse_input(user_input);
    match action {
        parser::Action::Exit => Err("Exiting".to_string()),
        Action::Save => world.save_state(),
        Action::Load => world.load_state(),
        Action::Inventory => Ok(world.get_player().list_inventory()),
        Action::Move(direction) => acceptable_error(world.move_player(&direction)),
        Action::Take(item_name) => acceptable_error(world.take_item(&*item_name)),
        Action::Use(subject, target) => world.use_item(&subject, &target),
        Action::Unknown => Ok("You cannot do that".to_string()),
        _ => Ok("You cannot do that".to_string()),
    }
}

fn acceptable_error(error: Result<String, String>) -> Result<String, String> {
    match error {
        Ok(msg) => Ok(msg),
        Err(msg) => Ok(msg),
    }
}
