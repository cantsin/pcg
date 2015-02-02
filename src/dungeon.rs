use std::vec::{Vec};
use std::iter::{range};

use cell::Cell;

#[derive(Clone)]
pub struct Dungeon {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>
}

impl Dungeon {
    pub fn new(width: usize, height: usize) -> Dungeon {
        let cells = range(0, width).map(|i| {
            range(0, height).map(|j| {
                Cell::new(i as u32, j as u32, None)
            }).collect()
        }).collect();
        Dungeon {
            width: width,
            height: height,
            cells: cells
        }
    }
}
