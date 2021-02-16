mod block;
mod input;
mod constants;

use crate::constants::*;
use crate::block::*;
use crate::input::*;

use ggez::event;
use ggez::filesystem;
use ggez::graphics::{self, Rect, Color, MeshBuilder, DrawMode, DrawParam};
use ggez::input as ggez_input;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{EventHandler};
use ggez::mint::Point2;

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
    next_block: Block,
    blocks: Vec<Block>,
    squares: Vec<Square>,
    input: Input,
    ticks: i32,
    lines_block_count: Vec<i32>
}

impl Tetris {
    pub fn new(_ctx: &mut Context) -> Tetris {
        Tetris 
        {
            current_block: Block::new(rand::random()), 
            next_block: Block::new(rand::random()),
            blocks: Vec::new(),
            squares: Vec::new(),
            input: Input::default(),
            ticks: 0,
            lines_block_count: vec![0; (BOARD_HEIGHT / SQUARE_SIZE) as usize]
        }
    }

    fn clear_line(&mut self, line: usize) {
        self.squares.retain(|s| s.row != line as f32);
        self.lines_block_count[line] = 0;

        for square in self.squares.iter_mut() {
            if square.row < line as f32 {
                square.row += 1.0;
            }
        } 

        for i in (0..line + 1).rev() {
            if i == 0 {
                self.lines_block_count[i] = 0;
            }
            else {
                self.lines_block_count[i] = self.lines_block_count[i - 1];
            }
        }
    } 

    fn translate_current_block(&mut self, x: f32, y: f32) -> bool {
        if self.current_block.will_collide(&self.squares, x) {
            return false;
        }

        self.current_block.translate(x, y);
        true
    }

    fn draw_next_block(&mut self, ctx: &mut Context) -> GameResult<()> {
        for square in self.next_block.to_squares() {
            let mut mesh = MeshBuilder::new();
            mesh.rectangle(DrawMode::fill(), square.component, square.color);

            let mesh = &mesh.build(ctx).unwrap();
            graphics::draw(ctx, mesh, DrawParam {
                dest: Point2 {
                    x: (((BOARD_WIDTH + 2.0 * SQUARE_SIZE) / SQUARE_SIZE) + 1.0 + square.column) * SQUARE_SIZE,
                    y: (2.0 + square.row) * SQUARE_SIZE,
                },
                .. Default::default()
            }).unwrap();
        }

        Ok(())
    }

    fn draw_borders(&mut self, ctx: &mut Context) -> GameResult<()> {
        let color = Color::new(1.0, 1.0, 1.0, 1.0);
        let top = Rect::new(0.0, 0.0, 2.0 * SQUARE_SIZE + BOARD_WIDTH, ENTRY_POINT.1);
        let left = Rect::new(0.0, 0.0, ENTRY_POINT.0, 2.0 * SQUARE_SIZE + BOARD_HEIGHT);
        let bottom = Rect::new(0.0, BOARD_HEIGHT + ENTRY_POINT.1, 2.0 * SQUARE_SIZE + BOARD_WIDTH, ENTRY_POINT.1);
        let right = Rect::new(BOARD_WIDTH + ENTRY_POINT.0, 0.0, ENTRY_POINT.0, 2.0 * SQUARE_SIZE + BOARD_HEIGHT);

        self.draw_border(ctx, top, color).unwrap();
        self.draw_border(ctx, bottom, color).unwrap();
        self.draw_border(ctx, left, color).unwrap();
        self.draw_border(ctx, right, color)
    }
    
    fn draw_border(&mut self, ctx: &mut Context, border: Rect, color: Color) -> GameResult<()> {
        let mut mesh = MeshBuilder::new();
        mesh.rectangle(DrawMode::fill(), border, color);

        let mesh = &mesh.build(ctx).unwrap();
        graphics::draw(ctx, mesh, DrawParam {
            dest: Point2 {
                x: 0.0,
                y: 0.0,
            },
            .. Default::default()
        }).unwrap();

        Ok(())
    }
}

impl EventHandler for Tetris {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.ticks += 1;
        
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            if !self.translate_current_block(0.0, seconds + self.input.speed_boost) {
                self.blocks.push(self.current_block.clone());

                let squares = self.current_block.to_squares();
                for square in squares {
                    if square.row == 0.0 {
                        event::quit(ctx);
                    }

                    self.lines_block_count[square.row  as usize] += 1;
                    self.squares.push(square);
                }

                for i in 0..self.lines_block_count.len() {
                    let line_block_count = self.lines_block_count[i];
                    if line_block_count == 10 {
                        self.clear_line(i);
                    }
                }

                self.current_block = self.next_block.clone();
                self.next_block = Block::new(rand::random());
                return Ok(());
            }
        }

        if self.ticks >= ROTATION_INTERVAL {
            if self.input.rotate {
                let old_positions = self.current_block.positions.clone();
                
                self.current_block.rotate();
                if self.current_block.will_collide(&self.squares, 0.0) {
                    self.current_block.positions = old_positions;
                }
            }
            
            if self.ticks > MOVE_INTERVAL {
                self.translate_current_block(self.input.movement, 0.0);
            }

            self.ticks = 0;
        }
            
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        self.current_block.draw(ctx).unwrap();

        for square in self.squares.iter_mut() {
            square.draw(ctx).unwrap();
        }

        self.draw_next_block(ctx).unwrap();

        self.draw_borders(ctx).unwrap();

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
