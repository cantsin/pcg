use dungeon::{Dungeon};
use celloption::{CellOption};
use genotype::{Genotype};
use phenotype::{Seed};

use rand::{Rng, thread_rng};
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
    fn generate(&self) -> Dungeon {
        let mut rng = thread_rng();
        let w = self.seed.width;
        let h = self.seed.height;
        let mut dungeon = Dungeon::new(w, h, None);
        for i in 0..w {
            for j in 0..h {
                let tile = self.seed.tiles.choose(&mut rng).clone();
                dungeon.cells[i][j].tile = Some(tile);
            }
        }
        dungeon.clone()
    }
}
