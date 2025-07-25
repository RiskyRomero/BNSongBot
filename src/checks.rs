use poise::serenity_prelude as serenity;

use crate::Context;

#[must_use]
pub fn is_moderator(ctx: Context<'_>) -> bool {
    let mod_role_id: serenity::RoleId = ctx.data().mod_role_id;

    match ctx {
        Context::Application(app_context) => {
            let Some(member) = &app_context.interaction.member else {
                // Invoked outside guild
                return false;
            };

            member.roles.contains(&mod_role_id)
        }
        Context::Prefix(msg_context) => {
            let Some(member) = &msg_context.msg.member else {
                // Command triggered outside MessageCreateEvent?
                return false;
            };

            member.roles.contains(&mod_role_id)
        }
    }
}

pub async fn check_is_moderator(
    ctx: Context<'_>,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let user_has_moderator_role = is_moderator(ctx);

    if !user_has_moderator_role {
        ctx.send(
            poise::CreateReply::default()
                .content("This command is only available to moderators.")
                .ephemeral(true),
        )
        .await
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync>)?; // Convert to Box<dyn Error>
    }

    Ok(user_has_moderator_role)
}
