use chapter2::celloption::{CellOption, Tile, Occupant, Item};

#[derive(Clone, Debug)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub tile: Option<Tile>,
    pub occupant: Option<Occupant>,
    pub items: Vec<Item>
}

impl Cell {
    pub fn new(x: u32, y: u32, tile: Option<Tile>) -> Cell {
        Cell {
            x: x,
            y: y,
            tile: tile,
            occupant: None,
            items: vec![],
        }
    }

    pub fn has_attribute(&self, attribute: &str) -> bool {
        match self.tile {
            None => false,
            Some(ref t) => t.name() == attribute
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.tile {
            None => true,
            Some(ref t) => t.name() == "floor"
        }
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.x == other.x && self.y == other.y
    }
}
