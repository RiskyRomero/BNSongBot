use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, Color, colours};
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

#[derive(Debug)]
struct Song {
    id: i32,
    title: String,
    album: String,
}

/// Gets a single or multiple random songs
#[poise::command(slash_command, prefix_command)]
pub async fn random(
    ctx: Context<'_>,
    #[description = "Select an album to get a random song from"] album: Option<Album>,
    #[description = "Amount of songs to get"] amount: Option<u32>,
) -> Result<(), Error> {
    let album_str = match album {
        Some(Album::SinglesBSides) => "Singles/B-Sides",
        Some(Album::LiveSongs) => "Live Songs",
        Some(Album::Covers) => "Covers",
        None => "",
    }
    .to_string();

    let amount_to_query = amount.unwrap_or(1).min(25) as i32; // prevent overly large queries
    let db = ctx.data().db.clone();

    let songs = tokio::task::spawn_blocking(move || {
        let db_lock = db.blocking_lock();

        let mut stmt = if album_str.is_empty() {
            db_lock.prepare("SELECT * FROM songs ORDER BY RANDOM() LIMIT ?1")?
        } else {
            db_lock.prepare("SELECT * FROM songs WHERE album = ?1 ORDER BY RANDOM() LIMIT ?2")?
        };

        let map_row = |row: &Row<'_>| {
            Ok(Song {
                id: row.get(0)?,
                title: row.get(1)?,
                album: row.get(2)?,
            })
        };

        let song_iter = if album_str.is_empty() {
            stmt.query_map(params![amount_to_query], map_row)?
        } else {
            stmt.query_map(params![album_str, amount_to_query], map_row)?
        };

        // Collect all the songs
        let mut songs = Vec::new();
        for song_result in song_iter {
            songs.push(song_result?);
        }

        Ok::<_, rusqlite::Error>(songs)
    })
    .await??;

    if songs.is_empty() {
        let song_empty_embed = serenity::CreateEmbed::new()
            .title("Error")
            .color(Color::RED)
            .description("Couldn't retrieve any songs. No songs found in database.");
        ctx.send(poise::CreateReply::default().embed(song_empty_embed))
            .await?;
    } else {
        let formatted = songs
            .iter()
            .enumerate()
            .map(|(i, s)| format!("{}. {} — {} [ID: {}]", i + 1, s.title, s.album, s.id))
            .collect::<Vec<_>>()
            .join("\n");

        let random_song_embed = serenity::CreateEmbed::new()
            .title("Random Result(s)")
            .color(Color::BLUE)
            .description(formatted);

        ctx.send(poise::CreateReply::default().embed(random_song_embed))
            .await?;
    }

    Ok(())
}
