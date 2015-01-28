use std::rand::{Rng, Rand};

/// TOML
enum CellType {
    CellTile(String),
    CellItem(String),
    CellOccupant(String),
    CellUnknown(String)
}

impl CellType {
    // parse?
}

type CellTypes = Vec<CellType>;

#[derive(Clone)]
enum CellTile {
    Tile(String)
}

#[derive(Clone)]
enum CellItem {
    Item(String)
}

#[derive(Clone)]
enum CellOccupant {
    Occupant(String)
}

#[derive(Clone)]
pub struct Cell {
    x: u32,
    y: u32,
    tile: Option<CellTile>,
    occupants: Vec<CellOccupant>,
    items: Vec<CellItem>
}

impl Cell {
    pub fn new(x: u32, y: u32, tile: Option<CellTile>) -> Cell {
        Cell {
            x: x,
            y: y,
            tile: tile,
            occupants: vec![],
            items: vec![],
        }
    }

    pub fn add(&mut self, occupant: CellOccupant) -> () {
        self.occupants.push(occupant)
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Rand for Cell {
    fn rand<R: Rng>(rng: &mut R) -> Cell {
        Cell {
            x: 0,
            y: 0,
            tile: None,
            occupants: vec![],
            items: vec![]
        }
    }
}
