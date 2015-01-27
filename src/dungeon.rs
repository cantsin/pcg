use std::vec::{Vec};
use std::iter::{repeat};

use cell::Cell;

pub struct Dungeon {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>
}

impl Dungeon {
    pub fn new(width: usize, height: usize) -> Dungeon {
        let default = Cell::new(0, 0, None);
        Dungeon {
            width: width,
            height: height,
            cells: repeat(repeat(default).take(width).collect()).take(height).collect()
        }
    }
}
