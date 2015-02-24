use dungeon::{Dungeon};
use celloption::{CellOptions, CellOption, Tile, Item, Occupant};
use genotype::{Genotype};
use statistics::{Statistic, Statistics};
use config::{Config};

use std::iter::{repeat};
use std::collections::{HashMap};
use rand::{Rng, ThreadRng};
use util::{odds};

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
    statistic: Statistic
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
            statistic: Statistic::new()
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

impl Genotype for WallPatterns {
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
        // draw the patterns according to the indices we have.
        let n = self.patterns.len();
        for i in 0..self.dungeon.width {
            let x: usize = i / self.pattern_width;
            let inner_x = i % self.pattern_width;
            for j in 0..self.dungeon.height {
                let y: usize = j / self.pattern_height;
                let index = self.indices[(y * self.pattern_width + x) % n];
                let ref pattern = self.patterns[index];
                // here, we have to invert due to a mismatch between how we draw the patterns and opengl coords
                let inner_y = self.pattern_height - (j % self.pattern_height) - 1;
                let tile = pattern.pattern[inner_y * self.pattern_width + inner_x].clone();
                self.dungeon.cells[i][j].tile = tile.clone();
                // randomly add an occupant
                match tile {
                    Some(ref t) if t.name() == "floor" => {
                        // occupants have 0.05% chance to generate
                        if odds(rng, 5, 100) {
                            let occupant = self.occupants.choose(rng);
                            self.dungeon.cells[i][j].occupant = Some(occupant.clone());
                        }
                    }
                    _ => ()
                }
            }
        }
        self.dungeon.clone()
    }

    fn last(&self) -> Dungeon {
        self.dungeon.clone()
    }
}

impl Statistics for WallPatterns {
    fn set_iteration(&mut self, v: u32) {
        self.statistic.iteration = v;
    }

    fn get_iteration(&self) -> u32 {
        self.statistic.iteration
    }

    fn set_ranking(&mut self, v: f64) {
        self.statistic.ranking = v;
    }

    fn get_ranking(&self) -> f64 {
        self.statistic.ranking
    }
}
