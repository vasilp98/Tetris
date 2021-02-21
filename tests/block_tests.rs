use tetris::block::*;
use ggez::graphics;
use tetris::configuration::*;
use tetris::constants::*;

#[test]
fn block_validate_will_stop() {
    let mut block = Block::new(BlockType::Z, Configuration::new());
    block.translate(0.0, 15.0);

    let squares = vec!(Square::new(15.0, 0.0, graphics::BLACK));

    assert!(block.will_collide(&squares, 0.0));
}

#[test]
fn block_validate_cannot_move_horizontally_left_border() {
    let mut block = Block::new(BlockType::Z, Configuration::new());
    block.translate(0.0, 0.0);

    assert!(block.will_collide(&Vec::new(), -1.0));
}

#[test]
fn block_validate_cannot_move_horizontally_right_border() {
    let mut block = Block::new(BlockType::Z, Configuration::new());
    block.translate(BOARD_WIDTH / SQUARE_SIZE, 0.0);

    assert!(block.will_collide(&Vec::new(), 1.0));
}

#[test]
fn block_validate_cannot_move_horizontally_another_square() {
    let mut block = Block::new(BlockType::Z, Configuration::new());
    block.translate(1.0, 15.0);

    let squares = vec!(Square::new(15.0, 0.0, graphics::BLACK));

    assert!(block.will_collide(&squares, -1.0));
}

#[test]
fn block_validate_rotation_z() {
    let mut block = Block::new(BlockType::Z, Configuration::new());
    block.rotate();

    assert!(block.positions.iter().any(|p| *p == (0.0, 2.0)));
    assert!(block.positions.iter().any(|p| *p == (1.0, 1.0)));
    assert!(block.positions.iter().any(|p| *p == (2.0, 1.0)));
    assert!(block.positions.iter().any(|p| *p == (1.0, 2.0)));
}

#[test]
fn block_validate_rotation_j() {
    let mut block = Block::new(BlockType::J, Configuration::new());
    block.rotate();

    assert!(block.positions.iter().any(|p| *p == (0.0, 0.0)));
    assert!(block.positions.iter().any(|p| *p == (1.0, 0.0)));
    assert!(block.positions.iter().any(|p| *p == (1.0, 1.0)));
    assert!(block.positions.iter().any(|p| *p == (1.0, 2.0)));
}