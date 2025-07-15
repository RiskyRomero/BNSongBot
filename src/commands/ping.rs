use crate::{Context, Error};

/// Pings the bot
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Ping!").await?;
    Ok(())
}
