use dungeon::{Dungeon};
use celloption::{CellOptions, CellOption, Tile, Item, Occupant};
use genotype::{GenoType};

use std::collections::{HashMap};
use rand::{ThreadRng};
use util::{odds};

#[derive(Clone, Debug)]
pub struct RandomSeed {
    fitness: f64,
    dungeon: Dungeon,
    tiles: CellOptions<Tile>,
    items: CellOptions<Item>,
    occupants: CellOptions<Occupant>,
    // statistics
    pub iteration: u32,
    pub ranking: f64
}

impl RandomSeed {
    pub fn new(width: usize,
               height: usize,
               tiles: CellOptions<Tile>,
               items: CellOptions<Item>,
               occupants: CellOptions<Occupant>) -> RandomSeed {
        let dungeon = Dungeon::new(width, height);
        RandomSeed {
            fitness: 0.0,
            dungeon: dungeon,
            tiles: tiles,
            items: items,
            occupants: occupants,
            iteration: 0,
            ranking: -1.0
        }
    }
}

impl GenoType for RandomSeed {
    fn mutate(&mut self, rng: &mut ThreadRng) {
        for i in 0..self.dungeon.width {
            for j in 0..self.dungeon.height {
                let tile = self.tiles.choose(rng).clone();
                // occupants have 0.05% chance to generate
                if tile.name() == "floor" && odds(rng, 5, 100) {
                    let occupant = self.occupants.choose(rng);
                    self.dungeon.cells[i][j].occupant = Some(occupant.clone());
                }
                self.dungeon.cells[i][j].tile = Some(tile);
            }
        }
    }

    fn generate(&mut self, _: &mut ThreadRng) -> Dungeon {
        self.dungeon.clone()
    }

    fn statistics(&mut self, stats: &HashMap<String, f64>) {
        match stats.get("iteration") {
            None => (),
            Some(&val) => self.iteration = val as u32
        }
        match stats.get("ranking") {
            None => (),
            Some(&val) => self.ranking = val
        }
    }

    fn last(&self) -> Dungeon {
        self.dungeon.clone()
    }
}
