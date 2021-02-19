use ggez::audio::{self, SoundSource, Source};
use ggez::mint::Point2;
use ggez::graphics;
use ggez::graphics::{DrawParam};
use ggez::Context;
use ggez::GameResult;

use crate::constants::*;
use crate::block::*;

pub struct Bomb {
    pub pos: Point2<f32>,
    image: graphics::Image,
    pub sound: Source
}

impl Bomb {
    pub fn new(ctx: &mut Context) -> GameResult<Bomb> {
        let image = graphics::Image::new(ctx, "/bomb.png")?;
        let sound = audio::Source::new(ctx, "/bomb.ogg")?;
        
        Ok(Bomb {
            pos: Point2 {
                x: ENTRY_POINT.0,
                y: ENTRY_POINT.1
            },
            image,
            sound
        })
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(ctx, &self.image, DrawParam {
            dest: self.pos,
            .. Default::default()
        })
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        let speed: f32 = y * SQUARE_SIZE;

        self.pos.x += x;
        self.pos.y += speed;
    }

    fn should_stop(&mut self, squares: &Vec<Square>) -> bool {
        for square in squares.iter() {
            if self.pos.y + self.image.height() as f32 >= (square.row + 1.0) * SQUARE_SIZE &&
               self.pos.y <= square.row * SQUARE_SIZE &&
               self.pos.x == (square.column + 1.0) * SQUARE_SIZE {
               
                return true;
            } 
        }

        if self.pos.y + self.image.height() as f32 > BOARD_HEIGHT + ENTRY_POINT.0 {
            return true;
        }

        return false;
    }

    pub fn will_collide(&mut self, squares: &Vec<Square>, movement: f32, speed: f32) -> bool {
        let square_column = ((self.pos.x + movement) / SQUARE_SIZE).round() - 1.0;
        let square_row = ((self.pos.y) / SQUARE_SIZE + speed).round() - 1.0;
        if let Some(_) = squares.iter().find(|s| s.column == square_column && s.row == square_row) {
            return true;
        }

        if self.pos.x + movement > BOARD_WIDTH ||
           self.pos.x + movement < ENTRY_POINT.1 {
            return true;
        }
    
        return self.should_stop(squares);
    }

    pub fn explode(&mut self) {
        let _ = self.sound.play_detached();
    }
}