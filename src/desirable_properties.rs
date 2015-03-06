use dungeon::{Dungeon};
use celloption::{Occupant};
use genotype::{Genotype};
use phenotype::{Seed};
use config::{Config};

use rand::{Rng};

#[derive(Clone, Debug)]
pub struct DesirableProperties {
    seed: Seed,
    room_number: u32,
    room_size: u32,
    rooms: Vec<Room>,
    doors: u32,
    monsters: u32,
    path_length: u32,
    branching: u32,
    occupants: Vec<(Occupant, (u32, u32))>,
}

#[derive(Clone, Debug)]
pub struct Room {
    x: u32,
    y: u32,
    w: u32,
    h: u32
}

impl Room {
    pub fn random<T: Rng>(rng: &mut T, width: u32, height: u32, room_size: u32) -> Room {
        let w: u32 = rng.gen_range(2, room_size);
        let h: u32 = rng.gen_range(2, room_size);
        assert!(width >= w);
        assert!(height >= h);
        let x: u32 = rng.gen_range(1, width - w);
        let y: u32 = rng.gen_range(1, height - h);
        Room {
            x: x,
            y: y,
            w: w,
            h: h
        }
    }

    pub fn intersects(rooms: &Vec<Room>, room: &Room) -> bool {
        for r in rooms {
            let rw = r.x + r.w;
            let rh = r.y + r.h;
            let roomw = room.x + room.w;
            let roomh = room.y + room.h;
            if roomw >= r.x && roomh >= r.y && room.x <= rw && room.y <= rh {
                return true
            }
        }
        false
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
            rooms: vec![],
            doors: doors,
            monsters: monsters,
            path_length: path_length,
            branching: branching,
            occupants: vec![],
        }
    }

}

impl Genotype for DesirableProperties {
    fn initialize<T: Rng>(&self, rng: &mut T) -> DesirableProperties {
        let occupants = self.seed.random_occupants(rng).iter().take(self.monsters as usize).cloned().collect();
        let mut rooms = self.rooms.clone();
        for _ in range(0, self.room_number) {
            let room = Room::random(rng, self.seed.width, self.seed.height, self.room_size);
            for _ in range(0, 10) {
                if !Room::intersects(&rooms, &room) {
                    rooms.push(room);
                    break;
                }
            }
        }
        DesirableProperties {
            seed: self.seed.clone(),
            room_number: self.room_number,
            room_size: self.room_size,
            rooms: rooms,
            doors: self.doors,
            monsters: self.monsters,
            path_length: self.path_length,
            branching: self.branching,
            occupants: occupants,
        }
    }

    fn mutate<T: Rng>(&mut self, rng: &mut T, percentage: f64) {
        // randomly generate rooms

        // fill in mazes

        // find connectors

        // remove dead ends
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
        dungeon.clone()
    }
}
