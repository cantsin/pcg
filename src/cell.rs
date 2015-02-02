use celloption::{Tile, Occupant, Item};

#[derive(Clone)]
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

    pub fn add(&mut self, occupant: &Occupant) {
        self.occupants.push(occupant.clone());
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.x == other.x && self.y == other.y
    }
}
