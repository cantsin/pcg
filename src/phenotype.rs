use dungeon::{Dungeon};
use statistics::{Statistic};
use celloption::{CellOptions, CellOption, Tile, Item, Occupant};
use util::{odds};

use rand::{Rng};

/// Seed holds all the information necessary to generate the phenotype.
#[derive(Clone, Debug)]
pub struct Seed {
    pub width: u32,
    pub height: u32,
    pub tiles: CellOptions<Tile>,
    pub items: CellOptions<Item>,
    pub occupants: CellOptions<Occupant>,
    pub occupant_chance: f64,
}

impl Seed {
    pub fn new(width: u32,
               height: u32,
               tiles: CellOptions<Tile>,
               items: CellOptions<Item>,
               occupants: CellOptions<Occupant>,
               occupant_chance: f64) -> Seed {
        Seed {
            width: width,
            height: height,
            tiles: tiles,
            items: items,
            occupants: occupants,
            occupant_chance: occupant_chance
        }
    }

    // generate a random occupant on this tile if it is empty.
    pub fn random_occupant<R: Rng>(&self, rng: &mut R, tile: &Tile) -> Option<Occupant> {
        let percentage = (self.occupant_chance * 100.0) as u64;
        if tile.name() == "floor" && odds(rng, percentage, 100) {
            Some(self.occupants.choose(rng).clone())
        } else {
            None
        }
    }

    // generate a range of random occupants on random coordinates.
    pub fn random_occupants<R: Rng>(&self, rng: &mut R) -> Vec<(Occupant, (u32, u32))>{
        let n = (self.width * self.height) as f64;
        let percentage = (self.occupant_chance * n) as u64;
        range(0, percentage).map(|_| {
            let x = rng.gen_range(1, self.width);
            let y = rng.gen_range(1, self.height);
            let o = self.occupants.choose(rng).clone();
            (o, (x, y))
        }).collect()
    }
}

#[derive(Clone, Debug)]
pub struct Phenotype {
    dungeon: Dungeon,
    statistics: Statistic
}
