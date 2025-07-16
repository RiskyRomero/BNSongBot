use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, Color};

struct Album {
    id: i32,
    name: String,
}

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("create", "delete", "list"),
    subcommand_required,
    check = "crate::checks::check_is_moderator"
)]

/// Create, delete or list an album
pub async fn album(_: Context<'_>) -> Result<(), Error> {
    // This will never be called, because `subcommand_required` parameter is set
    Ok(())
}

/// Create an album
#[poise::command(prefix_command, slash_command)]
pub async fn create(
    ctx: Context<'_>,
    #[description = "The name of the album"] album_name: String,
) -> Result<(), Error> {
    let album_name_cloned = album_name.clone();

    let db = ctx.data().db.clone();

    let _result = tokio::task::spawn_blocking(move || -> Result<(), rusqlite::Error> {
        let db_lock = db.blocking_lock();

        let mut stmt = db_lock.prepare("INSERT INTO albums (name) VALUES (?1)")?;

        stmt.execute([album_name_cloned])?;

        Ok(())
    })
    .await??;

    ctx.say(format!(
        "Album '{}' has been created successfully.",
        album_name
    ))
    .await?;

    Ok(())
}

/// Delete an album
#[poise::command(prefix_command, slash_command)]
pub async fn delete(
    ctx: Context<'_>,
    #[description = "The name of the album"] album_name: String,
) -> Result<(), Error> {
    let album_name_cloned = album_name.clone();

    let db = ctx.data().db.clone();

    let result = tokio::task::spawn_blocking(move || -> Result<bool, rusqlite::Error> {
        let db_lock = db.blocking_lock();

        let mut stmt = db_lock.prepare("DELETE FROM albums WHERE UPPER(name) = UPPER(?1)")?;

        let affected_rows = stmt.execute([album_name_cloned])?;

        if affected_rows == 0 {
            return Ok(false); // Indicate that no album was deleted
        }

        Ok(true) // Album was deleted successfully
    })
    .await??;

    if !result {
        let delete_fail_embed = serenity::CreateEmbed::default()
            .title("Error")
            .color(Color::RED)
            .description("Album not found.");

        ctx.send(poise::CreateReply::default().embed(delete_fail_embed))
            .await?;
    } else {
        let delete_success_embed = serenity::CreateEmbed::default()
            .title("Success")
            .description(format!(
                "Album '{}' has been deleted successfully.",
                album_name
            ));

        ctx.send(poise::CreateReply::default().embed(delete_success_embed))
            .await?;
    }

    Ok(())
}

/// List all albums
#[poise::command(prefix_command, slash_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let db = ctx.data().db.clone();

    let albums = tokio::task::spawn_blocking(move || -> Result<Vec<Album>, rusqlite::Error> {
        let db_lock = db.blocking_lock();

        let mut stmt = db_lock.prepare("SELECT * FROM albums")?;

        let album_iter = stmt.query_map([], |row| {
            Ok(Album {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let mut albums = Vec::new();
        for album_result in album_iter {
            albums.push(album_result?);
        }

        Ok(albums)
    })
    .await??; // Unwrap the results of the blocking task

    if albums.is_empty() {
        let no_albums_embed = serenity::CreateEmbed::default()
            .title("No Albums")
            .description("No albums were found in the database.");

        ctx.send(poise::CreateReply::default().embed(no_albums_embed))
            .await?;

        return Ok(());
    }

    let album_list = albums
        .iter()
        .map(|album: &Album| format!("- `{}`", album.name))
        .collect::<Vec<String>>()
        .join("\n");

    let album_list_embed = serenity::CreateEmbed::default()
        .title("Albums")
        .description(album_list);

    ctx.send(poise::CreateReply::default().embed(album_list_embed))
        .await?;

    Ok(())
}
