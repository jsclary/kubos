use std::{cell::RefCell, rc::Weak};

#[derive(Copy, Clone, Debug)]
pub enum BlockFace {
    Right = 1,
    Bottom = 2,
    Front = 3,
    Back = 4,
    Top = 5,
    Left = 6,
}

#[derive(Debug)]
pub struct BlockOrientation {
    pub up: BlockFace,
    pub west: BlockFace,
    pub reflected: bool,
}

impl BlockOrientation {
    pub fn is_valid(&self) -> bool {
        ((self.up as i8) - 4i8).abs() == ((self.west as i8) - 4i8).abs()
    }
}

impl Default for BlockOrientation {
    fn default() -> BlockOrientation {
        BlockOrientation {
            up: BlockFace::Top,
            west: BlockFace::Left,
            reflected: false,
        }
    }
}

#[derive(Debug)]
pub struct Substance {}

#[derive(Debug, Default)]
pub struct Block {
    pub parent: Option<Weak<RefCell<Block>>>,
    pub children: Option<[[[Box<Block>; 2]; 2]; 2]>,
    pub substance: Option<Weak<RefCell<Substance>>>,
    pub orientation: BlockOrientation,
}

pub fn foo() {
    let cube = Block {
        ..Default::default()
    };
    println!("{cube:?} {0}", cube.orientation.is_valid());
    let faces = [
        BlockFace::Front,
        BlockFace::Back,
        BlockFace::Top,
        BlockFace::Bottom,
        BlockFace::Left,
        BlockFace::Right,
    ];
    println!("{faces:?}");
}
