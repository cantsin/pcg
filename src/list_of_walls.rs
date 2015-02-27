use dungeon::{Dungeon};
use genotype::{Genotype};
use phenotype::{Seed};
use config::{Config};
use util::{odds};

use std::num::{Float};
use rand::{Rng};

#[derive(Clone, Debug)]
pub struct ListOfWalls {
    seed: Seed,
    walls: Vec<Wall>,
    coverage: f64,
    door_chance: f64,
    entrance: (u32, u32),
    exit: (u32, u32),
}

#[derive(Clone, Debug)]
struct Wall {
    x: u32,
    y: u32,
    length: usize,
    xstep: i32,
    ystep: i32,
    door: Option<(i32, i32)> // can be out of bounds
}

impl Wall {
    pub fn random<T: Rng>(rng: &mut T, width: u32, height: u32, door_chance: u64) -> Wall {
        let x: u32 = rng.gen_range(1, width);
        let y: u32 = rng.gen_range(1, height);
        let n = ((width * width + height * height) as f64).sqrt();
        let length: usize = rng.gen_range(2, n as usize);
        let xstep: i32 = rng.gen_range(-1, 2);
        let ystep: i32 = rng.gen_range(-1, 2);
        // randomly assign a door.
        let has_door = odds(rng, door_chance, 100);
        let door = if has_door {
            let distance: i32 = rng.gen_range(1, length as i32);
            let door_x = x as i32 + (xstep * distance);
            let door_y = y as i32 + (ystep * distance);
            Some((door_x, door_y))
        } else {
            None
        };
        Wall {
            x: x,
            y: y,
            length: length,
            xstep: xstep,
            ystep: ystep,
            door: door
        }
    }
}

impl ListOfWalls {
    pub fn new(config: &Config, seed: &Seed) -> ListOfWalls {
        let wall_vars = config.get_table(None, "list-of-walls");
        let door_chance = config.get_float(wall_vars, "door_chance");
        let coverage = config.get_float(wall_vars, "coverage");
        ListOfWalls {
            seed: seed.clone(),
            door_chance: door_chance,
            coverage: coverage,
            walls: vec![],
            entrance: (0, 0),
            exit: (0, 0),
        }
    }
}

impl Genotype for ListOfWalls {
    fn initialize<T: Rng>(&self, rng: &mut T) -> ListOfWalls {
        let w = self.seed.width;
        let h = self.seed.height;
        let door_chance = (self.door_chance * 100.0) as u64;
        let percentage = (100.0 - self.coverage * 100.0) as u32;
        let n = w * h / percentage;
        let walls = range(0, n).map(|_| {
            Wall::random(rng, w, h, door_chance)
        }).collect();
        // don't worry about collisions, just plop them down somewhere.
        let entrance = (rng.gen_range(1, w), rng.gen_range(1, h));
        let exit = (rng.gen_range(1, w), rng.gen_range(1, h));
        ListOfWalls {
            seed: self.seed.clone(),
            door_chance: self.door_chance,
            coverage: self.coverage,
            walls: walls,
            entrance: entrance,
            exit: exit,
        }
    }

    fn mutate<T: Rng>(&mut self, rng: &mut T, percentage: f64) {
        let w = self.seed.width;
        let h = self.seed.height;
        let length = self.walls.len();
        let n = (length as f64 * percentage) as u32;
        for _ in range(0, n) {
            let index = rng.gen_range(1, length);
            let door_chance = (self.door_chance * 100.0) as u64;
            let wall = Wall::random(rng, w, h, door_chance);
            self.walls[index] = wall;
        }
    }

    fn generate(&self) -> Dungeon {
        let w = self.seed.width;
        let h = self.seed.height;
        let floor = self.seed.tiles.get("floor").unwrap();
        let mut dungeon = Dungeon::new(w, h, Some(floor.clone()));
        let wall_tile = self.seed.tiles.get("wall").unwrap();
        let door_tile = self.seed.tiles.get("door").unwrap();
        for wall in self.walls.iter() {
            let mut x = wall.x as i32;
            let mut y = wall.y as i32;
            for _ in range(0, wall.length) {
                x += wall.xstep;
                y += wall.ystep;
                if dungeon.in_bounds(x, y) {
                    match wall.door {
                        Some((dx, dy)) if dx == x && dy == y =>
                            dungeon.cells[x as usize][y as usize].tile = Some(door_tile.clone()),
                        _ =>
                            dungeon.cells[x as usize][y as usize].tile = Some(wall_tile.clone())
                    }
                }
            }
        }
        let entrance = self.seed.tiles.get("entrance").unwrap();
        let (x, y) = self.entrance;
        dungeon.cells[x as usize][y as usize].tile = Some(entrance.clone());
        let exit = self.seed.tiles.get("exit").unwrap();
        let (x, y) = self.exit;
        dungeon.cells[x as usize][y as usize].tile = Some(exit.clone());
        dungeon.clone()
    }
}
