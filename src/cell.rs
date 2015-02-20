use celloption::{CellOption, Tile, Occupant, Item};

#[derive(Clone, Debug)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub tile: Option<Tile>,
    pub occupants: Vec<Occupant>,
    pub items: Vec<Item>
}

impl Cell {
    pub fn new(x: u32, y: u32, tile: Option<Tile>) -> Cell {
        Cell {
            x: x,
            y: y,
            tile: tile,
            occupants: vec![],
            items: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.tile {
            None => true,
            Some(ref t) => t.name() == "floor"
        }
    }

    pub fn add(&mut self, occupant: &Occupant) {
        self.occupants.push(occupant.clone());
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.x == other.x && self.y == other.y
    }
}
