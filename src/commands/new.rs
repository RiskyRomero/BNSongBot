use crate::{Context, Error};
use futures::{Stream, StreamExt};
use poise::serenity_prelude::{self as serenity, Color};
use rusqlite::{Row, params};

#[derive(Debug, poise::ChoiceParameter)]
pub enum Album {
    #[name = "Singles/B-Sides"]
    SinglesBSides,
    #[name = "Live Songs"]
    LiveSongs,
    #[name = "Covers"]
    Covers,
}

async fn autocomplete_album<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let db = ctx.data().db.clone(); // Access the database connection

    // Clone `partial` to move it into the blocking task closure
    let partial_cloned = partial.to_string(); // Clone `partial` as a `String`

    // Perform the database query in a blocking task
    let stream = tokio::task::spawn_blocking(move || -> Result<Vec<String>, rusqlite::Error> {
        let db_lock = db.blocking_lock(); // Lock the database for thread-safe access

        let mut stmt = db_lock.prepare("SELECT name FROM albums WHERE name LIKE ?1")?;
        let album_iter = stmt.query_map([format!("{}%", partial_cloned)], |row: &Row| {
            row.get::<_, String>(0)
        })?;

        // Collect the album names into a Vec
        let mut album_names = Vec::new();
        for album_result in album_iter {
            match album_result {
                Ok(album_name) => {
                    album_names.push(album_name.clone());
                }
                Err(err) => {
                    // Log the error but continue collecting other names
                    eprintln!("Error while fetching album: {}", err);
                }
            }
        }

        Ok(album_names)
    })
    .await;

    let album_names = stream
        .expect("Failed to fetch album names")
        .expect("Error fetching album names");

    if album_names.is_empty() {
        eprintln!("No matching albums found.");
    }

    futures::stream::iter(album_names)
}

/// Adds a new song to the list
#[poise::command(
    slash_command,
    prefix_command,
    check = "crate::checks::check_is_moderator"
)]
pub async fn new(
    ctx: Context<'_>,
    #[description = "Title of the song"] title: String,
    #[autocomplete = "autocomplete_album"]
    #[description = "Album of the song"]
    album: String,
) -> Result<(), Error> {
    // let album_str = match album {
    //     Album::SinglesBSides => "Singles/B-Sides",
    //     Album::LiveSongs => "Live Songs",
    //     Album::Covers => "Covers",
    // };

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
                title, album
            ));

        ctx.send(poise::CreateReply::default().embed(fail_embed))
            .await?;
        return Ok(());
    }

    // If the song does not exist, insert it
    db_lock.execute(
        "INSERT INTO songs (title, album) VALUES (?1, ?2)",
        params![title, album],
    )?;

    let success_embed = serenity::CreateEmbed::default()
        .title("Success!")
        .color(Color::LIGHT_GREY)
        .description(format!(
            "Inserted song: '{}' with ID: {} in album '{}'.",
            title,
            db_lock.last_insert_rowid(),
            album
        ));

    ctx.send(poise::CreateReply::default().embed(success_embed))
        .await?;

    Ok(())
}
