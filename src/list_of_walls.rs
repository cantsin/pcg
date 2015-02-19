use dungeon::{Dungeon};
use celloption::{CellOptions, Tile, Item, Occupant};
use genotype::{GenoType};

use rand::{Rng, ThreadRng, thread_rng};

#[derive(Clone, Debug)]
pub struct ListOfWalls {
    fitness: f64,
    dungeon: Dungeon,
    walls: Vec<Wall>,
    tiles: CellOptions<Tile>,
    items: CellOptions<Item>,
    occupants: CellOptions<Occupant>
}

#[derive(Clone, Debug)]
struct Wall {
    x: usize,
    y: usize,
    length: usize,
    xstep: i32,
    ystep: i32
}

impl ListOfWalls {
    pub fn new(width: usize,
               height: usize,
               tiles: CellOptions<Tile>,
               items: CellOptions<Item>,
               occupants: CellOptions<Occupant>) -> ListOfWalls {
        let dungeon = Dungeon::new(width, height);
        let n = width * height / 10;
        let mut rng = thread_rng();
        let walls: Vec<Wall> = range(0, n).map(|_| ListOfWalls::random_wall(&mut rng, width, height)).collect();
        ListOfWalls {
            fitness: 0.0,
            dungeon: dungeon,
            tiles: tiles,
            items: items,
            occupants: occupants,
            walls: walls
        }
    }

    pub fn random_wall(rng: &mut ThreadRng, width: usize, height: usize) -> Wall {
        let x: usize = rng.gen_range(1, width);
        let y: usize = rng.gen_range(1, height);
        let length: usize = rng.gen_range(1, width);
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

impl GenoType for ListOfWalls {
    fn mutate(&mut self, rng: &mut ThreadRng) {
        // change 33% of walls.
        let length = self.walls.len();
        let n = (length as f32 * 0.33) as usize;
        for _ in range(0, n) {
            let index = rng.gen_range(1, length);
            self.walls[index] = ListOfWalls::random_wall(rng, self.dungeon.width, self.dungeon.height);
        }
    }

    fn generate(&mut self, rng: &mut ThreadRng) -> Dungeon {
        let w = self.dungeon.width as i32;
        let h = self.dungeon.height as i32;
        let floor = self.tiles.get("floor").unwrap();
        for i in 0..self.dungeon.width {
            for j in 0..self.dungeon.height {
                self.dungeon.cells[i][j].tile = Some(floor.clone());
            }
        }
        let wall_tile = self.tiles.get("wall").unwrap();
        for wall in self.walls.iter() {
            let mut x = wall.x as i32;
            let mut y = wall.y as i32;
            for _ in range(0, wall.length) {
                x += wall.xstep;
                y += wall.ystep;
                if x < 0 || x >= w || y < 0 || y >= h {
                    break
                }
                self.dungeon.cells[x as usize][y as usize].tile = Some(wall_tile.clone());
                // small chance of a door?
            }
        }
        // randomly place entrance
        let entrance = self.tiles.get("entrance").unwrap();
        let entrance_x = rng.gen_range(1, self.dungeon.width);
        let entrance_y = rng.gen_range(1, self.dungeon.height);
        self.dungeon.cells[entrance_x][entrance_y].tile = Some(entrance.clone());

        // randomly place exit
        let exit = self.tiles.get("exit").unwrap();
        let exit_x = rng.gen_range(1, self.dungeon.width);
        let exit_y = rng.gen_range(1, self.dungeon.height);
        self.dungeon.cells[exit_x][exit_y].tile = Some(exit.clone());

        self.dungeon.clone()
    }

    fn last(&self) -> Dungeon {
        self.dungeon.clone()
    }
}
