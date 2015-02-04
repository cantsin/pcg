use dungeon::{Dungeon};
use celloption::{CellOptions, Tile, Item, Occupant};

pub trait GenoType {
    fn mutate(&mut self);
    fn generate(&self) -> Dungeon;

    fn evaluate<F: Fn(&Dungeon) -> f64>(&self, dungeon: &Dungeon, strategies: &[F]) -> f64 {
        strategies.iter().fold(1.0, |accum, f| accum * f(dungeon))
    }
}

use std::rand::{thread_rng};

struct RandomSeed {
    fitness: f64,
    width: usize,
    height: usize,
    tiles: CellOptions<Tile>,
    items: CellOptions<Item>,
    occupants: CellOptions<Occupant>
}

impl RandomSeed {
    pub fn new(width: usize,
               height: usize,
               tiles: CellOptions<Tile>,
               items: CellOptions<Item>,
               occupants: CellOptions<Occupant>) -> RandomSeed {
        RandomSeed {
            fitness: 0.0,
            width: width,
            height: height,
            tiles: tiles,
            items: items,
            occupants: occupants
        }
    }
}

impl GenoType for RandomSeed {
    fn mutate(&mut self) { /* no-op */ }

    fn generate(&self) -> Dungeon {
        let mut rng = thread_rng();
        let mut dungeon = Dungeon::new(self.width, self.height);
        for i in 0..dungeon.width {
            for j in 0..dungeon.height {
                let tile = self.tiles.choose(&mut rng).clone();
                dungeon.cells[i][j].tile = Some(tile);

                // TODO: add possibility (0.05% per occupant)
                let occupants = self.occupants.sample(&mut rng, 2);
                for occupant in occupants.iter() {
                    dungeon.cells[i][j].add(*occupant);
                }
            }
        }
        dungeon
    }
}
