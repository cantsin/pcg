use dungeon::{Dungeon};
use celloption::{CellOptions, CellOption, Tile, Item, Occupant};
use genotype::{Genotype};
use phenotype::{Seed};
use config::{Config};

use std::iter::{repeat};
use std::collections::{HashMap};
use rand::{Rng, ThreadRng};
use util::{odds};

#[derive(Clone, Debug)]
pub struct WallPatterns {
    seed: Seed,
    patterns: Vec<Pattern>,
    pattern_width: usize,
    pattern_height: usize,
    indices: Vec<usize>
}

#[derive(Clone, Debug)]
struct Pattern {
    pattern: Vec<Option<Tile>>
}

impl Pattern {
    fn from_config(mapping: &HashMap<char, &Tile>, description: Vec<String>, width: usize, height: usize) -> Pattern {
        let mut pattern = Vec::with_capacity(width * height);
        for line in description {
            for ch in line.chars() {
                let tile = mapping.get(&(ch));
                match tile {
                    None => pattern.push(None),
                    Some(&tile) => pattern.push(Some((*tile).clone()))
                }
            }
        }
        assert!(pattern.len() == width * height); // sanity check
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
        let pattern_width = config.get_integer(tile_vars, "width") as usize;
        let pattern_height = config.get_integer(tile_vars, "height") as usize;
        let mut mapping = HashMap::new();
        // TODO: function to iterate over toml section settings
        mapping.insert(config.get_char(tile_vars, "floor"), seed.tiles.get("floor").unwrap());
        mapping.insert(config.get_char(tile_vars, "wall"), seed.tiles.get("wall").unwrap());
        mapping.insert(config.get_char(tile_vars, "entrance"), seed.tiles.get("entrance").unwrap());
        mapping.insert(config.get_char(tile_vars, "exit"), seed.tiles.get("exit").unwrap());
        mapping.insert(config.get_char(tile_vars, "door"), seed.tiles.get("door").unwrap());
        let room_vars = config.get_table(Some(wallpatterns), "rooms");
        let rooms = vec!["empty","altar","down_","up___","rand1","rand2","rand3","rand4"];
        let patterns: Vec<Pattern> = rooms.iter().map(|&r| {
            let description = config.get_array(room_vars, r);
            Pattern::from_config(&mapping, description, pattern_width, pattern_height)
        }).collect();
        WallPatterns {
            seed: seed.clone(),
            patterns: patterns,
            pattern_width: pattern_width,
            pattern_height: pattern_height,
            indices: vec![],
        }
    }

}

impl Genotype for WallPatterns {
    fn initialize<T: Rng>(&mut self, rng: &mut T) {
        // TODO: Fix
        self.indices = repeat(0).take(self.patterns.len()).collect();
    }

    fn mutate<T: Rng>(&mut self, rng: &mut T) {
        // change up to 33% of indices.
        let length = self.patterns.len();
        let n = (length as f32 * 0.33) as usize;
        for _ in range(0, n) {
            let index = rng.gen_range(1, length);
            self.indices[index] = index;
        }
    }

    fn generate<T: Rng>(&self, rng: &mut T) -> Dungeon {
        // draw the patterns according to the indices we have.
        let dungeon = Dungeon::new(self.seed.width, self.seed.height);
        let n = self.patterns.len();
        for i in 0..dungeon.width {
            let x: usize = i / self.pattern_width;
            let inner_x = i % self.pattern_width;
            for j in 0..dungeon.height {
                let y: usize = j / self.pattern_height;
                let index = self.indices[(y * self.pattern_width + x) % n];
                let ref pattern = self.patterns[index];
                // here, we have to invert due to a mismatch between how we draw the patterns and opengl coords
                let inner_y = self.pattern_height - (j % self.pattern_height) - 1;
                let tile = pattern.pattern[inner_y * self.pattern_width + inner_x].clone();
                dungeon.cells[i][j].tile = tile.clone();
                // randomly add an occupant
                match tile {
                    Some(ref t) if t.name() == "floor" => {
                        // occupants have 0.05% chance to generate
                        if odds(rng, 5, 100) {
                            let occupant = self.seed.occupants.choose(rng);
                            dungeon.cells[i][j].occupant = Some(occupant.clone());
                        }
                    }
                    _ => ()
                }
            }
        }
        dungeon.clone()
    }
}
