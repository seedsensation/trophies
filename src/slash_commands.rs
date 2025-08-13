//! Stores all the Discord slash command objects.
//!
//! These all point to their relevant files in [commands] -
//! the main purpose of this existing is to allow me to document
//! the functions separately from the `help` strings that Poise
//! automatically uses.


use crate::{commands, Context, Error, serenity};

/// Reset your progress, with an advantage.
#[poise::command(slash_command, prefix_command)]
pub async fn prestige(
    ctx: Context<'_>,
    #[description="A new word to add to your Title."] title: String,
) -> Result<(),Error> {
    commands::prestige(ctx,title).await
}

/// Check your current XP, Level and Prestige.
#[poise::command(slash_command, prefix_command)]
pub async fn level(
    ctx: Context<'_>,
    #[description = "Selected User"] user: Option<serenity::User>,
) -> Result<(), Error> {
    commands::level(ctx, user).await
}

/// Complete an Achievement, and gain XP.
#[poise::command(slash_command, prefix_command)]
pub async fn achievement(
    ctx: Context<'_>,
    #[description = "Title of your achievement"] title: String,
    #[description = "XP Achieved"] xp: i64,
    #[description = "Recipient of Achievement"] recipient: Option<serenity::User>,
) -> Result<(),Error> {
    commands::achievement(ctx, title, xp, recipient).await
}

/// Reregister application commands with Discord.
#[poise::command(slash_command, prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn update_title(ctx: Context<'_>) -> Result<(),Error> {
    commands::update_title(ctx).await
}
