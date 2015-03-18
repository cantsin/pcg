use dungeon::{Dungeon};
use celloption::{Tile, Occupant};
use genotype::{Genotype};
use phenotype::{Seed};
use config::{Config};

use std::collections::{HashMap};
use rand::{Rng};

#[derive(Clone, Debug)]
pub struct WallPatterns {
    seed: Seed,
    patterns: Vec<Pattern>,
    pattern_width: u32,
    pattern_height: u32,
    indices: Vec<usize>,
    occupants: Vec<(Occupant, (u32, u32))>,
}

#[derive(Clone, Debug)]
struct Pattern {
    pattern: Vec<Option<Tile>>
}

impl Pattern {
    fn from_config(mapping: &HashMap<char, &Tile>, description: Vec<String>, width: u32, height: u32) -> Pattern {
        let size = width as usize * height as usize;
        let mut pattern = Vec::with_capacity(size);
        for line in description {
            for ch in line.chars() {
                let tile = mapping.get(&(ch));
                match tile {
                    None => pattern.push(None),
                    Some(&tile) => pattern.push(Some((*tile).clone()))
                }
            }
        }
        assert!(pattern.len() == size); // sanity check
        Pattern {
            pattern: pattern
        }
    }
}

impl WallPatterns {
    pub fn new(config: &Config, seed: &Seed) -> WallPatterns {
        // read in the configurations
        let wallpatterns = config.get_table(None, "wallpatterns");
        let tile_vars = config.get_table(Some(wallpatterns), "tiles");
        let pattern_width = config.get_integer(tile_vars, "width") as u32;
        let pattern_height = config.get_integer(tile_vars, "height") as u32;
        let mut mapping = HashMap::new();
        let tiles = config.get_listing(tile_vars, vec!["width", "height"]);
        for tile in tiles {
            let name = tile.as_slice();
            let graphical_tile = seed.tiles.get(name).unwrap();
            mapping.insert(config.get_char(tile_vars, name), graphical_tile);
        }
        let room_vars = config.get_table(Some(wallpatterns), "rooms");
        let rooms = config.get_listing(room_vars, vec![]);
        let patterns: Vec<Pattern> = rooms.iter().map(|r| {
            let description = config.get_array(room_vars, r);
            Pattern::from_config(&mapping, description, pattern_width, pattern_height)
        }).collect();
        WallPatterns {
            seed: seed.clone(),
            patterns: patterns,
            pattern_width: pattern_width,
            pattern_height: pattern_height,
            indices: vec![],
            occupants: vec![],
        }
    }

}

impl Genotype for WallPatterns {
    fn initialize<T: Rng>(&self, rng: &mut T) -> WallPatterns {
        let n = self.patterns.len();
        assert!(n != 0);
        let indices = rng.gen_iter::<usize>().take(n).map(|v| v % n).collect();
        let occupants = self.seed.random_occupants(rng);
        WallPatterns {
            seed: self.seed.clone(),
            patterns: self.patterns.clone(),
            pattern_width: self.pattern_width,
            pattern_height: self.pattern_height,
            indices: indices,
            occupants: occupants,
        }
    }

    fn mutate<T: Rng>(&mut self, rng: &mut T, percentage: f64) {
        let length = self.patterns.len();
        let n = (length as f64 * percentage) as u32;
        for _ in range(0, n) {
            let index = rng.gen_range(1, length);
            self.indices[index] = index;
        }
        self.occupants = self.seed.random_occupants(rng);
    }

    fn generate(&self) -> Dungeon {
        // draw the patterns according to the indices we have.
        let w = self.seed.width;
        let h = self.seed.height;
        let n = self.patterns.len();
        let mut dungeon = Dungeon::new(w, h, None);
        for i in 0..w {
            let x: u32 = i / self.pattern_width;
            let inner_x = i % self.pattern_width;
            for j in 0..h {
                let y: u32 = j / self.pattern_height;
                let index = self.indices[(y * self.pattern_width + x) as usize % n];
                let ref pattern = self.patterns[index];
                // here, we have to invert due to a mismatch between how we draw the patterns and opengl coords
                let inner_y = self.pattern_height - (j % self.pattern_height) - 1;
                let tile_index = inner_y * self.pattern_width + inner_x;
                if let Some(tile) = pattern.pattern[tile_index as usize].clone() {
                    dungeon.set_tile(i, j, &tile);
                }
            }
        }
        // draw the occupants if their tile is not otherwise occupied.
        for (occupant, coord) in self.occupants.clone() {
            let x = coord.0;
            let y = coord.1;
            if dungeon.has_attribute(x, y, "floor") {
                dungeon.set_occupant(x, y, &occupant);
            }
        }
        dungeon.clone()
    }
}
