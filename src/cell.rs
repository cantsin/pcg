use std::rand::{Rng, sample};

#[derive(Clone)]
pub enum CellTile {
    Tile(String)
}

#[derive(Clone)]
pub enum CellItem {
    Item(String)
}

#[derive(Clone)]
pub enum CellOccupant {
    Occupant(String)
}

#[derive(Clone)]
pub struct Cell {
    x: u32,
    y: u32,
    pub tile: Option<CellTile>,
    occupants: Vec<CellOccupant>,
    items: Vec<CellItem>
}

pub struct CellTiles {
    tiles: Vec<CellTile>
}

impl CellTiles {
    pub fn new(names: &[&str]) -> CellTiles {
        CellTiles {
            tiles: names.iter().map(|&name| CellTile::Tile(String::from_str(name))).collect()
        }
    }

    pub fn random<R: Rng>(&self, rng: &mut R) -> CellTile {
        assert!(self.tiles.len() > 0, "Cannot retrieve random cell tile.");
        sample(rng, self.tiles.iter(), 1).into_iter().next().unwrap().clone()
    }
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

    pub fn add(&mut self, occupant: &CellOccupant) -> () {
        self.occupants.push(occupant.clone())
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.x == other.x && self.y == other.y
    }
}
