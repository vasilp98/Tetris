use crate::constants::*;

use std::mem;
use ggez::graphics::{self, Rect, Color, MeshBuilder, DrawMode, DrawParam};
use ggez::mint::Point2;
use ggez::Context;
use ggez::GameResult;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
    thread_rng
};

#[derive(Clone, Copy, Debug)]
pub enum BlockType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z
}

impl Distribution<BlockType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockType {
        match rng.gen_range(0..=6) {
            0 => BlockType::I,
            1 => BlockType::J,
            2 => BlockType::L,
            3 => BlockType::O,
            4 => BlockType::S,
            5 => BlockType::T,
            6 => BlockType::Z,
            _ => BlockType::I
        }
    }
}

pub struct Square {
    pub row: f32,
    pub column: f32,
    pub color: Color,
    pub component: Rect
}

impl Square {
    pub fn new(row: f32, column: f32, color: Color) -> Self {
        Square {
            row,
            column,
            color,
            component: Rect::new(BORDER_SIZE, BORDER_SIZE, SQUARE_SIZE - (BORDER_SIZE * 2.0), SQUARE_SIZE - (BORDER_SIZE * 2.0))
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut mesh = MeshBuilder::new();
        mesh.rectangle(DrawMode::fill(), self.component, self.color);

        let mesh = &mesh.build(ctx).unwrap();
        graphics::draw(ctx, mesh, DrawParam {
            dest: Point2 {
                x: self.column * SQUARE_SIZE + ENTRY_POINT.0,
                y: self.row * SQUARE_SIZE + ENTRY_POINT.1,
            },
            .. Default::default()
        }).unwrap();
        
        Ok(())
    }
}

#[derive(Clone)]
pub struct Block {
    pub positions: Vec<(f32, f32)>,
    pub translate: (f32, f32),
    block_type: BlockType,
    //speed: f32,
    color: Color
}

impl Block {
    pub fn new(block_type: BlockType) -> Self {
        let positions: Vec<(f32, f32)>;
        match block_type {
            BlockType::I => positions = vec!((0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (3.0, 0.0)),
            BlockType::J => positions = vec!((0.0, 1.0), (1.0, 1.0), (2.0, 1.0), (2.0, 0.0)),
            BlockType::L => positions = vec!((0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (2.0, 1.0)),
            BlockType::O => positions = vec!((0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)),
            BlockType::S => positions = vec!((0.0, 1.0), (0.0, 2.0), (1.0, 0.0), (1.0, 1.0)),
            BlockType::T => positions = vec!((0.0, 0.0), (0.0, 1.0), (0.0, 2.0), (1.0, 1.0)),
            BlockType::Z => positions = vec!((0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 2.0)),
        }
        
        let mut rng = thread_rng();
        let random_color = Color::new(rng.gen_range(0.5..1.0), rng.gen_range(0.5..1.0), rng.gen_range(0.5..1.0), rng.gen_range(0.5..1.0));

        Block {
            block_type,
            positions,
            color: random_color,
            //speed: 0.0,
            translate: (0.0, 0.0)
        }
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        let speed: f32 = y * 2.0;

        self.translate.0 += x;
        self.translate.1 += speed;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        for pos in self.positions.iter() {
            let mut mesh = MeshBuilder::new();
            mesh.rectangle(DrawMode::fill(), Square::new(0.0, 0.0, self.color).component, self.color);

            let mesh = &mesh.build(ctx).unwrap();
            graphics::draw(ctx, mesh, DrawParam {
                dest: Point2 {
                    x: (pos.0 + self.translate.0) * SQUARE_SIZE + ENTRY_POINT.0,
                    y: (pos.1 + self.translate.1) * SQUARE_SIZE + ENTRY_POINT.1,
                },
                .. Default::default()
            }).unwrap();
        }

        Ok(())
    }

    pub fn to_squares(&mut self) -> Vec<Square> {
        let mut squares: Vec<Square> = Vec::new();
        for pos in self.positions.iter() {
            let row = (pos.1 + self.translate.1).round();
            let column = (pos.0 + self.translate.0).round();

            squares.push(Square::new(row, column, self.color));
        }

        squares
    }

    pub fn rotate(&mut self) {
        match self.block_type {
            BlockType::J | BlockType::L | BlockType::S | BlockType::T | BlockType::Z => self.rotate_easy_blocks(),
            BlockType::I => {
                for pos in self.positions.iter_mut() {
                    mem::swap(&mut pos.0, &mut pos.1);
                }
            },
            _ => ()//Do nothing
        }

        let mut move_left: f32 = 0.0;
        for pos in self.positions.iter() {
            if pos.0 + self.translate.0 + 1.0 > BOARD_WIDTH / SQUARE_SIZE {
                if move_left < pos.0 + self.translate.0 + 1.0 - BOARD_WIDTH / SQUARE_SIZE {
                    move_left = pos.0 + self.translate.0 + 1.0 - BOARD_WIDTH / SQUARE_SIZE;
                }
            }
        }

        self.translate.0 -= move_left;
    }

    fn rotate_easy_blocks(&mut self) {
        let mut new_positions: Vec<(f32, f32)> = Vec::new();
        for pos in self.positions.iter_mut() {
            mem::swap(&mut pos.0, &mut pos.1);
        }

        let diagonal_matrix = [(0.0, 2.0), (1.0, 1.0), (2.0, 0.0)];

        for pos in self.positions.iter() {
            for value in diagonal_matrix.iter() {
                if pos.1 == value.1 {
                    new_positions.push((pos.0, value.0));
                }
            }
        }

        self.positions = new_positions;
    }

    fn should_stop(&mut self, squares: &Vec<Square>) -> bool {
        for pos in self.positions.iter() {
            for square in squares.iter() {
                if (pos.1 + self.translate.1 + 1.0) >= square.row &&
                   (pos.1 + self.translate.1 + 1.0) <= square.row + 1.0 &&
                    pos.0 + self.translate.0 == square.column {
                    return true;
                } 
            }

            if pos.1 + self.translate.1 + 1.0 > BOARD_HEIGHT / SQUARE_SIZE {
                return true;
            }
        }

        return false;
    }

    pub fn will_collide(&mut self, squares: &Vec<Square>, movement: f32) -> bool {
        for pos in self.positions.iter() {
            let square_column = (pos.0 + self.translate.0 + movement).round();
            let square_row = (pos.1 + self.translate.1).round();
            if let Some(_) = squares.iter().find(|s| s.column == square_column && s.row == square_row) {
                return true;
            }

            if pos.0 + self.translate.0 + movement + 1.0 > BOARD_WIDTH / SQUARE_SIZE ||
               pos.0 + self.translate.0 + movement < 0.0 {
                return true;
            }
        }

        return self.should_stop(squares);
    }
}