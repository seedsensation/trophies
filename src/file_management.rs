use crate::modules::player_data::Player;
use crate::Error;
use std::fs;

pub fn save(players: &Vec<Player>) -> Result<(), Error> {
    let j = serde_json::to_string(players).expect("Failed to convert to JSON");
    return Ok(fs::write("players.json",j)?)

}

pub fn load() -> Vec<Player> {
    let data = fs::read_to_string("players.json");
    if data.is_err() {
        return vec![]
    }

    serde_json::from_str(data.unwrap().as_str()).expect("Failed to parse JSON")
}
