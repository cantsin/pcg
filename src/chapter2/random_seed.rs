use chapter2::dungeon::{Dungeon};
use chapter2::genotype::{Genotype};
use chapter2::phenotype::{Seed};

use rand::{Rng, thread_rng};

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
        for i in 0..dungeon.width as u32 {
            for j in 0..dungeon.height as u32 {
                let tile = self.seed.tiles.choose(&mut rng).clone();
                if let Some(occupant) = self.seed.random_occupant(&mut rng, &tile) {
                    dungeon.set_occupant(i, j, &occupant);
                }
                dungeon.set_tile(i, j, &tile);
            }
        }
        dungeon.clone()
    }
}
