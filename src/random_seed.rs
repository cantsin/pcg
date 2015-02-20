use dungeon::{Dungeon};
use celloption::{CellOptions, Tile, Item, Occupant};
use genotype::{GenoType};

use std::collections::{HashMap};
use rand::{ThreadRng};

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
            ranking: 0.0
        }
    }
}

impl GenoType for RandomSeed {
    fn mutate(&mut self, rng: &mut ThreadRng) {
        for i in 0..self.dungeon.width {
            for j in 0..self.dungeon.height {
                let tile = self.tiles.choose(rng).clone();
                self.dungeon.cells[i][j].tile = Some(tile);

                // TODO: add possibility (0.05% per occupant)
                let occupants = self.occupants.sample(rng, 2);
                for occupant in occupants.iter() {
                    self.dungeon.cells[i][j].add(*occupant);
                }
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
