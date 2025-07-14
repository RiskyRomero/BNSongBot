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

    if album == "" {
        let list_str = tokio::task::spawn_blocking(move || {
            let db_lock = db.blocking_lock(); // NOTE: blocking_lock in spawn_blocking

            let mut stmt = db_lock.prepare("SELECT * FROM songs").unwrap(); // handle properly in real code
            let song_iter = stmt
                .query_map([], |row| {
                    Ok(Song {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        album: row.get(2)?,
                    })
                })
                .unwrap();

            let mut list_str = String::new();
            for song_result in song_iter {
                let song = song_result.unwrap(); // handle properly
                list_str += &format!("{} - {} ({})\n", song.id, song.title, song.album);
            }

            list_str
        })
        .await
        .unwrap(); // unwrap() is safe here unless thread panicked

        ctx.say(list_str).await?;
    } else {
        let list_str = tokio::task::spawn_blocking(move || {
            let db_lock = db.blocking_lock(); // NOTE: blocking_lock in spawn_blocking

            let mut stmt = db_lock
                .prepare("SELECT * FROM songs WHERE album = ?1")
                .unwrap(); // handle properly in real code
            let song_iter = stmt
                .query_map([album], |row| {
                    Ok(Song {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        album: row.get(2)?,
                    })
                })
                .unwrap();

            let mut list_str = String::new();
            for song_result in song_iter {
                let song = song_result.unwrap(); // handle properly
                list_str += &format!("{} - {} ({})\n", song.id, song.title, song.album);
            }

            list_str
        })
        .await
        .unwrap(); // unwrap() is safe here unless thread panicked

        ctx.say(list_str).await?;
    }

    Ok(())
}
