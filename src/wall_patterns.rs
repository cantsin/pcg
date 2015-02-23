use dungeon::{Dungeon};
use celloption::{CellOptions, Tile, Item, Occupant};
use genotype::{GenoType};
use config::{Config};

use std::iter::{repeat};
use std::collections::{HashMap};
use rand::{Rng, ThreadRng, thread_rng};

#[derive(Clone, Debug)]
pub struct WallPatterns {
    fitness: f64,
    dungeon: Dungeon,
    patterns: Vec<WallPattern>,
    pattern_width: usize,
    pattern_height: usize,
    indices: Vec<usize>,
    items: CellOptions<Item>,
    occupants: CellOptions<Occupant>,
    // statistics
    pub iteration: u32,
    pub ranking: f64
}

#[derive(Clone, Debug)]
struct WallPattern {
    pattern: Vec<Option<Tile>>
}

impl WallPatterns {
    pub fn new(config: &Config,
               width: usize,
               height: usize,
               tiles: CellOptions<Tile>,
               items: CellOptions<Item>,
               occupants: CellOptions<Occupant>) -> WallPatterns {
        // read in the configurations
        let wallpatterns = config.get_table(None, "wallpatterns");
        let tile_vars = config.get_table(Some(wallpatterns), "tiles");
        let pattern_width = config.get_integer(tile_vars, "width") as usize;
        let pattern_height = config.get_integer(tile_vars, "height") as usize;
        let mut mapping = HashMap::new();
        // TODO: function to iterate over toml section settings
        mapping.insert(config.get_char(tile_vars, "floor"), tiles.get("floor").unwrap());
        mapping.insert(config.get_char(tile_vars, "wall"), tiles.get("wall").unwrap());
        mapping.insert(config.get_char(tile_vars, "entrance"), tiles.get("entrance").unwrap());
        mapping.insert(config.get_char(tile_vars, "exit"), tiles.get("exit").unwrap());
        mapping.insert(config.get_char(tile_vars, "door"), tiles.get("door").unwrap());
        let room_vars = config.get_table(Some(wallpatterns), "rooms");
        let rooms = vec!["empty","altar","down_","up___","rand1","rand2","rand3","rand4"];
        let patterns: Vec<WallPattern> = rooms.iter().map(|&r| {
            let description = config.get_array(room_vars, r);
            WallPatterns::construct(&mapping, description, pattern_width, pattern_height)
        }).collect();
        let indices: Vec<usize> = repeat(0).take(patterns.len()).collect();
        let dungeon = Dungeon::new(width, height);
        WallPatterns {
            fitness: 0.0,
            dungeon: dungeon,
            patterns: patterns,
            pattern_width: pattern_width,
            pattern_height: pattern_height,
            indices: indices,
            items: items,
            occupants: occupants,
            iteration: 0,
            ranking: -1.0
        }
    }

    fn construct(mapping: &HashMap<char, &Tile>, description: Vec<String>, width: usize, height: usize) -> WallPattern {
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
        WallPattern {
            pattern: pattern
        }
    }
}

impl GenoType for WallPatterns {
    fn mutate(&mut self, rng: &mut ThreadRng) {
        // change up to 33% of indices.
        let length = self.patterns.len();
        let n = (length as f32 * 0.33) as usize;
        for _ in range(0, n) {
            let index = rng.gen_range(1, length);
            self.indices[index] = index;
        }
    }

    fn generate(&mut self, rng: &mut ThreadRng) -> Dungeon {
        let w = self.dungeon.width as i32;
        let h = self.dungeon.height as i32;

        self.dungeon.clone()
    }

    fn statistics(&mut self, stats: &HashMap<String, f64>) {
        match stats.get("iteration") {
            None => (),
            Some(&val) => self.iteration = val as u32
        }
        match stats.get("ranking") {
            None => (),
            Some(&val) => self.ranking = val
        }
    }

    fn last(&self) -> Dungeon {
        self.dungeon.clone()
    }
}
