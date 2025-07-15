use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, Color};
use rusqlite::params;

#[derive(Debug, poise::ChoiceParameter)]
pub enum Album {
    #[name = "Singles/B-Sides"]
    SinglesBSides,
    #[name = "Live Songs"]
    LiveSongs,
    #[name = "Covers"]
    Covers,
}

/// Adds a new song to the list
#[poise::command(slash_command, prefix_command)]
pub async fn new(
    ctx: Context<'_>,
    #[description = "Title of the song"] title: String,
    #[description = "Album of the song"] album: Album,
) -> Result<(), Error> {
    let album_str = match album {
        Album::SinglesBSides => "Singles/B-Sides",
        Album::LiveSongs => "Live Songs",
        Album::Covers => "Covers",
    };

    let db_lock = ctx.data().db.lock().await;

    // Check if the song already exists
    let exists: i32 = db_lock
        .query_row(
            "SELECT COUNT(*) FROM songs WHERE UPPER(title) = UPPER(?1)",
            params![title],
            |row| row.get(0),
        )
        .unwrap_or(0); // Default to 0 if the query fails

    if exists > 0 {
        let fail_embed = serenity::CreateEmbed::default()
            .title("Error!")
            .color(Color::RED)
            .description(format!(
                "The song '{}' already exists in the album '{}'.",
                title, album_str
            ));

        ctx.send(poise::CreateReply::default().embed(fail_embed))
            .await?;
        return Ok(());
    }

    // If the song does not exist, insert it
    db_lock.execute(
        "INSERT INTO songs (title, album) VALUES (?1, ?2)",
        params![title, album_str],
    )?;

    let success_embed = serenity::CreateEmbed::default()
        .title("Success!")
        .color(Color::LIGHT_GREY)
        .description(format!(
            "Inserted song: '{}' with ID: {} in album '{}'.",
            title,
            db_lock.last_insert_rowid(),
            album_str
        ));

    ctx.send(poise::CreateReply::default().embed(success_embed))
        .await?;

    Ok(())
}
