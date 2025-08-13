use crate::{
    Context,
    Error,
    player_data,
    cmp,
    file_management,
    serenity,
};


/// Reset your progress, for an upgraded Title and faster XP gain in future.
#[poise::command(slash_command, prefix_command)]
pub async fn prestige(
    ctx: Context<'_>,
    #[description="A new word to add to your Title."] title: String,
) -> Result<(),Error> {

    player_data::verify_player(ctx, Some(ctx.author().id.get())).expect("Failed to verify player");

    let mut players = file_management::load();
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

        let prestige_points = p.lvl as f64 / 10 as f64;
        let prestige_value: i64 = cmp::max((p.prestige * player_data::PRESTIGE_THRESHOLD) as i64, player_data::PRESTIGE_MINIMUM as i64);
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
