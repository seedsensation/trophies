pub use poise::serenity_prelude as serenity;
use dotenv::dotenv;

mod modules;

use crate::modules::types::Player;
use crate::modules::file_management;

struct Data{} // user data, stored and accessible everywhere

// define error and context
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







fn player_from_id(id: u64) -> Option<Player> {
    file_management::load().iter().find(|x| x.user_id == id).cloned()
}

async fn user_from_id(id: u64, ctx: Context<'_>) -> Option<serenity::User> {
    Player::new(id).user_data(ctx).await
}

#[poise::command(slash_command, prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    // register_application_commands_buttons returns Result<(), Error> -
    // the ? means that it will only await if the result is NOT error, and otherwise
    // it will return the function, passing the error to the Result of the function
    poise::builtins::register_application_commands_buttons(ctx).await?;
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

    let mut players = file_management::load();
    let mut id_vector = players.iter().map(|x| x.user_id).collect::<Vec<_>>();

    let current_id = u.id.get();


    if xp < 0 && !std::ptr::eq(u, author) {
        ctx.send(poise::CreateReply::default()
        .content("You cannot remove points from somebody else...")
        .ephemeral(true)).await?;
        return Ok(())
    } else {
        if !id_vector.contains(&current_id) {
            players.push(Player::new(current_id));
            id_vector.push(current_id)
        }
    }


    {
        let p: &mut Player = players.iter_mut().find(|x| x.user_id == current_id).expect("Just added user to vector - where did it go?");
        p.xp += xp;

        println!("{} (Lv. {}) just got {} XP!", u.display_name(), p.lvl, xp);


        let trophy = if xp <= 0 { "💩" } else if xp < 25 { "🥉" } else if xp < 50 { "🥈" } else { "🥇" };

        let output = format!("{trophy} | {} {} _(Lv. {}, {} / {})_ just scored **{xp} XP** for: `{title}`!", p.title(), u.display_name(), p.lvl, p.xp - p.lvl/100, p.lvl * 100);
        ctx.say(output).await?;
    }

    file_management::save(&players).expect("Failed to save");

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
