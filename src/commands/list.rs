use poise::serenity_prelude::{self as serenity, Color};
use rusqlite::{Row, Statement};

use crate::{Context, Error};

#[derive(Debug, poise::ChoiceParameter)]
pub enum Album {
    #[name = "Singles/B-Sides"]
    SinglesBSides,
    #[name = "Live Songs"]
    LiveSongs,
    #[name = "Covers"]
    Covers,
}

struct Song {
    id: i32,
    title: String,
    album: String,
}

/// Displays a list of songs
#[poise::command(
    slash_command,
    prefix_command,
    check = "crate::checks::check_is_moderator"
)]
pub async fn list(
    ctx: Context<'_>,
    #[description = "Select an album to view songs"] album: Option<Album>,
) -> Result<(), Error> {
    let album = match album {
        Some(Album::SinglesBSides) => "Singles/B-Sides",
        Some(Album::LiveSongs) => "Live Songs",
        Some(Album::Covers) => "Covers",
        None => "",
    };

    let db = ctx.data().db.clone();

    let list_str = tokio::task::spawn_blocking(move || {
        let db_lock = db.blocking_lock();

        let mut stmt: Statement<'_>;

        // Prepare the query based on whether an album is selected or not
        if album == "" {
            stmt = db_lock.prepare("SELECT * FROM songs").unwrap();
        } else {
            stmt = db_lock
                .prepare("SELECT * FROM songs WHERE album = ?1")
                .unwrap();
        }

        let map_row = |row: &Row<'_>| {
            Ok(Song {
                id: row.get(0)?,
                title: row.get(1)?,
                album: row.get(2)?,
            })
        };

        let song_iter = if album == "" {
            stmt.query_map([], map_row).unwrap()
        } else {
            stmt.query_map([album], map_row).unwrap()
        };

        // Create a string to hold the song list output
        let mut list_str = String::from("**ID - Title (Album)**\n");
        for song_result in song_iter {
            let song = song_result.unwrap();
            list_str += &format!("{} - {} ({})\n", song.id, song.title, song.album);
        }

        list_str
    })
    .await
    .unwrap();

    if list_str.is_empty() {
        let no_songs_found_str: String;
        if album == "" {
            no_songs_found_str = "No songs found.".to_string();
        } else {
            no_songs_found_str = format!("No songs found for {}.", album);
        }
        let no_songs_embed = serenity::CreateEmbed::new()
            .title("Error")
            .color(Color::RED)
            .description(no_songs_found_str);
        ctx.send(poise::CreateReply::default().embed(no_songs_embed))
            .await?;
    } else {
        let list_embed = serenity::CreateEmbed::new()
            .title("Song List")
            .color(Color::MAGENTA)
            .description(list_str);
        ctx.send(poise::CreateReply::default().embed(list_embed))
            .await?;
    }

    Ok(())
}
