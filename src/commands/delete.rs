use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, Color};
use rusqlite::params;

/// Deletes a song from the list by its ID
#[poise::command(
    slash_command,
    prefix_command,
    check = "crate::checks::check_is_moderator"
)]
pub async fn delete(
    ctx: Context<'_>,
    #[description = "The ID of the song to delete"] song_id: i32,
) -> Result<(), Error> {
    let db = ctx.data().db.clone();

    let result = tokio::task::spawn_blocking(move || -> Result<usize, rusqlite::Error> {
        let db_lock = db.blocking_lock();

        let mut stmt = db_lock.prepare("DELETE FROM songs WHERE id = ?1")?;

        let affected_rows = stmt.execute(params![song_id])?;

        Ok(affected_rows)
    })
    .await??;

    // Check if a song was actually deleted
    if result == 0 {
        let embed = serenity::CreateEmbed::new()
            .title("Error")
            .color(Color::RED)
            .description(format!("No song found with ID: {}", song_id));

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    } else {
        let embed = serenity::CreateEmbed::new()
            .title("Success")
            .color(Color::DARK_GREEN)
            .description(format!(
                "Song with ID: {} has been successfully deleted.",
                song_id
            ));

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    }

    Ok(())
}
