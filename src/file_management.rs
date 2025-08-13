use crate::player_data::Player;
use std::fs;

pub fn save(players: &Vec<Player>) {
    let j = serde_json::to_string(players).expect("Failed to convert to JSON");
    fs::write("players.json",j).expect("Failed to save file");

}

pub fn load() -> Vec<Player> {
    let data = fs::read_to_string("players.json");
    if data.is_err() {
        return vec![]
    }

    serde_json::from_str(data.unwrap().as_str()).expect("Failed to parse JSON")
}
