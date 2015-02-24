use dungeon::{Dungeon};
use celloption::{CellOptions, CellOption, Tile, Item, Occupant};
use genotype::{Genotype};
use statistics::{Statistic, Statistics};

use rand::{ThreadRng};
use util::{odds};

#[derive(Clone, Debug)]
pub struct RandomSeed {
    fitness: f64,
    dungeon: Dungeon,
    tiles: CellOptions<Tile>,
    items: CellOptions<Item>,
    occupants: CellOptions<Occupant>,
    statistic: Statistic
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
            statistic: Statistic::new()
        }
    }
}

impl Genotype for RandomSeed {
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

    fn last(&self) -> Dungeon {
        self.dungeon.clone()
    }
}

impl Statistics for RandomSeed {
    fn set_iteration(&mut self, v: u32) {
        self.statistic.iteration = v;
    }

    fn get_iteration(&self) -> u32 {
        self.statistic.iteration
    }

    fn set_ranking(&mut self, v: f64) {
        self.statistic.ranking = v;
    }

    fn get_ranking(&self) -> f64 {
        self.statistic.ranking
    }
}
