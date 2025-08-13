use crate::{
    Context,
    Error,
    player_data,
    file_management,
    serenity,
};

/// Check your current XP, Level and Prestige.
#[poise::command(slash_command, prefix_command)]
pub async fn level(
    ctx: Context<'_>,
    #[description = "Selected User"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let current_id = u.id.get();

    player_data::verify_player(ctx, Some(current_id)).expect("Error verifying player");

    let mut players = file_management::load();

    let p: &mut player_data::Player = players.iter_mut().find(|x| x.user_id == current_id).expect("User not present in Players despite verification");

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
