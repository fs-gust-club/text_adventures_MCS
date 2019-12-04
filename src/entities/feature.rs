use serde::{Deserialize, Serialize};
use std::fs;

/// Describes an attribute of a room, which will eventually hold behaviour
#[derive(Debug, new, Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
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
}
