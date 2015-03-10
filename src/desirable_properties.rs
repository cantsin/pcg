use dungeon::{Dungeon, SurroundingCells, Surrounding};
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
    branching: f64,
    occupants: Vec<(Occupant, (u32, u32))>,
    rooms: Vec<Room>,
    mazes: Vec<Maze>,
    connectors: Vec<Connector>,
    entrance: (u32, u32),
    exit: (u32, u32),
}

#[derive(Clone, Debug)]
struct Room {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    region: u32,
}

#[derive(Clone, Debug)]
struct Maze {
    path: HashSet<(u32, u32)>,
    region: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West
}

static CARDINALS: [(Direction, (i32, i32)); 4] = [(Direction::North, (0, 1)),
                                                  (Direction::East, (1, 0)),
                                                  (Direction::South, (0, -1)),
                                                  (Direction::West, (-1, 0))];

// helper function: given a coordinate and a relative direction, get
// the coordinate n spaces away in that direction.
fn scale(coord: (u32, u32), dir: (i32, i32), n: i32) -> (u32, u32) {
    let (x, y) = coord;
    let (dx, dy) = dir;
    (((x as i32 + dx * n) as u32), ((y as i32 + dy * n) as u32))
}

#[derive(Clone, Debug)]
struct Connector {
    location: (u32, u32),
    regions: HashSet<u32>,
}

impl Connector {
    pub fn new(location: (u32, u32), regions: &HashSet<u32>) -> Connector {
        Connector {
            location: location,
            regions: regions.clone(),
        }
    }

    pub fn find_all(mazes: &Vec<Maze>, rooms: &Vec<Room>) -> Vec<Connector> {
        // build up a hashmap of coords to region id
        let mut lookup: HashMap<(u32, u32), u32> = HashMap::new();
        for maze in mazes {
            for coord in maze.clone().path {
                lookup.insert(coord, maze.region);
            }
        }
        for room in rooms {
            for coord in room.border() {
                lookup.insert(coord, room.region);
            }
        }
        // find connectors (blank spaces between two regions)
        let mut connectors: Vec<Connector> = Vec::new();
        let mut examined: HashSet<(u32, u32)> = HashSet::new();
        for (&coord, &region) in lookup.iter() {
            examined.insert(coord);
            let possibles: Vec<(u32, u32)> = CARDINALS.iter().map(|&(_, rel_dir)| scale(coord, rel_dir, 1)).collect();
            for new_coord in possibles {
                if !examined.contains(&new_coord) || !lookup.contains_key(&new_coord) {
                    let regions: HashSet<u32> = CARDINALS.iter().map(|&(_, rel_dir)| {
                        let coord = scale(new_coord, rel_dir, 1);
                        if lookup.contains_key(&coord) {
                            lookup[coord]
                        } else {
                            region // by design, at least two directions must fail
                        }
                    }).collect();
                    if regions.len() >= 2 {
                        let connector = Connector::new(new_coord, &regions);
                        connectors.push(connector);
                    }
                    examined.insert(new_coord);
                }
            }
        }
        connectors
    }

