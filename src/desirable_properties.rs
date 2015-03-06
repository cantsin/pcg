use dungeon::{Dungeon};
use celloption::{Occupant};
use genotype::{Genotype};
use phenotype::{Seed};
use config::{Config};

use std::iter::{range_step};
use rand::{Rng};

#[derive(Clone, Debug)]
pub struct DesirableProperties {
    seed: Seed,
    room_number: u32,
    room_size: u32,
    doors: u32,
    monsters: u32,
    path_length: u32,
    branching: u32,
    occupants: Vec<(Occupant, (u32, u32))>,
    rooms: Vec<Room>,
    mazes: Vec<Maze>,
}

#[derive(Clone, Debug)]
struct Room {
    x: u32,
    y: u32,
    w: u32,
    h: u32
}

#[derive(Clone, Debug)]
struct Maze {
    region: u32,
    path: Vec<(u32, u32)>
}

impl Maze {
    pub fn new(region: u32, x: u32, y: u32) -> Maze {
        let path = vec!((x, y));
        Maze {
            region: region,
            path: path
        }
    }
}

impl Room {
    pub fn random<T: Rng>(rng: &mut T, width: u32, height: u32, room_size: u32) -> Room {
        let w: u32 = rng.gen_range(3, room_size);
        let h: u32 = rng.gen_range(3, room_size);
        assert!(width >= w);
        assert!(height >= h);
        let x: u32 = rng.gen_range(1, width - w);
        let y: u32 = rng.gen_range(1, height - h);
        Room {
            x: Room::make_odd(x),
            y: Room::make_odd(y),
            w: Room::make_odd(w),
            h: Room::make_odd(h),
        }
    }

    pub fn make_odd(n: u32) -> u32 {
        if n % 2 == 0 { n - 1 } else { n }
    }

    pub fn intersects(&self, rooms: &Vec<Room>) -> bool {
        for r in rooms {
            let rw = r.x + r.w;
            let rh = r.y + r.h;
            let roomw = self.x + self.w;
            let roomh = self.y + self.h;
            if roomw >= r.x && roomh >= r.y && self.x <= rw && self.y <= rh {
                return true
            }
        }
        false
    }

    pub fn contains(&self, x: u32, y: u32) -> bool {
        let roomw = self.x + self.w;
        let roomh = self.y + self.h;
        x >= self.x && x <= roomw && y >= self.y && y <= roomh
    }
}

impl DesirableProperties {
    pub fn new(config: &Config, seed: &Seed) -> DesirableProperties {
        // read in the configurations
        let desirables = config.get_table(None, "desirable_patterns");
        let room_number = config.get_integer(desirables, "room_number") as u32;
        let room_size = config.get_integer(desirables, "room_size") as u32;
        let doors = config.get_integer(desirables, "doors") as u32;
        let monsters = config.get_integer(desirables, "monsters") as u32;
        let path_length = config.get_integer(desirables, "path_length") as u32;
        let branching = config.get_integer(desirables, "branching") as u32;
        DesirableProperties {
            seed: seed.clone(),
            room_size: room_size,
            room_number: room_number,
            doors: doors,
            monsters: monsters,
            path_length: path_length,
            branching: branching,
            occupants: vec![],
            rooms: vec![],
            mazes: vec![]
        }
    }

}

impl Genotype for DesirableProperties {
    fn initialize<T: Rng>(&self, rng: &mut T) -> DesirableProperties {
        let occupants = self.seed.random_occupants(rng).iter().take(self.monsters as usize).cloned().collect();
        // randomly generate rooms
        let mut rooms = vec![];
        for _ in range(0, self.room_number) {
            for _ in range(0, 10) {
                let room = Room::random(rng, self.seed.width, self.seed.height, self.room_size);
                if !room.intersects(&rooms) {
                    rooms.push(room);
                    break;
                }
            }
        }
        // fill in mazes
        let mut mazes = vec![];
        let mut region = 0;
        for x in range_step(1, self.seed.width, 2) {
            for y in range_step(1, self.seed.height, 2) {
                let is_occupied = rooms.iter().fold(false, |accum, ref m| accum || m.contains(x, y));
                if !is_occupied {
                    let maze = Maze::new(region, x, y);
                    mazes.push(maze);
                    region += 1;
                }
            }
        }
        // find connectors
        // remove dead ends
        DesirableProperties {
            seed: self.seed.clone(),
            room_number: self.room_number,
            room_size: self.room_size,
            doors: self.doors,
            monsters: self.monsters,
            path_length: self.path_length,
            branching: self.branching,
            occupants: occupants,
            rooms: rooms,
            mazes: mazes,
        }
    }

    fn mutate<T: Rng>(&mut self, _: &mut T, _: f64) {
        // mutate a certain % of the rooms and start again
    }

    fn generate(&self) -> Dungeon {
        let w = self.seed.width;
        let h = self.seed.height;
        let mut dungeon = Dungeon::new(w, h, None);
        let floor = self.seed.tiles.get("floor").unwrap();
        for room in self.rooms.iter() {
            for i in range(room.x, room.x + room.w) {
                for j in range(room.y, room.y + room.h) {
                    dungeon.cells[i as usize][j as usize].tile = Some(floor.clone())
                }
            }
        }
        for maze in self.mazes.iter() {
            for path in maze.path.iter() {
                let (x, y): (u32, u32) = *path;
                dungeon.cells[x as usize][y as usize].tile = Some(floor.clone());
            }
        }
        dungeon.clone()
    }
}
