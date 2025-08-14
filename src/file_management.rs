use crate::player_data::Player;
use std::fs;
use std::hash::Hash;
use std::collections::HashSet;
use crate::json_data::FileFormat;

/// Saves a vector of Player to file.
///
/// Panics if the vector it has to save has duplicate objects.
/// This should never happen and would break everything if it did.
pub fn save(data: &FileFormat) {
    let j = serde_json::to_string(data).expect("Failed to convert to JSON");
    fs::write("players.json",j).expect("Failed to save file");

}

pub fn save_players(players: &Vec<Player>) {

    // crash if there are any duplicate IDs
    assert!(no_unique_elements(&players.iter().map(|x| x.user_id).collect::<Vec<_>>()));

    // load existing data with which to overwrite players
    let mut existing_data = load();
    existing_data.player_list = players.clone();
    save(&existing_data)
}

/// Loads a vector of Player from the file.
///
/// If the file doesn't exist, just return an empty vector.
pub fn load() -> FileFormat {
    let data = fs::read_to_string("players.json");
    if data.is_err() {
        return FileFormat::new()
    }

    serde_json::from_str(data.unwrap().as_str()).expect("Failed to parse JSON - invalid format")
}

pub fn load_players() -> Vec<Player> {
    load().player_list
}




/// Checks to see if an iterator has any duplicate elements
///
/// Creates a hashset out of elements from the iterator,
/// and if they all successfully insert into the hashmap
/// (i.e. no duplicates) then it returns `true`.
pub fn no_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}
