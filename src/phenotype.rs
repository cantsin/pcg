use dungeon::{Dungeon};
use statistics::{Statistic};
use celloption::{CellOptions, Tile, Item, Occupant};

/// Seed holds all the information necessary to generate the phenotype.
#[derive(Clone, Debug)]
pub struct Seed {
    pub width: u32,
    pub height: u32,
    pub tiles: CellOptions<Tile>,
    pub items: CellOptions<Item>,
    pub occupants: CellOptions<Occupant>,
}

impl Seed {
    pub fn new(width: u32,
               height: u32,
               tiles: CellOptions<Tile>,
               items: CellOptions<Item>,
               occupants: CellOptions<Occupant>) -> Seed {
        Seed {
            width: width,
            height: height,
            tiles: tiles,
            items: items,
            occupants: occupants,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Phenotype {
    dungeon: Dungeon,
    statistics: Statistic
}
