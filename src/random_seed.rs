use dungeon::{Dungeon};
use genotype::{Genotype};
use phenotype::{Seed};

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
        for i in 0..dungeon.width {
            for j in 0..dungeon.height {
                let tile = self.seed.tiles.choose(&mut rng).clone();
                let occupant = self.seed.random_occupant(&mut rng, &tile);
                dungeon.cells[i][j].occupant = occupant;
                dungeon.cells[i][j].tile = Some(tile);
            }
        }
        dungeon.clone()
    }
}
