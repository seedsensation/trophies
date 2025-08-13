use crate::types;


/// Complete an Achievement, and gain XP.
#[poise::command(slash_command, prefix_command)]
pub async fn achievement(
    ctx: Context<'_>,
    #[description = "Title of your achievement"] title: String,
    #[description = "XP Achieved"] xp: i64,
    #[description = "Recipient of Achievement"] recipient: Option<serenity::User>,
) -> Result<(),Error> {

    let u = recipient.as_ref().unwrap_or_else(|| ctx.author());
    let author = ctx.author();

    types::Player::verify_player(ctx, Some(u.id.get())).expect("Failed to verify player");

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
