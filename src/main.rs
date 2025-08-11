use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use tokio::time::{sleep, Duration};
use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::json;



struct Data{} // user data, stored and accessible everywhere

// i don't really know what these mean but it's in the docs so i'm putting it in here
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Displays an account's creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected User"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Player {
    user_id: u64,
    xp: i32,
    lvl: i32,
    prestige: i32,
    prestige_achievements: Vec<String>,
    title_segments: Vec<String>,
}

impl Player {
    fn title(&self) -> String {
        return "hello".to_string()
    }

    async fn user_data(&self, ctx: Context<'_>) -> serenity::User {
        serenity::UserId::new(self.user_id).to_user(ctx.http()).await.expect(&String::from("Failed to get user from ID {self.user_id}"))
    }
}


fn new_player(id: u64) -> Player {
    Player {
        user_id: id,
        xp: 0,
        lvl: 1,
        prestige: 0,
        prestige_achievements: vec![],
        title_segments: vec![],
    }

}

fn save(players: &Vec<Player>) -> Result<(), Error> {
    let j = serde_json::to_string(players).expect("Failed to convert to JSON");
    return Ok(fs::write("players.json",j)?)

}

fn load() -> Vec<Player> {
    let data = fs::read_to_string("players.json");
    if data.is_err() {
        return vec![]
    }

    serde_json::from_str(data.unwrap().as_str()).expect("Failed to parse JSON")
}

#[poise::command(slash_command, prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    // register_application_commands_buttons returns Result<(), Error> -
    // the ? means that it will only await if the result is NOT error, and otherwise
    // it will return the function, passing the error to the Result of the function
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

// this command was purely written to test out how to get defer working
#[poise::command(slash_command, prefix_command)]
async fn wait_five_seconds(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    sleep(Duration::from_millis(5000)).await;
    ctx.say("Five seconds have passed!").await?;
    Ok(())
}

// achievement score
#[poise::command(slash_command, prefix_command)]
async fn achievement(
    ctx: Context<'_>,
    #[description = "Title of your achievement"] title: String,
    #[description = "XP Achieved"] xp: i32,
    #[description = "Recipient of Achievement"] recipient: Option<serenity::User>,
) -> Result<(),Error> {

    let u = recipient.as_ref().unwrap_or_else(|| ctx.author());
    let author = ctx.author();

    let mut players = load();
    let mut id_vector = players.iter().map(|x| x.user_id).collect::<Vec<_>>();

    let current_id = u.id.get();


    if xp < 0 && !std::ptr::eq(u, author) {
        ctx.send(poise::CreateReply::default()
        .content("You cannot remove points from somebody else...")
        .ephemeral(true)).await?;
        return Ok(())
    } else {
        if !id_vector.contains(&current_id) {
            players.push(new_player(current_id));
            id_vector.push(current_id)
        }
    }


    {
        let p: &mut Player = players.iter_mut().find(|x| x.user_id == current_id).expect("Just added user to vector - where did it go?");
        p.xp += xp;

        println!("{} (Lv. {}) just got {} XP!", u.display_name(), p.lvl, xp);


        let trophy = if xp <= 0 { "💩" } else if xp < 25 { "🥉" } else if xp < 50 { "🥈" } else { "🥇" };

        let output = format!("{trophy} | {} _(Lv. {}, {} / {})_ just scored **{xp} XP** for: `{title}`!",u.display_name(), p.lvl, p.xp - p.lvl/100, p.lvl * 100);
        ctx.say(output).await?;
    }

    save(&players).expect("Failed to save");

    Ok(())
}


#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                age(),
                register(),
                wait_five_seconds(),
                achievement()
            ],
            ..Default::default()

        })
        .setup(|_ctx, _ready, _framework| {
            Box::pin(async move {
                // poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();

}
