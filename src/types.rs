pub mod player_data {
    use crate::{
        Serialize,
        Deserialize,
        Context,
        serenity,
        file_management,
        cmp,
    };

    pub const PRESTIGE_THRESHOLD: f64 = 0.5;
    pub const XP_EXPONENT: f64 = 1.05;
    pub const PRESTIGE_MINIMUM: f64 = 10.0;
    pub const PRESTIGE_MULTIPLIER: f64 = 0.2;

    #[derive(Serialize,Deserialize,Clone)]
    #[non_exhaustive]
    pub struct Player {
        pub user_id: u64,
        pub xp: i64,
        pub lvl: i64,
        pub prestige: f64,
        pub title_segments: Vec<String>,
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
                prestige: 1.0,
                title_segments: vec![],
            }
        }

        pub fn xp_change(&self, xp: i64) -> i64 {
            (xp as f64 * (1.0 + (self.prestige * PRESTIGE_MULTIPLIER))) as i64
        }

        pub fn add_xp(&mut self, xp: i64) {
            self.xp += self.xp_change(xp);
        }

        pub fn xp_threshold_level(&self, _level: Option<i64>) -> i64 {
            // println!("Debug: Threshold for level {}: {}",level.unwrap_or(self.lvl),2^level.unwrap_or(self.lvl - 1));
            // (50.0 * ((XP_EXPONENT).powf(level.unwrap_or(self.lvl - 1) as f64))) as i64
            return (100.0 * (self.prestige / 2.0).max(1.0)) as i64;
        }
        pub fn xp_threshold(&self) -> i64 {
            self.xp_threshold_level(None)
        }

        /// Checks whether a Player has enough XP to level up.
        ///
        /// First, checks to see if they have negative XP.
        /// If the XP is below 0, then it
        pub async fn lvl_check(&mut self, ctx: Option<Context<'_>>) -> Vec<String> {
            let mut output = vec![];
            let old_lvl = self.lvl;



            let username: String =
                if ctx.is_some() {
                    self.user_data(ctx.unwrap()).await.expect("Failed to find user data").display_name().to_owned()
                } else {
                    "[Unknown Username]".to_owned()
                };


            while self.xp < 0 && self.lvl > 1 {
                self.lvl -= 1;
                self.xp += self.xp_threshold();
                output.push(format!("{username} lost a level! They are now at Lv. {}!", self.lvl));
            }

            while self.xp >= self.xp_threshold() {
                self.xp -= self.xp_threshold();
                self.lvl += 1;
                output.push(format!("{username} gained a level! They are now at Lv. {}!", self.lvl));
            }

            if self.lvl >= cmp::max((self.prestige * PRESTIGE_THRESHOLD) as i64, PRESTIGE_MINIMUM as i64) as i64 && old_lvl < (self.prestige * PRESTIGE_THRESHOLD) as i64 {
                output.push("You are now eligible to Prestige! Use `/prestige` to find out more.".to_string())
            }

            if output.len() > 10 {
                output[1] = "...".to_string();
            }
            while output.len() > 10 {
                output.remove(2);
            }
            return output;
        }

        /// Return an XP bar, as a string.
        ///
        /// **Example**
        /// ```
        /// Player.xp = 43;
        /// assert_eq!(Player.xp_threshold(), 50)
        /// ```
        /// **Output**
        /// ```text
        /// ████████░░
        /// ```
        ///
        pub fn xp_bar(&self) -> String {
            let progress = ((self.xp as f64 / self.xp_threshold() as f64) * 10.0) as usize;

            let xp_gotten = "█".repeat(progress);
            let xp_left = "░".repeat(10-progress);

            format!("{xp_gotten}{xp_left}")

        }
    }

    /// Find a Player object from their ID.
    ///
    /// Currently unused, as it does not verify the player's presence beforehand,
    /// making it less safe than just running it manually.
    pub fn find_player_by_id(id: u64) -> Player {
        let players = file_management::load();
        players.iter().find(|x| x.user_id == id).expect("User not present in players.").clone()
    }

    /// Verify whether a player is present inside `players.json`.
    ///
    /// Check through the saved file, to see if the given ID is present.
    /// If it isn't, save it back to the file, and run the check again.
    /// **Panics if it is not present after the second check.**
    ///
    /// It only saves the file and runs the second check if the first check fails.
    ///
    pub fn verify_player(ctx: Context<'_>, id: Option<u64>) {
        let u_id = id.unwrap_or_else(|| ctx.author().id.get());
        let mut players = file_management::load();
        let id_vector = players.iter().map(|x| x.user_id).collect::<Vec<_>>();

        if !id_vector.contains(&u_id) {
            players.push(Player::new(u_id));

            // only needs to save if a change needs to be made
            file_management::save(&players);

            // assert that the loaded file, mapped for ids, contains the id that we're looking for
            assert!(file_management::load().iter().map(|x| x.user_id).collect::<Vec<_>>().contains(&u_id));
            // if it doesn't, then all hope is lost
        }



    }

}
