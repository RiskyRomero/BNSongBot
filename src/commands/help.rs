use crate::{Context, Error};

/// Displays help about a command
#[poise::command(
    prefix_command,
    track_edits,
    slash_command,
    check = "crate::checks::check_is_moderator"
)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            ephemeral: false,
            include_description: true,
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}
