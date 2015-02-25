use dungeon::{Dungeon};
use celloption::{CellOptions, CellOption, Tile, Item, Occupant};
use genotype::{Genotype};
use phenotype::{Seed};

use rand::{Rng};
use util::{odds};

#[derive(Clone, Debug)]
pub struct RandomSeed {
    seed: Seed,
}

impl RandomSeed {
    pub fn new(seed: &Seed) -> RandomSeed {
        RandomSeed {
            seed: seed.clone(),
        }
    }
}

impl Genotype for RandomSeed {
    fn generate<T: Rng>(&self, rng: &mut T) -> Dungeon {
        let dungeon = Dungeon::new(self.seed.width, self.seed.height);
        for i in 0..dungeon.width {
            for j in 0..dungeon.height {
                let tile = self.seed.tiles.choose(rng).clone();
                // occupants have 0.05% chance to generate
                if tile.name() == "floor" && odds(rng, 5, 100) {
                    let occupant = self.seed.occupants.choose(rng);
                    dungeon.cells[i][j].occupant = Some(occupant.clone());
                }
                dungeon.cells[i][j].tile = Some(tile);
            }
        }
        dungeon.clone()
    }
}
