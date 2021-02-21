mod block;
mod input;
mod constants;
mod assets;
mod bomb;
mod configuration;

use crate::constants::*;
use crate::block::*;
use crate::input::*;
use crate::assets::*;
use crate::bomb::*;
use crate::configuration::*;

use ggez::event;
use ggez::audio::{SoundSource};
use ggez::filesystem;
use ggez::graphics::{self, TextFragment, Scale, Font, Text, Rect, Color, MeshBuilder, DrawMode, DrawParam};
use ggez::input as ggez_input;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{EventHandler};
use ggez::mint::Point2;
use rand::{ Rng, thread_rng };
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

    let mut tetris_game = Tetris::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut tetris_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct Tetris {
    current_block: Block,
    next_block: Block,
    squares: Vec<Square>,
    input: Input,
    viewing_area_start_row: i32,
    bomb: Option<Bomb>,
    game_over: bool,
    configuration: Configuration,
    lines_block_count: Vec<i32>,
    lines: i32,
    score: i32,
    speed: f32,
    level: i32,
    ticks: i32
}

impl Tetris {
    const ROTATION_INTERVAL: i32 = 5;
    const MOVE_INTERVAL: i32 = 5;

    pub fn new(ctx: &mut Context) -> Tetris {
        let mut assets = Assets::new(ctx).unwrap();
        assets.theme_song.set_repeat(true);
        let _ = assets.theme_song.play_detached();

        let configuration = Configuration::new();

        Tetris
        {
            current_block: Block::new(rand::random(), configuration.clone()), 
            next_block: Block::new(rand::random(), configuration.clone()),
            squares: Vec::new(),
            input: Input::default(),
            viewing_area_start_row: 0,
            bomb: None,
            game_over: false,
            configuration: configuration.clone(),
            lines_block_count: vec![0; (BOARD_HEIGHT / SQUARE_SIZE) as usize],
            lines: 0,
            score: 0,
            speed: configuration.default_speed(),
            level: 1,
            ticks: 0
        }
    }

