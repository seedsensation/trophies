use crate::player_data::Player;
use std::fs;
use std::hash::Hash;
use std::collections::HashSet;

/// Saves a vector of Player to file.
///
/// Panics if the vector it has to save has duplicate objects.
/// This should never happen and would break everything if it did.
pub fn save(players: &Vec<Player>) {
    // panic if duplicate IDs are present within `players`.
    assert!(!has_unique_elements(&players.iter().map(|x| x.user_id).collect::<Vec<_>>()));
    let j = serde_json::to_string(players).expect("Failed to convert to JSON");
    fs::write("players.json",j).expect("Failed to save file");

}

/// Loads a vector of Player from the file.
///
/// If the file doesn't exist, just return an empty vector.
pub fn load() -> Vec<Player> {
    let data = fs::read_to_string("players.json");
    if data.is_err() {
        return vec![]
    }

    serde_json::from_str(data.unwrap().as_str()).expect("Failed to parse JSON")
}

/// Checks to see if an iterator has any duplicate elements
///
/// Creates a hashset out of elements from the iterator,
/// and if any of them fail to insert into the hashset
/// (i.e. duplicates) then it returns `true`.
pub fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}
