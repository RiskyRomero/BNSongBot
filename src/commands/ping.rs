use crate::{Context, Error};

/// Pings the bot
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let latency = &ctx.framework().shard_manager.runners.lock().await
        [&ctx.serenity_context().shard_id]
        .latency;
    let content: String;

    if let Some(ping) = latency {
        content = format!("Pong! 🏓 My latency is `{:.2}ms`", ping.as_millis());
    } else {
        content = String::from("Pong! 🏓 Latency is unknown");
    }

    ctx.say(content).await?;
    Ok(())
}
