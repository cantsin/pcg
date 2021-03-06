use std::vec::{Vec};
use std::iter::{Iterator};

use chapter2::cell::{Cell};
use chapter2::celloption::{Tile, Occupant};

#[derive(Clone, Debug)]
pub struct Dungeon {
    pub width: usize,
    pub height: usize,
    cells: Vec<Vec<Cell>>
}

impl Dungeon {
    pub fn new(width: u32, height: u32, tile: Option<Tile>) -> Dungeon {
        let cells = (0..width).map(|i| {
            (0..height).map(|j| {
                Cell::new(i, j, tile.clone())
            }).collect()
        }).collect();
        Dungeon {
            width: width as usize,
            height: height as usize,
            cells: cells
        }
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    // these methods proxy to self.cells.
    pub fn set_tile(&mut self, x: u32, y: u32, tile: &Tile) {
        self.cells[x as usize][y as usize].tile = Some(tile.clone());
    }

    pub fn set_occupant(&mut self, x: u32, y: u32, occupant: &Occupant) {
        self.cells[x as usize][y as usize].occupant = Some(occupant.clone());
    }

    pub fn has_attribute(&self, x: u32, y: u32, attribute: &str) -> bool {
        self.cells[x as usize][y as usize].has_attribute(attribute)
    }

    pub fn is_empty(&self, x: u32, y: u32) -> bool {
        self.cells[x as usize][y as usize].is_empty()
    }
}

// external iterator.
pub struct DungeonCells {
    dungeon: Dungeon,
    coords: (u32, u32)
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
        let column = self.dungeon.cells.get(x as usize);
        match column {
            Some(col) => {
                let row = col.get(y as usize);
                match row {
                    Some(cell) => {
                        let new_x = (x + 1) % self.dungeon.width as u32;
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

pub struct SurroundingCells {
    dungeon: Dungeon,
    coords: [(i32, i32); 8],
    index: usize
}

pub enum Surrounding {
    Cardinal,
    AllDirections
}

impl SurroundingCells {
    pub fn new(dungeon: &Dungeon, x: u32, y: u32, around: Surrounding) -> SurroundingCells {
        let ref cell = dungeon.cells[x as usize][y as usize];
        let x = cell.x as i32;
        let y = cell.y as i32;
        // clockwise, starting from the top
        let coords = match around {
            Surrounding::Cardinal => {
                let invalid = (-1, -1);
                [(x  , y-1),
                 (x+1, y  ),
                 (x  , y+1),
                 (x-1, y  ),
                 invalid,
                 invalid,
                 invalid,
                 invalid]
            }
            Surrounding::AllDirections => {
                [(x  , y-1),
                 (x+1, y-1),
                 (x+1, y  ),
                 (x+1, y+1),
                 (x  , y+1),
                 (x-1, y+1),
                 (x-1, y  ),
                 (x-1, y-1)]
            }
        };
        SurroundingCells {
            dungeon: dungeon.clone(),
            coords: coords,
            index: 0
        }
    }
}

impl Iterator for SurroundingCells {
    type Item = Cell;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.index < self.coords.len() {
            let mut coord = self.coords[self.index];
            // look for the first valid coordinate.
            while !self.dungeon.in_bounds(coord.0, coord.1) {
                self.index += 1;
                if self.index >= self.coords.len() {
                    break;
                }
                coord = self.coords[self.index];
            }
            if self.index < self.coords.len() {
                let cell = self.dungeon.cells[coord.0 as usize][coord.1 as usize].clone();
                self.index += 1;
                Some(cell)
            } else {
                None
            }
        }
        else {
            None
        }
    }
}
