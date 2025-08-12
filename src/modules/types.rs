use poise::serenity_prelude as serenity;
use crate::Context;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub user_id: u64,
    pub xp: i32,
    pub lvl: i32,
    pub prestige: i32,
    prestige_achievements: Vec<String>,
    title_segments: Vec<String>,
}

impl Player {
    pub fn title(&self) -> String {
        let mut output: String = "".to_owned();
        for i in &self.title_segments {
            output.push_str(i);
            output.push_str(" ");
        }
        output
    }

    pub async fn user_data(&self, ctx: Context<'_>) -> Option<serenity::User> {
        serenity::UserId::new(self.user_id).to_user(ctx.http()).await.ok()
    }
    pub fn new(id: u64) -> Player {
        Player {
            user_id: id,
            xp: 0,
            lvl: 1,
            prestige: 0,
            prestige_achievements: vec![],
            title_segments: vec![],
        }

    }
}

