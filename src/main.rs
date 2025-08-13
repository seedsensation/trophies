pub use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use std::cmp;

mod modules;

use crate::modules::player_data::Player;
use crate::modules::file_management;

struct Data{} // user data, stored and accessible everywhere

// define error and context
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

static PRESTIGE_THRESHOLD: f64 = 0.5;
static XP_EXPONENT: f64 = 1.05;
static PRESTIGE_MINIMUM: f64 = 10.0;
static PRESTIGE_MULTIPLIER: f64 = 0.2;

/// Displays an account's creation date
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
    Player::new(id).user_data(ctx).await
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

/// Reset your progress, for an upgraded Title and faster XP gain in future.
#[poise::command(slash_command, prefix_command)]
async fn prestige(
    ctx: Context<'_>,
    #[description="A new word to add to your Title."] title: String,
) -> Result<(),Error> {

    Player::verify_player(ctx, Some(ctx.author().id.get())).expect("Failed to verify player");

    let mut players = file_management::load();
    // Additional scope so that players can be edited. It will be saved upon the closing of the scope
    {
        let p: &mut Player = players.iter_mut().find(|x| x.user_id == ctx.author().id.get()).expect("User not present in Players despite verification");

        if title.len() > 10 {
            ctx.send(poise::CreateReply::default()
                    .content("Your new title cannot be more than 10 characters long.")
                    .ephemeral(true)
            ).await?;
            return Ok(())
        } else if title.split(" ").collect::<Vec<_>>().len() > 1 {
            ctx.send(poise::CreateReply::default()
                    .content("Your new title can only be one word long.")
                    .ephemeral(true)
            ).await?;
            return Ok(())

        }
        //ctx.say(format!("{:.1}",(p.lvl as f64 / p.prestige_threshold() as f64))).await.expect("Unknown error");

        let prestige_points = p.lvl as f64 / 10 as f64;
        let prestige_value: i64 = cmp::max((p.prestige * PRESTIGE_THRESHOLD) as i64, PRESTIGE_MINIMUM as i64);
        if p.lvl < prestige_value  as i64 {
            ctx.send(poise::CreateReply::default()
                    .content(format!("You need to be at least level {} to Prestige{}.",
                                    prestige_value,
                                    if p.prestige == 1.0 {
                                        " for the first time"
                                    } else { "" }
                    ))
                .ephemeral(true)).await?;
            return Ok(())
        }

        let components = serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("prestige.accept")
                .label("Prestige")
                .style(serenity::ButtonStyle::Danger),
            serenity::CreateButton::new("prestige.decline")
                .label("Cancel Prestige")
                .style(serenity::ButtonStyle::Primary),
        ]);

        let builder = poise::CreateReply::default()
            .content("test")
            .components(vec![components]);

        let reply = ctx.send(builder).await?;

        let interaction = reply
            .message()
            .await?
            .await_component_interaction(ctx)
            .author_id(ctx.author().id)
            .await;

        reply
            .edit(
                ctx,
                poise::CreateReply::default()
                    .components(vec![])
                    .content("Processing..."),
                ).await?;

        let pressed_button_id = match &interaction {
            Some(m) => &m.data.custom_id,
            None => {
                ctx.say(":warning: You didn't react in time, sorry!").await?;
                return Ok(())
            }
        };

        let acceptance = match &**pressed_button_id {
            "prestige.accept" => true,
            "prestige.decline" => false,
            other => {
                panic!("Unknown register button ID: {:?}",other);
            }
        };

        if acceptance {
            reply.edit(ctx, poise::CreateReply::default()
                    .content(format!("{} has Prestiged{}, and earned {:.1} Prestige Points!",
                                        ctx.author().display_name(),
                                        if p.prestige == 1.0 { " for the first time" } else { "" },
                                        prestige_points * p.prestige
            ))).await?;
            p.prestige += p.prestige * prestige_points;
            p.lvl = 1;
            p.xp = 0;
            p.title_segments.push(title);


        } else {
            reply.delete(ctx).await?;
            ctx.send(poise::CreateReply::default()
                    .content("Cancelled :)")
                    .ephemeral(true)).await?;

        }
    }

    file_management::save(&players)
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
                  



/// Complete an Achievement, and gain XP.
#[poise::command(slash_command, prefix_command)]
async fn achievement(
    ctx: Context<'_>,
    #[description = "Title of your achievement"] title: String,
    #[description = "XP Achieved"] xp: i64,
    #[description = "Recipient of Achievement"] recipient: Option<serenity::User>,
) -> Result<(),Error> {

    let u = recipient.as_ref().unwrap_or_else(|| ctx.author());
    let author = ctx.author();

    Player::verify_player(ctx, Some(u.id.get())).expect("Failed to verify player");

    let mut players = file_management::load();

    let current_id = u.id.get();


    if xp < 0 && !std::ptr::eq(u, author) {
        ctx.send(poise::CreateReply::default()
        .content("You cannot remove points from somebody else...")
        .ephemeral(true)).await?;
        return Ok(())
    }

    // new scope with which to access a player from `players`.
    {
        let p: &mut Player = players.iter_mut().find(|x| x.user_id == current_id).expect("User not present in Players despite verification");
        p.add_xp(xp);

        let lvl_output = p.lvl_check(Some(ctx)).await;

        ctx.send(poise::CreateReply::default()
                 .embed(serenity::CreateEmbed::new()
                 .title(format!("{} | Achievement Unlocked!",
                                if xp <= 0 { "💩" }
                                else if xp < 25 { "🥉" }
                                else if xp < 50 { "🥈" }
                                else { "🥇" }
                 ))
                 .author(
                    serenity::CreateEmbedAuthor::new(format!("Lv. {} {} {}", p.lvl, p.title(), u.display_name()))
                        .icon_url(u.static_avatar_url().expect("No avatar image?")))
                .fields([
                    ("Achievement",title,false),
                    ("XP Gained", p.xp_change(xp).to_string(), false),
                    ("XP Total", format!("{} _({} / {})_",p.xp_bar(), p.xp, p.xp_threshold()), false)
                ])
                .description(lvl_output.join("\n\n")))
        ).await?;
    }
    // scope exited. `players` can now be saved to file.

    file_management::save(&players)
}

/// Check your current XP, Level and Prestige.
#[poise::command(slash_command, prefix_command)]
async fn level(
    ctx: Context<'_>,
    #[description = "Selected User"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let current_id = u.id.get();

    Player::verify_player(ctx, Some(current_id)).expect("Error verifying player");

    let mut players = file_management::load();

    let p: &mut Player = players.iter_mut().find(|x| x.user_id == current_id).expect("User not present in Players despite verification");

    ctx.send(poise::CreateReply::default()
        .embed(serenity::CreateEmbed::new()
               .title(format!("User Data"))
               .author(
                    serenity::CreateEmbedAuthor::new(format!("Lv. {} {} {}", p.lvl, p.title(), u.display_name()))
                        .icon_url(u.static_avatar_url().expect("No avatar image?")))
               .fields([
                   ("Level", p.lvl.to_string(), true),
                   if p.prestige > 1.0 {
                       ("Prestige", format!("{:.1}",p.prestige), true)
                   } else {
                       ("","".to_string(),true)
                   },
                   ("XP", format!("{} _({} / {})_",p.xp_bar(), p.xp, p.xp_threshold()), false)
               ])
            )).await?;



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
                achievement(),
                level(),
                prestige(),
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
