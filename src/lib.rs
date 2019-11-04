#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate log;

#[macro_use]
pub mod world_building;

use std::io::{self};
use world_building::*;

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
    let mut world = shaper_of_worlds!(
        location = "A",
        rooms = [
            [
                "A",
                "The dungeon entrance",
                items = ["Stick", "Stone"],
                exits = ["north" => "B" "west" => "c"]
            ]
            [
                "B",
                "This is B",
                items = [],
                exits = ["south" => "A"]
            ]
            [
                "C",
                "This is C",
                items = [],
                exits = ["east" => "A"]
            ]
        ]        
    );

    let player = Player::new("Bob".to_string());
    world.player = player;
    return world;
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
fn perform_action(world: &mut World, user_intput: &str) -> Result<String, String> {
    let cased = user_intput.to_lowercase().trim().to_string();

    match cased.as_ref() {
        "exit" => Err("Exiting".to_string()),
        "save" => world.save_state(),
        "load" => world.load_state(),
        _ => match world.move_player(&cased) {
            Ok(message) => Ok(message),
            Err(message) => Ok(message),
        },
    }
}