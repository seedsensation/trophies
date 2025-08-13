use poise::serenity_prelude as serenity;
use crate::{Context, Error, PRESTIGE_THRESHOLD, XP_EXPONENT, PRESTIGE_MULTIPLIER, PRESTIGE_MINIMUM};
use serde::{Deserialize, Serialize};
use crate::lib::*;
use crate::cmp;


#[derive(Serialize, Deserialize, Clone)]
#[non_exhaustive]
pub struct Player {
    pub user_id: u64,
    pub xp: i64,
    pub lvl: i64,
    pub prestige: f64,
    prestige_achievements: Vec<String>,
    pub title_segments: Vec<String>,
}

impl Player {




    pub fn find_player_by_id(id: u64) -> Player {
        let players = file_management::load();
        players.iter().find(|x| x.user_id == id).expect("User not present in Players despite verification").clone()
    }





}
