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

#[poise::command(slash_command, prefix_command)]
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
        let mut list_str = String::new();
        for song_result in song_iter {
            let song = song_result.unwrap();
            list_str += &format!("{} - {} ({})\n", song.id, song.title, song.album);
        }

        list_str
    })
    .await
    .unwrap();

    if list_str.is_empty() {
        if album == "" {
            ctx.say("No songs found.").await?;
        } else {
            ctx.say(format!("No songs found for {}.", album)).await?;
        }
    } else {
        ctx.say(list_str).await?;
    }

    Ok(())
}
