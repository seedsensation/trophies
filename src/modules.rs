pub mod player_data {
    use crate::{
        Serialize,
        Deserialize,
        Context,
        serenity,
        file_management,
        functions,
    };
    use functions::Overflows;


    /// The amount by which the XP threshold is multiplied by prestige
    pub const XP_THRESHOLD_MULTIPLIER: f64 = 2.0;

    /// The amount by which XP is multiplied by prestige
    pub const XP_MULTIPLIER: f64 = 0.5;


    /// Contains all required info about a given player.
    ///
    /// Can be cross-referenced with the Discord API using
    /// the [`user_id`](Self::user_id) property.
    ///
    /// Players are stored in a vector, accessible from a
    /// file by using [`file_management::load()`], and saved
    /// using [`file_management::save()`]. Whenever you access
    /// the vector created by [`load()`](file_management::load()),
    /// use [`verify_player()`](verify_player) first, to ensure that the
    /// player is present - otherwise it will panic.
    #[derive(Serialize,Deserialize,Clone)]
    #[non_exhaustive]
    pub struct Player {

        /// The ID of the user. Should be entirely unique.
        ///
        /// Uniqueness of `user_id` is checked whenever saved.
        pub user_id: u64,

        /// The user's XP. Increased by using
        /// [`/achievement`](crate::commands::achievement).
        ///
        pub xp: i64,

        /// The player's current level.
        ///
        /// Increases whenever XP passes
        /// [`xp_threshold`](Self::xp_threshold), and
        /// decreases whenever it goes below 0.
        pub lvl: i64,

        /// The player's current prestige level.
        ///
        /// Multiplies all XP gained using
        /// [`xp_change`](Self::xp_change).
        /// Can be increased by using the
        /// [`/prestige`](crate::commands::prestige) command.
        ///
        /// New prestige is multiplicative, not additive,
        /// so your Prestige grows exponentially.
        pub prestige: f64,

        /// Each word in your Title.
        ///
        /// The entire title is calculated using
        /// [`title()`](Self::title), and can be edited using
        /// the
        pub title_segments: Vec<String>,

        /// The last level at which the player prestiged.
        ///
        /// Starts at 10.0, and increases to current level whenever
        /// the player prestiges.
        pub prestige_threshold: i64,
    }

    impl Player {

        /// Calculates a title for the object, using its
        /// [`title_segments`](Self::title_segments) attribute.
        pub fn title(&self) -> String {
            let mut output: String = "".to_owned();
            for i in &self.title_segments {
                output.push_str(i);
                output.push_str(" ");
            }
            output
        }

        /// Returns Discord user from Player
        ///
        /// Requires a `ctx` object in order to access Discord's servers.
        pub async fn user_data(&self, ctx: Context<'_>) -> Option<serenity::User> {
            serenity::UserId::new(self.user_id).to_user(ctx.http()).await.ok()
        }

        /// Initialise a new [`Player`] object, from a given ID.
        pub fn new(id: u64) -> Player {
            Player {
                user_id: id,
                xp: 0,
                lvl: 1,
                prestige: 1.0,
                title_segments: vec![],
                prestige_threshold: 10,
            }
        }

        /// Calculates how much XP you should earn, from a base number.
        ///
        /// The formula for the XP is:
        /// ```
        /// xp * (1 + ((self.prestige - 1) * PRESTIGE_MULTIPLIER))
        /// ```
        ///
        pub fn xp_change(&self, xp: i64) -> i64 {
            println!("Calculating XP change");
            match functions::overflow_check::<_,i64>(|| xp * self.prestige as i64) {
                Overflows::Int => i64::MAX - self.xp,
                Overflows::Float => xp * self.prestige as i64,
                Overflows::Safe => (xp as f64 + (xp as f64 * ( self.prestige - 1.0 ) * XP_MULTIPLIER)) as i64,
            }
        }

        /// Adds XP, calculated using [`xp_change`](Self::xp_change).
        pub fn add_xp(&mut self, xp: i64) {
            println!("Adding XP internally");
            self.xp += self.xp_change(xp);
            println!("XP added");
        }

        /// Checks how much XP you need to level up.
        ///
        /// Uses this formula:
        /// ```
        /// 100.0 * (self.prestige * PRESTIGE_MULTIPLIER * 0.5).max(1.0)
        /// ```
        ///
        /// It scales with prestige ([half as fast as XP scales](Self::xp_change)),
        /// *to a minimum of 1*. This is so that the threshold is, *at minimum*, 100.
        ///
        /// Originally was going to scale exponentially, but I discovered that it
        /// would make it basically impossible to prestige after your third prestige.
        /// (In one test, it required billions of XP to reach a single level past level
        /// 60, and it required reaching level 2000 to be able to prestige 😭)

        pub fn xp_threshold(&self) -> i64 {
            // println!("Debug: Threshold for level {}: {}",level.unwrap_or(self.lvl),2^level.unwrap_or(self.lvl - 1));
            // (50.0 * ((XP_EXPONENT).powf(level.unwrap_or(self.lvl - 1) as f64))) as i64
            return 50 + (25.0 * XP_THRESHOLD_MULTIPLIER * (self.prestige - 1.0)) as i64;
        }

        // Prestige Points Section
        // So. Prestige points should be calculated as starting from the current prestige threshold.
        // e.g. prestige threshold = 10, lvl = 11 => prestige points = 1 + (1 / 20)
        // therefore, we need the formula:
        // ((level - prestige_threshold)/20) + 1
        //
        // prestige_threshold should then be related to your current prestige,
        // preferably multiplicative rather than additive.
        //
        //

        pub fn prestige_points(&self) -> f64 {
            ((self.lvl as f64 - self.prestige_threshold as f64 + 10.0) / 100.0) + 1.0
        }

        /// Checks whether a Player has enough [`XP`](Self::xp) to level up.
        ///
        /// First, checks to see if they have negative XP.
        /// If the XP is below 0, then it removes a level,
        /// adds XP back, and repeats, until the XP is in
        /// the positive again.
        ///
        /// Then, it does the same, but in reverse.
        /// If the XP is above [`xp_threshold`](Self::xp_threshold), then it
        /// removes XP, increments the level, and repeats,
        /// until the XP is below [`xp_threshold`](Self::xp_threshold) again.
        pub async fn lvl_check(&mut self, ctx: Option<Context<'_>>) -> Vec<String> {
            let mut output = vec![];
            let old_lvl = self.lvl;



            let username: String =
                if ctx.is_some() {
                    self.user_data(ctx.unwrap()).await.expect("Failed to find user data").display_name().to_owned()
                } else {
                    "[Unknown Username]".to_owned()
                };


            println!("Checking level down");
            while self.xp < 0 && self.lvl > 1 {
                self.lvl -= 1;
                self.xp += self.xp_threshold();
                output.push(format!("{username} lost a level! They are now at Lv. {}!", self.lvl));
            }

            println!("Checking level up");

            if self.xp > self.xp_threshold() {
                let level_change = self.xp / self.xp_threshold();
                if level_change > 4 {
                    output.push(format!("{username} gained a level! They are now at Lv. {}!\n...",self.lvl + 1));
                    output.push(format!("{username} gained a level! They are now at Lv. {}!",self.lvl + level_change - 1));
                    output.push(format!("{username} gained a level! They are now at Lv. {}!",self.lvl + level_change));
                } else {
                    for i in 1..level_change {
                        output.push(format!("{username} gained a level! They are now at Lv. {}!",self.lvl + i));
                    }
                }
                self.lvl += self.xp / self.xp_threshold();
                println!("Gained {} levels!",self.xp / self.xp_threshold());
                self.xp = self.xp % self.xp_threshold();
            }
            // while self.xp >= self.xp_threshold() {
            //     println!("Subtracting XP");
            //     self.xp -= self.xp_threshold();
            //     println!("Increasing level");
            //     self.lvl += 1;
            //     println!("Adding to output string");
            //     output.push(format!("{username} gained a level! They are now at Lv. {}!", self.lvl));
            // }

            println!("Checking prestige eligibility");

            if self.lvl >= self.prestige_threshold && old_lvl < self.prestige_threshold {
                output.push("You are now eligible to Prestige! Use `/prestige` to find out more.".to_string())
            }

            println!("Compressing msg");

            if output.len() > 10 {
                output[1] = "...".to_string();
            }
            while output.len() > 10 {
                println!("Removing line");
                output.remove(2);
            }

            println!("Level check complete");
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
        let players = file_management::load_players();
        players.iter().find(|x| x.user_id == id).expect("User not present in players.").clone()
    }

    /// Verify whether a player is present inside `players.json`.
    ///
    /// Check through the saved file, to see if the given ID is present.
    /// If it isn't, save it back to the file, and run the check again.
    ///
    /// It only saves the file and runs the second check if the first check fails.
    /// **Panics if the second check fails.**
    ///
    pub fn verify_player(ctx: Context<'_>, id: Option<u64>) {
        let u_id = id.unwrap_or_else(|| ctx.author().id.get());
        let mut players = file_management::load_players();
        let id_vector = players.iter().map(|x| x.user_id).collect::<Vec<_>>();

        if !id_vector.contains(&u_id) {
            players.push(Player::new(u_id));

            // only needs to save if a change needs to be made
            file_management::save_players(&players);

            // assert that the loaded file, mapped for ids, contains the id that we're looking for
            assert!(file_management::load_players().iter().map(|x| x.user_id).collect::<Vec<_>>().contains(&u_id));
            // if it doesn't, then all hope is lost
        }



    }
}

pub mod json_data {
    use crate::{Serialize, Deserialize, player_data};

    #[non_exhaustive]
    #[derive(Serialize, Deserialize)]
    pub struct FileFormat {
        pub player_list: Vec<player_data::Player>,
    }

    impl FileFormat {
        pub fn new() -> FileFormat {
            FileFormat {
                player_list: vec![],
            }
        }
    }
}

pub mod functions {

    pub enum Overflows {
        Float,
        Int,
        Safe,
    }


    #[should_panic]
    pub fn overflow_check<F, N>(f: F) -> Overflows where
        F: FnOnce() -> i64 + std::panic::UnwindSafe {
        let result = std::panic::catch_unwind(||f());
        if result.is_err() {
            println!("Caught overflow");
            return Overflows::Int;

        }
        let result_unwrapped = result.unwrap();
        if result_unwrapped >= f64::MAX as i64 { Overflows::Float } else { Overflows::Safe }
    }
}
