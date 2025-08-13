use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use std::cmp;
use serde::{Serialize, Deserialize};

mod commands;
mod types;
mod file_management;

use types::player_data;

struct Data{} // user data, stored and accessible everywhere

// define error and context
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;




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
                commands::register(),
                commands::achievement(),
                commands::level(),
                commands::prestige(),
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
