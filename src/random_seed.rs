use dungeon::{Dungeon};
use celloption::{CellOptions, Tile, Item, Occupant};
use genotype::{GenoType};

use rand::{thread_rng};

#[derive(Clone, Debug)]
pub struct RandomSeed {
    fitness: f64,
    dungeon: Dungeon,
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
        let dungeon = Dungeon::new(width, height);
        RandomSeed {
            fitness: 0.0,
            dungeon: dungeon,
            tiles: tiles,
            items: items,
            occupants: occupants
        }
    }
}

impl GenoType for RandomSeed {
    fn mutate(&mut self) {
        let mut rng = thread_rng();
        for i in 0..self.dungeon.width {
            for j in 0..self.dungeon.height {
                let tile = self.tiles.choose(&mut rng).clone();
                self.dungeon.cells[i][j].tile = Some(tile);

                // TODO: add possibility (0.05% per occupant)
                let occupants = self.occupants.sample(&mut rng, 2);
                for occupant in occupants.iter() {
                    self.dungeon.cells[i][j].add(*occupant);
                }
            }
        }
    }

    fn generate(&mut self) -> Dungeon {
        self.dungeon.clone()
    }

    fn last(&self) -> Dungeon {
        self.dungeon.clone()
    }
}