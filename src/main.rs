use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use std::cmp;
use serde::{Serialize, Deserialize};

mod commands;
mod types;
mod file_management;

use types::player_data;
use commands::*;

struct Data{} // user data, stored and accessible everywhere

// define error and context
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


/// Displays an account's creation date.
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

async fn user_from_id(id: u64, ctx: Context<'_>) -> Option<serenity::User> {
    player_data::Player::new(id).user_data(ctx).await
}

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
async fn update_title(ctx: Context<'_>) -> Result<(),Error> {

    let players = file_management::load();
    let p = players.iter().find(|x| x.user_id == ctx.author().id.get()).expect("User not present in Players despite verification").clone();

    if p.title_segments.len() == 0 {
        ctx.send(poise::CreateReply::default()
                 .content("You do not have a title to edit.")
                 .ephemeral(true)).await?;
        return Ok(())
    }
    // let menu = serenity::CreateActionRow::SelectMenu(
    //     CreateSelectMenu::new("selected_title"),

    // );

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
                achievement::achievement(),
                level::level(),
                prestige::prestige(),
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
