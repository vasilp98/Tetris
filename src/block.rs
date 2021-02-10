use crate::constants::*;

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

#[derive(Clone, Copy, Debug)]
pub struct Block {
    block_type: BlockType,
    positions: [(f32, f32); 4],
    color: Color,
    speed: f32
}

impl Block {
    pub fn new(block_type: BlockType) -> Self {
        let positions: [(f32, f32); 4];
        match block_type {
            BlockType::I => positions = [(0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (3.0, 0.0)],
            BlockType::J => positions = [(0.0, 1.0), (1.0, 1.0), (2.0, 1.0), (2.0, 0.0)],
            BlockType::L => positions = [(0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (2.0, 1.0)],
            BlockType::O => positions = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)],
            BlockType::S => positions = [(0.0, 1.0), (0.0, 2.0), (1.0, 0.0), (1.0, 1.0)],
            BlockType::T => positions = [(0.0, 0.0), (0.0, 1.0), (0.0, 2.0), (1.0, 1.0)],
            BlockType::Z => positions = [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 2.0)],
        }

        Block {
            block_type,
            positions,
            color: Color::new(42. / 255., 200. / 255., 255. / 255., 1.0),
            speed: 0.0
        }
    }

    pub fn update_speed(&mut self, seconds: f32) -> bool {
        let speed: f32 = seconds * 2.0;

        for pos in self.positions.iter() {
            if pos.1 + speed + 1.0 > WINDOW_HEIGHT / SQUARE_SIZE {
                return false;
            }
        }
        
        for pos in self.positions.iter_mut() {
            pos.1 += speed;
        }

        return true;
    }

    pub fn update_position(&mut self, offset: f32) {
        for pos in self.positions.iter() {
            if pos.0 + offset + 1.0 > WINDOW_WIDTH / SQUARE_SIZE || pos.0 + offset < 0.0 {
                return;
            }
        }
        
        for pos in self.positions.iter_mut() {
            pos.0 += offset;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        for pos in self.positions.iter() {
            let mut mesh = MeshBuilder::new();
            mesh.rectangle(DrawMode::fill(), Square::new().component, self.color);

            let mesh = &mesh.build(ctx).unwrap();
            graphics::draw(ctx, mesh, DrawParam {
                dest: Point2 {
                    x: f32::from(pos.0) * SQUARE_SIZE,
                    y: f32::from(pos.1) * SQUARE_SIZE,
                },
                .. Default::default()
            }).unwrap();
        }

        Ok(())
    }
}