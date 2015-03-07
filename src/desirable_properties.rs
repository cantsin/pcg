use dungeon::{Dungeon};
use celloption::{Occupant};
use genotype::{Genotype};
use phenotype::{Seed};
use config::{Config};
use util::{odds};

use std::iter::{range_step};
use std::collections::{HashMap, HashSet};
use rand::{Rng};

#[derive(Clone, Debug)]
pub struct DesirableProperties {
    seed: Seed,
    room_number: u32,
    room_size: u32,
    doors: u32,
    monsters: u32,
    path_length: u32,
    branching: f64,
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
    path: HashSet<(u32, u32)>
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West
}

pub type CollisionFn<'a> = Box<Fn(&(u32, u32)) -> bool + 'a>;

impl Maze {
    pub fn new<T: Rng>(rng: &mut T,
                       previous: &HashSet<(u32, u32)>,
                       collides: CollisionFn,
                       branching: f64,
                       region: u32,
                       x: u32,
                       y: u32) -> Maze {
        let path = Maze::grow(rng, previous, collides, branching, x, y);
        Maze {
            region: region,
            path: path
        }
    }

    fn grow<T: Rng>(rng: &mut T,
                    previous: &HashSet<(u32, u32)>,
                    collides: CollisionFn,
                    branching: f64,
                    x: u32,
                    y: u32) -> HashSet<(u32, u32)> {
        let branching_factor = (branching * 100.0) as u64;
        let mut path: Vec<(u32, u32)> = Vec::new();
        let mut all_paths: HashSet<(u32, u32)> = previous.clone();
        let all = vec!(Direction::North, Direction::East, Direction::South, Direction::West);
        let mut direction = rng.choose(all.as_slice()).unwrap().clone();
        let sc = vec!((Direction::North, (0, 1)),
                      (Direction::East, (1, 0)),
                      (Direction::South, (0, -1)),
                      (Direction::West, (-1, 0)));
        path.push((x, y));
        all_paths.insert((x, y));
        while !path.is_empty() {
            let (x, y) = path[path.len()-1];
            let coord_at = |(vx, vy): (i32, i32), n| ((x as i32 + vx*n) as u32, (y as i32 + vy*n) as u32);
            let uncarved: HashMap<Direction, (i32, i32)> = sc.iter().filter(|&&(_, new_dir)| {
                let coord1 = coord_at(new_dir, 1);
                let coord2 = coord_at(new_dir, 2);
                let coord3 = coord_at(new_dir, 3);
                !all_paths.contains(&coord1) &&
                    !all_paths.contains(&coord2) &&
                    !all_paths.contains(&coord3) &&
                    !collides(&coord2) &&
                    !collides(&coord3)
            }).cloned().collect();
            if !uncarved.is_empty() {
                let same_direction = odds(rng, branching_factor, 100);
                let rel = if same_direction && uncarved.contains_key(&direction) {
                    uncarved[direction]
                } else {
                    let (dir, coords) = uncarved.into_iter().next().unwrap();
                    direction = dir;
                    coords
                };
                let coord1 = coord_at(rel, 1);
                let coord2 = coord_at(rel, 2);
                all_paths.insert(coord1);
                all_paths.insert(coord2);
                path.push(coord2);
            } else {
                path.pop();
            }
        }
        all_paths.difference(previous).cloned().collect()
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
        let make_odd = |n| if n % 2 == 0 { n - 1 } else { n };
        Room {
            x: make_odd(x),
            y: make_odd(y),
            w: make_odd(w),
            h: make_odd(h),
        }
    }

    pub fn intersects(&self, rooms: &Vec<Room>) -> bool {
        for r in rooms {
            let rw = r.x + r.w;
            let rh = r.y + r.h;
            let roomw = self.x + self.w;
            let roomh = self.y + self.h;
            if roomw > r.x && roomh > r.y && self.x < rw && self.y < rh {
                return true
            }
        }
        false
    }

    pub fn contains(&self, x: u32, y: u32) -> bool {
        let roomw = self.x + self.w;
        let roomh = self.y + self.h;
        x >= self.x && x < roomw && y >= self.y && y < roomh
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
        let branching = config.get_float(desirables, "branching");
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
        let w = self.seed.width;
        let h = self.seed.height;
        let occupants = self.seed.random_occupants(rng).iter().take(self.monsters as usize).cloned().collect();
        // randomly generate rooms
        let mut rooms = vec![];
        for _ in range(0, self.room_number) {
            for _ in range(0, 10) {
                let room = Room::random(rng, w, h, self.room_size);
                if !room.intersects(&rooms) {
                    rooms.push(room);
                    break;
                }
            }
        }
        // fill in mazes
        let mut mazes = vec![];
        let mut positions: HashSet<(u32, u32)> = HashSet::new();
        let mut region = 0;
        let is_occupied = |x, y| rooms.clone().iter().fold(false, |accum, ref m| accum || m.contains(x, y));
        for x in range_step(1, w, 2) {
            for y in range_step(1, h, 2) {
                if !is_occupied(x, y) && !positions.contains(&(x, y)) {
                    let collides: CollisionFn = box |&(cx, cy)| cx >= w || cy >= h || is_occupied(cx, cy);
                    let maze = Maze::new(rng, &positions, collides, self.branching, region, x, y);
                    positions = positions.union(&maze.path).cloned().collect();
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
            rooms: rooms.clone(),
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
        let wall = self.seed.tiles.get("wall").unwrap();
        let floor = self.seed.tiles.get("floor").unwrap();
        for room in self.rooms.iter() {
            for i in range(room.x, room.x + room.w) {
                for j in range(room.y, room.y + room.h) {
                    dungeon.cells[i as usize][j as usize].tile = Some(wall.clone())
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
