use std::vec::{Vec};
use std::iter::{Iterator, range};

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

// external iterator. is there a better way?
pub struct DungeonCells {
    dungeon: Dungeon,
    coords: (usize, usize)
}

impl DungeonCells {
    pub fn new(dungeon: &Dungeon) -> DungeonCells {
        DungeonCells {
            dungeon: dungeon.clone(),
            coords: (0, 0)
        }
    }
}

impl Iterator for DungeonCells {
    type Item = Cell;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let (x, y) = self.coords;
        let column = self.dungeon.cells.get(x);
        match column {
            Some(col) => {
                let row = col.get(y);
                match row {
                    Some(cell) => {
                        let new_x = (x + 1) % self.dungeon.width;
                        let new_y = if new_x == 0 { y + 1 } else { y };
                        self.coords = (new_x, new_y);
                        Some(cell.clone())
                    }
                    None => None
                }
            }
            None => None
        }
    }
}
