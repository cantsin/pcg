use dungeon::{Dungeon};
use celloption::{Occupant};
use genotype::{Genotype};
use phenotype::{Seed};
use config::{Config};

use rand::{Rng};

#[derive(Clone, Debug)]
pub struct DesirableProperties {
    seed: Seed,
    rooms: u32,
    doors: u32,
    monsters: u32,
    path_length: u32,
    branching: u32,
    occupants: Vec<(Occupant, (u32, u32))>,
}

impl DesirableProperties {
    pub fn new(config: &Config, seed: &Seed) -> DesirableProperties {
        // read in the configurations
        let desirables = config.get_table(None, "desirable_patterns");
        let rooms = config.get_integer(desirables, "rooms") as u32;
        let doors = config.get_integer(desirables, "doors") as u32;
        let monsters = config.get_integer(desirables, "monsters") as u32;
        let path_length = config.get_integer(desirables, "path_length") as u32;
        let branching = config.get_integer(desirables, "branching") as u32;
        DesirableProperties {
            seed: seed.clone(),
            rooms: rooms,
            doors: doors,
            monsters: monsters,
            path_length: path_length,
            branching: branching,
            occupants: vec![],
        }
    }

}

impl Genotype for DesirableProperties {
    fn initialize<T: Rng>(&self, rng: &mut T) -> DesirableProperties {
        let occupants = self.seed.random_occupants(rng).iter().take(self.monsters as usize).cloned().collect();
        DesirableProperties {
            seed: self.seed.clone(),
            rooms: self.rooms,
            doors: self.doors,
            monsters: self.monsters,
            path_length: self.path_length,
            branching: self.branching,
            occupants: occupants,
        }
    }

    fn mutate<T: Rng>(&mut self, rng: &mut T, percentage: f64) {
    }

    fn generate(&self) -> Dungeon {
        let w = self.seed.width;
        let h = self.seed.height;
        let mut dungeon = Dungeon::new(w, h, None);
        dungeon.clone()
    }
}
