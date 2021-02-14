mod block;
mod input;
mod constants;

use crate::constants::*;
use crate::block::*;
use crate::input::*;

use ggez::event;
use ggez::filesystem;
use ggez::graphics;
use ggez::input as ggez_input;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{EventHandler};

use std::env;
use std::path;

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Tetris", "Vasil")
        .window_setup(ggez::conf::WindowSetup::default().title("Tetris"))
        .window_mode(
            ggez::conf::WindowMode::default()
            .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
       )
       .build()
       .unwrap();


    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        filesystem::mount(&mut ctx, &path, true);
    }

    let mut my_game = Tetris::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct Tetris {
    current_block: Block,
    blocks: Vec<Block>,
    input: Input,
    tick_interval: i32,
    lines_block_count: Vec<i32>
}

impl Tetris {
    pub fn new(_ctx: &mut Context) -> Tetris {
        let block_type: BlockType = rand::random();
        let current_block = Block::new(block_type); 
        Tetris 
        {
            current_block,  
            blocks: Vec::new(),
            input: Input::default(),
            tick_interval: 0,
            lines_block_count: vec![0; (WINDOW_HEIGHT / SQUARE_SIZE) as usize]
        }
    }
}

impl EventHandler for Tetris {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.tick_interval += 1;
        
        const DESIRED_FPS: u32 = 120;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            let is_moving = self.current_block.update_speed(seconds + self.input.speed_boost);
            if is_moving == false || self.current_block.will_collide(&self.blocks) {
                self.blocks.push(self.current_block.clone());
                
                for pos in self.current_block.positions.iter() {
                    let line = pos.0 + self.current_block.translate.0;
                    self.lines_block_count[line.floor() as usize] += 1;
                }

                let block_type: BlockType = rand::random();
                self.current_block = Block::new(block_type);
                return Ok(());
            }
        }
        
        if self.tick_interval >= 3 {
            self.tick_interval = 0;

            if self.input.rotate {
                self.current_block.rotate();
            }
            
            if self.current_block.can_move_horizontal(&self.blocks, self.input.movement) {
                self.current_block.update_position(self.input.movement);
            }
        }
            
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        self.current_block.draw(ctx).unwrap();

        for block in self.blocks.iter_mut() {
            block.draw(ctx).unwrap();
        }

        graphics::present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: event::KeyCode, _keymod: ggez_input::keyboard::KeyMods, _repeat: bool) {
        match keycode {
            event::KeyCode::Space => self.input.rotate = true,
            event::KeyCode::Left => self.input.movement = -1.0,
            event::KeyCode::Right => self.input.movement = 1.0,
            event::KeyCode::Down => self.input.speed_boost = 0.05,
            event::KeyCode::Escape => event::quit(ctx),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: event::KeyCode, _keymod: ggez_input::keyboard::KeyMods) {
        match keycode {
            event::KeyCode::Space => self.input.rotate = false,
            event::KeyCode::Left | event::KeyCode::Right => self.input.movement = 0.0,
            | event::KeyCode::Down => self.input.speed_boost = 0.0,
            _ => (), // Do nothing
        }
    }
}