    fn clear_line(&mut self, line: usize) {
        self.lines += 1;

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

    fn draw_next_block(&self, ctx: &mut Context) -> GameResult<()> {
        for square in self.next_block.to_squares() {
            let mut mesh = MeshBuilder::new();
            mesh.rectangle(DrawMode::fill(), square.component, square.color);

            let mesh = &mesh.build(ctx).unwrap();
            graphics::draw(ctx, mesh, DrawParam {
                dest: Point2 {
                    x: (((BOARD_WIDTH + 2.0 * SQUARE_SIZE) / SQUARE_SIZE) + 2.0 + square.column) * SQUARE_SIZE,
                    y: (2.0 + square.row) * SQUARE_SIZE,
                },
                .. Default::default()
            }).unwrap();
        }

        Ok(())
    }

    fn draw_borders(&self, ctx: &mut Context) -> GameResult<()> {
        let color = graphics::WHITE;
        let top = Rect::new(0.0, 0.0, WINDOW_WIDTH, ENTRY_POINT.1);
        let left = Rect::new(0.0, 0.0, ENTRY_POINT.0, 2.0 * SQUARE_SIZE + BOARD_HEIGHT);
        let bottom = Rect::new(0.0, BOARD_HEIGHT + ENTRY_POINT.1, 2.0 * SQUARE_SIZE + BOARD_WIDTH, ENTRY_POINT.1);
        let right = Rect::new(BOARD_WIDTH + ENTRY_POINT.0, 0.0, ENTRY_POINT.0, 2.0 * SQUARE_SIZE + BOARD_HEIGHT);

        self.draw_border(ctx, top, color).unwrap();
        self.draw_border(ctx, bottom, color).unwrap();
        self.draw_border(ctx, left, color).unwrap();
        self.draw_border(ctx, right, color)
    }
    
    fn draw_border(&self, ctx: &mut Context, border: Rect, color: Color) -> GameResult<()> {
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

    fn update_score(&mut self, lines_count: i32) {
        let mut multiplier = 1.0;
        if !self.configuration.classic_mode() {
            multiplier = 2.0 + (BOARD_HEIGHT / SQUARE_SIZE - self.configuration.viewing_area_rows_count() as f32) / 10.0;
        }
        match lines_count {
            1 => self.score += (SINGLE_LINE_POINTS as f32 * multiplier).round() as i32,
            2 => self.score += (DOUBLE_LINE_POINTS as f32 * multiplier).round() as i32,
            3 => self.score += (TRIPLE_LINE_POINTS as f32 * multiplier).round() as i32,
            4 => self.score += (TETRIS_POINTS as f32 * multiplier).round() as i32,
            _ => () //Do nothing
        }
    }

    fn draw_text(&self, ctx: &mut Context, text: String, dest: Point2<f32>) -> GameResult<()> {
        let mut text_fragment = TextFragment::new(text);
        text_fragment.color = Some(graphics::WHITE);
        text_fragment.scale = Some(Scale { x: 20.0, y: 24.0 });
        text_fragment.font = Some(Font::new(ctx, "/tetris_block.ttf")?);
        let text = Text::new(text_fragment);

        graphics::draw(ctx, &text, DrawParam {
            dest,
            .. Default::default()
        }).unwrap();

        Ok(())
    }   
    
    fn update_bomb(&mut self, speed: f32) {
        let bomb = self.bomb.as_mut().unwrap();
        bomb.translate(0.0, speed);

        if bomb.will_collide(&self.squares, 0.0, speed) {
            self.explode_bomb();

            self.current_block = self.next_block.clone();
            self.next_block = Block::new(rand::random(), self.configuration.clone());
        }
    }

    fn explode_bomb(&mut self) {
        let bomb = self.bomb.as_mut().unwrap();
        bomb.explode();

        let row = (bomb.pos.y / SQUARE_SIZE).round() - 1.0;
        let column = (bomb.pos.x / SQUARE_SIZE).round() - 1.0;
        
        self.squares.retain(|s| row - 1.0 > s.row || s.row > row + 1.0 || column - 1.0 > s.column || s.column > column + 1.0);

        self.lines_block_count = vec![0; (BOARD_HEIGHT / SQUARE_SIZE) as usize];
        for square in self.squares.iter() {
            self.lines_block_count[square.row as usize] += 1;
        }
        
        self.bomb = None;
    }

    fn update_viewing_area(&mut self) {
        if (self.viewing_area_start_row + self.configuration.viewing_area_rows_count() + self.input.viewing_area_movement) as f32 > BOARD_HEIGHT / SQUARE_SIZE ||
            self.viewing_area_start_row + self.input.viewing_area_movement < 0 {
            
            return;
        }  

        self.viewing_area_start_row += self.input.viewing_area_movement;
    }
}

impl EventHandler for Tetris {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.game_over {
            return Ok(());
        }

        self.ticks += 1;
        
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            let speed = (seconds + self.input.speed_boost + self.speed) * 2.0;
            if let Some(_) = self.bomb {
                self.update_bomb(speed);
            }
            else if !self.translate_current_block(0.0, speed) {
                for square in self.current_block.to_squares() {
                    if square.row == 0.0 {
                        self.game_over = true;
                    }

                    self.lines_block_count[square.row as usize] += 1;
                    self.squares.push(square);
                }

                let mut lines_count = 0;
                for i in 0..self.lines_block_count.len() {
                    let line_block_count = self.lines_block_count[i];
                    if line_block_count == 10 {
                        lines_count += 1;
                        self.clear_line(i);
                    }
                }

                self.update_score(lines_count);

                if self.lines >= self.configuration.lines_to_level_up() {
                    self.level += 1;
                    self.lines = 0;
                    self.speed = self.configuration.default_speed() * self.level as f32;
                }

                let mut rng = thread_rng();
                if rng.gen_range(0..4) == 1 && !self.configuration.classic_mode() {
                    self.bomb = Some(Bomb::new(ctx).unwrap());
                }
                else {
                    self.current_block = self.next_block.clone();
                    self.next_block = Block::new(rand::random(), self.configuration.clone());
                }
                return Ok(());
            }
        }

        let current_ticks = self.ticks;
        if current_ticks >= Tetris::ROTATION_INTERVAL {
            if self.input.rotate {
                let old_positions = self.current_block.positions.clone();
                
                self.current_block.rotate();
                if self.current_block.will_collide(&self.squares, 0.0) {
                    self.current_block.positions = old_positions;
                }
            }

            self.update_viewing_area();

            self.ticks = 0;
        }

        if current_ticks >= Tetris::MOVE_INTERVAL {
            self.translate_current_block(self.input.movement, 0.0);
            
            if let Some(bomb) = &mut self.bomb {
                if !bomb.will_collide(&self.squares, self.input.movement * SQUARE_SIZE, 0.0) {
                    bomb.translate(self.input.movement * SQUARE_SIZE, 0.0);
                }
            }

            self.ticks = 0;
        }
            
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        if self.game_over {
            self.draw_text(ctx, format!("GAME OVER! SCORE: {}", self.score), Point2 {
                x: (WINDOW_WIDTH - 400.0) / 2.0,
                y: (WINDOW_HEIGHT - 50.0) / 2.0,
            })?;

            graphics::present(ctx)?;
            return Ok(())
        }

        for square in self.squares.iter() {
            if square.row < (self.viewing_area_start_row + self.configuration.viewing_area_rows_count()) as f32 &&
               square.row >= self.viewing_area_start_row as f32 {

                square.draw(ctx).unwrap();
            } 
        }

        if !self.configuration.classic_mode() {
            // The borders of the viewing area
            self.draw_border(ctx, Rect::new(0.0, ENTRY_POINT.0 + self.viewing_area_start_row as f32 * SQUARE_SIZE - 5.0, 2.0 * SQUARE_SIZE + BOARD_WIDTH, 5.0), graphics::WHITE).unwrap();
            self.draw_border(ctx, Rect::new(0.0, ENTRY_POINT.0 + (self.viewing_area_start_row + self.configuration.viewing_area_rows_count()) as f32 * SQUARE_SIZE, 2.0 * SQUARE_SIZE + BOARD_WIDTH, 5.0), graphics::WHITE).unwrap();
        }

        if let Some(bomb) = &mut self.bomb {
            let row = (bomb.pos.y / SQUARE_SIZE).round() - 1.0;
            if row < (self.viewing_area_start_row + self.configuration.viewing_area_rows_count()) as f32 && row > self.viewing_area_start_row as f32 {
                bomb.draw(ctx).unwrap();
            }
        }
        else {
            self.current_block.draw(ctx, self.viewing_area_start_row).unwrap();

            // for square in self.current_block.to_squares().iter_mut() {
            //     if square.row < (self.viewing_area_start_row + self.configuration.viewing_area_rows_count()) as f32 && square.row >= self.viewing_area_start_row as f32 {
            //         square.draw(ctx).unwrap();
            //     } 
            // }
        }

        self.draw_next_block(ctx).unwrap();
        self.draw_borders(ctx).unwrap();
        self.draw_text(ctx, format!("score: {}", self.score.to_string()), Point2 { x: 12.5 * SQUARE_SIZE, y: 6.0 * SQUARE_SIZE }).unwrap();
        self.draw_text(ctx, format!("level: {}", self.level.to_string()), Point2 { x: 12.5 * SQUARE_SIZE, y: 8.0 * SQUARE_SIZE }).unwrap();

        graphics::present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: event::KeyCode, _keymod: ggez_input::keyboard::KeyMods, _repeat: bool) {
        match keycode {
            event::KeyCode::Space => self.input.rotate = true,
            event::KeyCode::Left => self.input.movement = -1.0,
            event::KeyCode::Right => self.input.movement = 1.0,
            event::KeyCode::W => self.input.viewing_area_movement = -1,
            event::KeyCode::S => self.input.viewing_area_movement = 1,
            event::KeyCode::Down => self.input.speed_boost = 0.1,
            event::KeyCode::Escape => event::quit(ctx),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: event::KeyCode, _keymod: ggez_input::keyboard::KeyMods) {
        match keycode {
            event::KeyCode::Space => self.input.rotate = false,
            event::KeyCode::Left | event::KeyCode::Right => self.input.movement = 0.0,
            event::KeyCode::W | event::KeyCode::S => self.input.viewing_area_movement = 0,
            | event::KeyCode::Down => self.input.speed_boost = 0.0,
            _ => (), // Do nothing
        }
    }
}
