use dungeon::{Dungeon};
use celloption::{CellOption};
use genotype::{Genotype};
use phenotype::{Seed};

use rand::{Rng};

#[derive(Clone, Debug)]
pub struct ListOfWalls {
    seed: Seed,
    walls: Vec<Wall>,
}

#[derive(Clone, Debug)]
struct Wall {
    x: usize,
    y: usize,
    length: usize,
    xstep: i32,
    ystep: i32
}

impl Wall {
    pub fn random<T: Rng>(rng: &mut T, width: usize, height: usize) -> Wall {
        let x: usize = rng.gen_range(1, width);
        let y: usize = rng.gen_range(1, height);
        let length: usize = rng.gen_range(1, width * height);
        let xstep: i32 = rng.gen_range(-1, 2);
        let ystep: i32 = rng.gen_range(-1, 2);
        Wall {
            x: x,
            y: y,
            length: length,
            xstep: xstep,
            ystep: ystep
        }
    }
}

impl ListOfWalls {
    pub fn new(seed: &Seed) -> ListOfWalls {
        ListOfWalls {
            seed: seed.clone(),
            walls: vec![],
        }
    }
}

impl Genotype for ListOfWalls {
    fn initialize<T: Rng>(&self, rng: &mut T) -> ListOfWalls {
        let n = self.seed.width * self.seed.height / 10;
        let walls = range(0, n).map(|_| {
            Wall::random(rng, self.seed.width, self.seed.height)
        }).collect();
        ListOfWalls {
            seed: self.seed.clone(),
            walls: walls,
        }
    }

    fn mutate<T: Rng>(&mut self, rng: &mut T) {
        // change 33% of walls.
        let length = self.walls.len();
        let n = (length as f32 * 0.33) as usize;
        for _ in range(0, n) {
            let index = rng.gen_range(1, length);
            self.walls[index] = Wall::random(rng, self.seed.width, self.seed.height);
        }
    }

    fn generate<T: Rng>(&self, rng: &mut T) -> Dungeon {
        let mut dungeon = Dungeon::new(self.seed.width, self.seed.height);
        let w = dungeon.width;
        let h = dungeon.height;
        let floor = self.seed.tiles.get("floor").unwrap();
        for i in 0..dungeon.width {
            for j in 0..dungeon.height {
                dungeon.cells[i][j].tile = Some(floor.clone());
            }
        }
        let wall_tile = self.seed.tiles.get("wall").unwrap();
        let door_tile = self.seed.tiles.get("door").unwrap();
        for wall in self.walls.iter() {
            let mut x = wall.x as i32;
            let mut y = wall.y as i32;
            for _ in range(0, wall.length) {
                x += wall.xstep;
                y += wall.ystep;
                if x < 0 || x >= w as i32 || y < 0 || y >= h as i32 {
                    break
                }
                // small chance for a door
                if rng.gen_range(0, wall.length * 5) == 0 {
                    dungeon.cells[x as usize][y as usize].tile = Some(door_tile.clone());
                }
                else {
                    dungeon.cells[x as usize][y as usize].tile = Some(wall_tile.clone());
                }
            }
        }
        // TODO check for collisions

        // occupants have 0.05% chance to generate
        let n = w * h;
        let occupants = self.seed.occupants.sample(rng, (n as f64 * 0.5) as usize);
        for occupant in occupants {
            let x = rng.gen_range(1, w);
            let y = rng.gen_range(1, h);
            let tile = dungeon.cells[x][y].tile.clone();
            match tile {
                Some(ref t) if t.name() == "floor" => {
                    dungeon.cells[x][y].occupant = Some(occupant.clone());
                }
                _ => ()
            }
        }

        // randomly place entrance
        let entrance = self.seed.tiles.get("entrance").unwrap();
        let entrance_x = rng.gen_range(1, w);
        let entrance_y = rng.gen_range(1, h);
        dungeon.cells[entrance_x][entrance_y].tile = Some(entrance.clone());

        // randomly place exit
        let exit = self.seed.tiles.get("exit").unwrap();
        let exit_x = rng.gen_range(1, w);
        let exit_y = rng.gen_range(1, h);
        dungeon.cells[exit_x][exit_y].tile = Some(exit.clone());

        dungeon.clone()
    }
}