    pub fn merge<T: Rng>(rng: &mut T, connectors: &Vec<Connector>) -> Vec<Connector> {
        // accumulate all of the regions
        let mut open: HashSet<u32> = HashSet::new();
        for connector in connectors {
            open = open.union(&connector.regions.clone()).cloned().collect();
        }
        // lookup for merged regions
        let region_number = open.len();
        let mut merged_lookup: Vec<u32> = range(0, region_number as u32).collect();
        let mut merged_connectors: Vec<Connector> = Vec::new();
        let mut current = connectors.clone();
        while open.len() > 1 {
            // pick a random connector and region
            let connectors = current.clone();
            let connector = rng.choose(connectors.as_slice()).unwrap();
            let mut regions: Vec<u32> = connector.regions.iter().map(|&r| merged_lookup[r as usize]).collect();
            let first = regions.pop().unwrap();
            let rest: HashSet<u32> = regions.clone().iter().cloned().collect();
            merged_connectors.push(connector.clone());
            for i in range(0, region_number) {
                if rest.contains(&merged_lookup[i]) {
                    merged_lookup[i] = first;
                }
            }
            // remove unnecessary connectors
            current = current.iter().filter(|&c| {
                let regions: HashSet<u32> = c.regions.iter().map(|&r| merged_lookup[r as usize]).collect();
                regions.len() > 1
            }).cloned().collect();
            open = open.difference(&rest).cloned().collect();
        }
        merged_connectors.clone()
    }
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
                    orig_x: u32,
                    orig_y: u32) -> HashSet<(u32, u32)> {
        let branching_factor = (branching * 100.0) as u64;
        let mut path: Vec<(u32, u32)> = Vec::new();
        let mut all_paths: HashSet<(u32, u32)> = previous.clone();
        let all = vec!(Direction::North, Direction::East, Direction::South, Direction::West);
        let mut direction = rng.choose(all.as_slice()).unwrap().clone();
        path.push((orig_x, orig_y));
        all_paths.insert((orig_x, orig_y));
        // "growing-tree" algorithm
        while !path.is_empty() {
            let new_coord = path[path.len()-1];
            let uncarved: HashMap<Direction, (i32, i32)> = CARDINALS.iter().filter(|&&(_, new_dir)| {
                let coord1 = scale(new_coord, new_dir, 1);
                let coord2 = scale(new_coord, new_dir, 2);
                let coord3 = scale(new_coord, new_dir, 3);
                !all_paths.contains(&coord1) &&
                    !all_paths.contains(&coord2) &&
                    !all_paths.contains(&coord3) &&
                    !collides(&coord2) &&
                    !collides(&coord3)
            }).cloned().collect();
            if !uncarved.is_empty() {
                let same_direction = odds(rng, branching_factor, 100);
                let rel_dir = if same_direction && uncarved.contains_key(&direction) {
                    uncarved[direction]
                } else {
                    let (dir, coords) = uncarved.into_iter().next().unwrap();
                    direction = dir;
                    coords
                };
                let coord1 = scale(new_coord, rel_dir, 1);
                let coord2 = scale(new_coord, rel_dir, 2);
                all_paths.insert(coord1);
                all_paths.insert(coord2);
                path.push(coord2);
            } else {
                path.pop();
            }
        }
        all_paths.difference(previous).cloned().collect()
    }

    pub fn prune(&mut self, connectors: &Vec<Connector>) {
        // given a maze, make sure there are no dead ends. we need to
        // use connectors to make sure we don't have paths to other
        // regions.
        let coords: HashSet<(u32, u32)> = connectors.iter().map(|&ref c| c.location).collect();
        let mut extraneous: HashSet<(u32, u32)> = HashSet::new();
        for &coord in self.path.iter() {
            let possibles: Vec<(u32, u32)> = CARDINALS.iter().map(|&(_, rel_dir)| scale(coord, rel_dir, 1)).collect();
            // if we only have one exit, consider this coord extraneous
            let exits = possibles.iter().fold(0, |accum, &c| {
                if coords.contains(&c) || self.path.contains(&c) {
                    accum + 1
                } else {
                    accum
                }
            });
            if exits == 1 {
                extraneous.insert(coord);
            }
        }
        // follow the extraneous ends to their inevitable conclusion
        let mut mod_path: HashSet<(u32, u32)> = self.path.difference(&extraneous).cloned().collect();
        for coord in extraneous {
            let mut new_coord = coord;
            loop {
                let paths: Vec<(u32, u32)> = CARDINALS
                    .iter()
                    .map(|&(_, rel_dir)| scale(new_coord, rel_dir, 1))
                    .filter(|&c| coords.contains(&c) || mod_path.contains(&c))
                    .collect();
                match paths.len() {
                    1 => {
                        mod_path.remove(&new_coord);
                        new_coord = paths[0];
                    }
                    _ => break
                }
            }
        }
        self.path = mod_path;
    }
}

