use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use serde::{Deserialize, Serialize};
use std::cmp;

mod big_int;
mod commands;
mod file_management;
mod modules;
mod slash_commands;
mod macros;

use modules::functions;
use modules::json_data;
use modules::player_data;

struct Data {} // user data, stored and accessible everywhere

// define error and context
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Reregister application commands with Discord.
#[poise::command(slash_command, prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    // register_application_commands_buttons returns Result<(), Error> -
    // the ? means that it will only await if the result is NOT error, and otherwise
    // it will return the function, passing the error to the Result of the function
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/// Edit your existing titles
#[poise::command(slash_command, prefix_command)]
async fn update_title(ctx: Context<'_>) -> Result<(), Error> {
    let players = file_management::load_players();
    let p = players
        .iter()
        .find(|x| x.user_id == ctx.author().id.get())
        .expect("User not present in Players despite verification")
        .clone();

    if p.title_segments.len() == 0 {
        ctx.send(
            poise::CreateReply::default()
                .content("You do not have a title to edit.")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }
    Ok(())
}


#[tokio::main]
async fn main() {
    // converts each argumnt into a BigInt, and outputs it
    test_bigint!(123, 1234, 92835729865723987238572398572398575937,292874, 1, i128::MAX);

    let (x,y,z) = into_types!(big_int::BigInt, for 150, 19999999999, 128);

    println!("{x}, {y}, {z}");

    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                slash_commands::register(),
                slash_commands::achievement(),
                slash_commands::level(),
                slash_commands::prestige(),
                slash_commands::leaderboard()
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
