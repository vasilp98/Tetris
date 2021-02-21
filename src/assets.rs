use ggez::audio;
use ggez::{Context, GameResult};

pub struct Assets {
    pub theme_song: audio::Source
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let theme_song = audio::Source::new(ctx, "/tetris_theme_song.mp3")?;

        Ok(Assets {
            theme_song
        })
    }
}