impl Room {
    pub fn random<T: Rng>(rng: &mut T, width: u32, height: u32, room_size: u32, region: u32) -> Room {
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
            region: region,
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

    pub fn border(&self) -> Vec<(u32, u32)> {
        let mut surrounding = vec![];
        for i in range(0, self.w) {
            surrounding.push(((self.x + i), self.y));
            surrounding.push(((self.x + i), self.y + self.h - 1));
        }
        for i in range(0, self.h) {
            surrounding.push((self.x, self.y + i));
            surrounding.push((self.x + self.w - 1, self.y + i));
        }
        surrounding
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
        let branching = config.get_float(desirables, "branching");
        assert!(room_number > 0);
        DesirableProperties {
            seed: seed.clone(),
            room_size: room_size,
            room_number: room_number,
            doors: doors,
            monsters: monsters,
            branching: branching,
            occupants: vec![],
            rooms: vec![],
            mazes: vec![],
            connectors: vec![],
            entrance: (0, 0),
            exit: (0, 0),
        }
    }

}

impl Genotype for DesirableProperties {
    fn initialize<T: Rng>(&self, rng: &mut T) -> DesirableProperties {
        let w = self.seed.width;
        let h = self.seed.height;
        let occupants = self.seed.random_occupants(rng).iter().take(self.monsters as usize).cloned().collect();
        let mut region = 0;
        // randomly generate rooms
        let mut rooms = vec![];
        for _ in range(0, self.room_number) {
            for _ in range(0, 10) {
                let room = Room::random(rng, w, h, self.room_size, region);
                if !room.intersects(&rooms) {
                    rooms.push(room);
                    region += 1;
                    break;
                }
            }
        }
        // fill in mazes
        let mut mazes = vec![];
        let mut positions: HashSet<(u32, u32)> = HashSet::new();
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
        let all_connectors = Connector::find_all(&mazes, &rooms);
        let filtered_connectors = Connector::merge(rng, &all_connectors);
        // remove dead ends
        positions.clear();
        for maze in mazes.iter_mut() {
            maze.prune(&filtered_connectors);
            positions = positions.union(&maze.path).cloned().collect();
        }
        let connectors: Vec<Connector> = filtered_connectors.iter().filter(|&c| {
            // must have at least two adjacent edges
            let is_occupied = |(x, y)| rooms.clone().iter().fold(false, |accum, ref m| accum || m.contains(x, y));
            let possibles: Vec<(u32, u32)> = CARDINALS
                .iter()
                .map(|&(_, rel_dir)| scale(c.location, rel_dir, 1))
                .filter(|&new_coord| is_occupied(new_coord) || positions.contains(&new_coord))
                .collect();
            possibles.len() > 1
        }).cloned().collect();
        // randomly place entrance/exit in separate rooms
        let result: Vec<(u32, u32)> = range(0, 2).map(|_| {
            let ref room = rng.choose(rooms.as_slice()).unwrap();
            let xw = room.x + room.w - 1;
            let xh = room.y + room.h - 1;
            (rng.gen_range(room.x, xw), rng.gen_range(room.y, xh))
        }).collect();
        let entrance = result[0];
        let exit = result[1];
        DesirableProperties {
            seed: self.seed.clone(),
            room_number: self.room_number,
            room_size: self.room_size,
            doors: self.doors,
            monsters: self.monsters,
            branching: self.branching,
            occupants: occupants,
            rooms: rooms.clone(),
            mazes: mazes,
            connectors: connectors,
            entrance: entrance,
            exit: exit,
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
        let door = self.seed.tiles.get("door").unwrap();
        let floor = self.seed.tiles.get("floor").unwrap();
        for room in self.rooms.iter() {
            for i in range(room.x, room.x + room.w) {
                for j in range(room.y, room.y + room.h) {
                    dungeon.cells[i as usize][j as usize].tile = Some(floor.clone())
                }
            }
            // will overdraw, but that's OK.
            for (i, j) in room.border() {
                dungeon.cells[i as usize][j as usize].tile = Some(wall.clone())
            }
        }
        for maze in self.mazes.iter() {
            for path in maze.path.iter() {
                let (x, y): (u32, u32) = *path;
                dungeon.cells[x as usize][y as usize].tile = Some(floor.clone());
            }
        }
        let mut door_number = 0;
        for connector in self.connectors.iter() {
            let (x, y) = connector.location;
            if door_number < self.doors {
                dungeon.cells[x as usize][y as usize].tile = Some(door.clone());
                door_number += 1;
            } else {
                dungeon.cells[x as usize][y as usize].tile = Some(floor.clone());
            }
            // make sure connectors are accessible (not walled off)
            let cell = dungeon.cells[x as usize][y as usize].clone();
            for sc in SurroundingCells::new(&dungeon, &cell, Surrounding::AllDirections) {
                if sc.has_attribute("wall") {
                    dungeon.cells[sc.x as usize][sc.y as usize].tile = Some(floor.clone());
                }
            }
        }
        // entrance/exit, if applicable
        let entrance = self.seed.tiles.get("entrance").unwrap();
        let (x, y) = self.entrance;
        dungeon.cells[x as usize][y as usize].tile = Some(entrance.clone());
        let exit = self.seed.tiles.get("exit").unwrap();
        let (x, y) = self.exit;
        dungeon.cells[x as usize][y as usize].tile = Some(exit.clone());
        // draw the occupants if their tile is not otherwise occupied.
        for (occupant, coord) in self.occupants.clone() {
            let x = coord.0 as usize;
            let y = coord.1 as usize;
            if dungeon.cells[x][y].has_attribute("floor") {
                dungeon.cells[x][y].occupant = Some(occupant.clone());
            }
        }
        dungeon.clone()
    }
}
