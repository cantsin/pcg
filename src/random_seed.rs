use dungeon::{Dungeon};
use celloption::{CellOption};
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
        let w = self.seed.width;
        let h = self.seed.height;
        let mut dungeon = Dungeon::new(w, h, None);
        for i in 0..w {
            for j in 0..h {
                let tile = self.seed.tiles.choose(rng).clone();
                dungeon.cells[i][j].tile = Some(tile);
            }
        }
        dungeon.clone()
    }
}
