use dungeon::{Dungeon};
use genotype::{Genotype};
use phenotype::{Seed};

use rand::{Rng};

#[derive(Clone, Debug)]
pub struct ListOfWalls {
    seed: Seed,
    walls: Vec<Wall>,
    entrance: (u32, u32),
    exit: (u32, u32),
}

#[derive(Clone, Debug)]
struct Wall {
    x: u32,
    y: u32,
    length: usize,
    xstep: i32,
    ystep: i32
}

impl Wall {
    pub fn random<T: Rng>(rng: &mut T, width: u32, height: u32) -> Wall {
        let x: u32 = rng.gen_range(1, width);
        let y: u32 = rng.gen_range(1, height);
        let n = width * height;
        let length: usize = rng.gen_range(1, n as usize);
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
            entrance: (0, 0),
            exit: (0, 0),
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
            entrance: (0, 0),
            exit: (0, 0),
        }
    }

    fn mutate<T: Rng>(&mut self, rng: &mut T, percentage: f64) {
        let length = self.walls.len();
        let n = (length as f64 * percentage) as u32;
        for _ in range(0, n) {
            let index = rng.gen_range(1, length);
            self.walls[index] = Wall::random(rng, self.seed.width, self.seed.height);
        }
// TODO
//        let entrance_x = rng.gen_range(1, w);
//        let entrance_y = rng.gen_range(1, h);
//        dungeon.cells[entrance_x][entrance_y].tile = Some(entrance.clone());
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
                    // TODO
                    // small chance for a door
                    // if rng.gen_range(0, wall.length * 5) == 0 {
                    //     dungeon.cells[x as usize][y as usize].tile = Some(door_tile.clone());
                    // }
                    // else {
                    dungeon.cells[x as usize][y as usize].tile = Some(wall_tile.clone());
                    // }
                }
            }
        }
        // TODO check for collisions

        // randomly place entrance
        let entrance = self.seed.tiles.get("entrance").unwrap();
        let (x, y) = self.entrance;
        dungeon.cells[x as usize][y as usize].tile = Some(entrance.clone());

        // randomly place exit
        let exit = self.seed.tiles.get("exit").unwrap();
        let (x, y) = self.exit;
        dungeon.cells[x as usize][y as usize].tile = Some(exit.clone());

        dungeon.clone()
    }
}
