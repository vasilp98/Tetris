use crate::constants::*;

use std::mem;
use ggez::graphics::{self, Rect, Color, MeshBuilder, DrawMode, DrawParam};
use ggez::mint::Point2;
use ggez::Context;
use ggez::GameResult;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
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
    component: Rect
}

impl Square {
    fn new() -> Self {
        Square {
            component: Rect::new(BORDER_SIZE, BORDER_SIZE, SQUARE_SIZE - (BORDER_SIZE * 2.0), SQUARE_SIZE - (BORDER_SIZE * 2.0)
            )
        }
    }
}

#[derive(Clone)]
pub struct Block {
    block_type: BlockType,
    pub positions: Vec<(f32, f32)>,
    color: Color,
    speed: f32,
    pub translate: (f32, f32)
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

        Block {
            block_type,
            positions,
            color: Color::new(42. / 255., 200. / 255., 255. / 255., 1.0),
            speed: 0.0,
            translate: (0.0, 0.0)
        }
    }

    pub fn update_speed(&mut self, seconds: f32) -> bool {
        let speed: f32 = seconds * 2.0;

        for pos in self.positions.iter() {
            if pos.1 + self.translate.1 + speed + 1.0 > WINDOW_HEIGHT / SQUARE_SIZE {
                return false;
            }
        }

        self.translate.1 += speed;

        return true;
    }

    pub fn update_position(&mut self, offset: f32) {
        for pos in self.positions.iter() {
            if pos.0 + self.translate.0 + offset + 1.0 > WINDOW_WIDTH / SQUARE_SIZE ||
               pos.0 + self.translate.0 + offset < 0.0 {
                return;
            }
        }
    
        self.translate.0 += offset;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        for pos in self.positions.iter() {
            let mut mesh = MeshBuilder::new();
            mesh.rectangle(DrawMode::fill(), Square::new().component, self.color);

            let mesh = &mesh.build(ctx).unwrap();
            graphics::draw(ctx, mesh, DrawParam {
                dest: Point2 {
                    x: ((pos.0 + self.translate.0) * SQUARE_SIZE).floor(),
                    y: ((pos.1 + self.translate.1) * SQUARE_SIZE).floor(),
                },
                .. Default::default()
            }).unwrap();
        }

        Ok(())
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
            if pos.0 + self.translate.0 + 1.0 > WINDOW_WIDTH / SQUARE_SIZE {
                if move_left < pos.0 + self.translate.0 + 1.0 - WINDOW_WIDTH / SQUARE_SIZE {
                    move_left = pos.0 + self.translate.0 + 1.0 - WINDOW_WIDTH / SQUARE_SIZE;
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

    pub fn will_collide(&mut self, blocks: &Vec<Block>) -> bool {
        for pos in self.positions.iter() {
            for block in blocks.iter() {
                for other_pos in block.positions.iter() {
                    if (pos.1 + self.translate.1 + 1.0) >= other_pos.1 + block.translate.1 &&
                       (pos.1 + self.translate.1 + 1.0) <= other_pos.1 + block.translate.1 + 1.0 &&
                        pos.0 + self.translate.0 == other_pos.0 + block.translate.0 {
                        return true;
                    } 
                }
            }
        }

        return false;
    }

    pub fn can_move_horizontal(&mut self, blocks: &Vec<Block>, movement: f32) -> bool {
        for pos in self.positions.iter() {
            for block in blocks.iter() {
                for other_pos in block.positions.iter() {
                    if ((movement > 0.0 && pos.0 + self.translate.0 + movement >= other_pos.0 + block.translate.0) || 
                       (movement < 0.0 && pos.0 + self.translate.0 + movement <= other_pos.0 + block.translate.0)) &&
                       (pos.1 + self.translate.1 + 1.0) >= other_pos.1 + block.translate.1 &&
                       (pos.1 + self.translate.1 + 1.0) <= other_pos.1 + block.translate.1 + 1.0 {
                        return false;
                    } 
                }
            }
        }

        return true;
    }
}