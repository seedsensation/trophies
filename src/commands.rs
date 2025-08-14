//! When adding a slash command, add the function itself here,
//! then add a second function to [slash_commands], that points
//! back here. That'll allow you to document it separately.


use crate::{ Context, Error, player_data, cmp, file_management, serenity };

/// Reset your progress, with an advantage.
///
/// You can only prestige if you have a high enough level -
/// threshold is calculated by:
/// ```
/// cmp::max(prestige * player_data::PRESTIGE_THRESHOLD, player_data::PRESTIGE_MINIMUM)
/// ```
///
/// IF you have a high enough level, it asks for confirmation, with a
/// confirm and deny button.
/// If you deny, it deletes the message.
///
/// If you confirm, it triggers the Prestige.
///
/// It adds your chosen title to your list of titles,,
/// multiplies your current prestige by your new level of prestige,
/// and sets your level and XP back to 0``
///
/// If you have a high enough level, it asks for confirmation, with a
/// confirm and deny button.
/// If you deny, it deletes the message.
///
/// If you confirm, it triggers the Prestige.
///
/// It adds your chosen title to your list of titles,,
/// multiplies your current prestige by your new level of prestige,
/// and sets your level and XP back to 0.
///
pub async fn prestige(
    ctx: Context<'_>,
    title: String,
) -> Result<(),Error> {


    player_data::verify_player(ctx, Some(ctx.author().id.get()));

    let mut players = file_management::load_players();
    // Additional scope so that players can be edited. It will be saved upon the closing of the scope
    {
        let p: &mut player_data::Player = players.iter_mut().find(|x| x.user_id == ctx.author().id.get()).expect("User not present in Players despite verification");

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

        let prestige_points = p.prestige_points();
        if p.lvl < p.prestige_threshold {
            ctx.send(poise::CreateReply::default()
                    .content(format!("You need to be at least level {} to Prestige{}.",
                                    p.prestige_threshold,
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
                    .content(format!("{} has Prestiged{}, and now has {:.2} Prestige Points!",
                                        ctx.author().display_name(),
                                        if p.prestige == 1.0 { " for the first time" } else { "" },
                                        prestige_points * p.prestige
            ))).await?;
            p.prestige = p.prestige * prestige_points;
            p.prestige_threshold = p.lvl;
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

    file_management::save_players(&players);

    Ok(())
}


/// Check your current XP, Level and Prestige.
pub async fn level(
    ctx: Context<'_>,
    user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let current_id = u.id.get();

    player_data::verify_player(ctx, Some(current_id));

    let mut players = file_management::load_players();

    let p: &mut player_data::Player = players.iter_mut().find(|x| x.user_id == current_id).expect("User not present in Players despite verification");

    ctx.send(poise::CreateReply::default()
        .embed(serenity::CreateEmbed::new()
               .title(format!("User Data"))
               .author(
                    serenity::CreateEmbedAuthor::new(format!("Lv. {} {} {}", p.lvl, p.title(), u.display_name()))
                        .icon_url(u.static_avatar_url().unwrap_or_else(|| u.default_avatar_url())))
               .fields([
                   ("Level", p.lvl.to_string(), true),
                   if p.prestige > 1.0 {
                       ("Prestige", format!("{:.2}",p.prestige), true)
                   } else {
                       ("","".to_string(),true)
                   },
                   ("XP", format!("{} _({} / {})_",p.xp_bar(), p.xp, p.xp_threshold()), false)
               ])
            )).await?;



    Ok(())
}

/// Complete an Achievement, and gain XP.
///
/// Accepts a title, an XP number, and a recipient (optional).
///
/// Loads the information of the recipient from the JSON file,
/// edits it to update the xp, then runs
/// [`lvl_check()`](player_data::lvl_check).
pub async fn achievement(
    ctx: Context<'_>,
    title: String,
    xp: i64,
    recipient: Option<serenity::User>,
) -> Result<(),Error> {

    let u = recipient.as_ref().unwrap_or_else(|| ctx.author());
    let author = ctx.author();

    player_data::verify_player(ctx, Some(u.id.get()));

    let mut players = file_management::load_players();

    let current_id = u.id.get();


    if xp < 0 && !std::ptr::eq(u, author) {
        ctx.send(poise::CreateReply::default()
        .content("You cannot remove points from somebody else...")
        .ephemeral(true)).await?;
        return Ok(())
    }

    // new scope with which to access a player from `players`.
    {
        println!("Adding XP");
        let p: &mut player_data::Player = players.iter_mut().find(|x| x.user_id == current_id).expect("User not present in Players despite verification");
        p.add_xp(xp);

        println!("XP added");

        println!("Checking level");
        let lvl_output = p.lvl_check(Some(ctx)).await;

        println!("Sending Message");
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

    file_management::save_players(&players);

    Ok(())
}


/// Edit your existing titles
pub async fn update_title(ctx: Context<'_>) -> Result<(),Error> {

    let players = file_management::load_players();
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
