use std::rand::{Rng, Rand};

#[derive(Clone, Rand, Debug)]
pub enum CellProperty {
    Floor,
    Wall,
    Entrance,
    Exit,
    Door
}

#[derive(Clone, Rand, Debug)]
pub enum CellOccupant
{
    Monster,
    Treasure,
    Trap,
    Teleporter
}

#[derive(Clone)]
pub struct Cell {
    x: u32,
    y: u32,
    property: Option<CellProperty>,
    occupants: Vec<CellOccupant>,
}

impl Cell {
    pub fn new(x: u32, y: u32, property: Option<CellProperty>) -> Cell {
        Cell {
            x: x,
            y: y,
            property: property,
            occupants: vec![]
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
            property: Some(rng.gen::<CellProperty>()),
            occupants: vec![]
        }
    }
}
