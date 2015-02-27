use rand::{Rng};
use celloption::{CellOptions, CellOption, Tile, Occupant};

// Fisher-Yates algorithm
pub fn shuffle<'a, R: Rng, T: Clone>(rng: &mut R, array: &'a mut [T]) {
    let mut n = array.len();
    while n > 1 {
        let k = rng.gen_range(0, n);
        n -= 1;
        array.swap(n, k);
    }
}

pub fn odds<R: Rng>(rng: &mut R, num: u64, den: u64) -> bool {
    num > rng.gen_range(0, den)
}

// this function is not strictly necessary but is available purely to add more exciting visuals.
pub fn random_occupant<R: Rng>(rng: &mut R, tile: &Tile, options: &CellOptions<Occupant>, chance: f64) -> Option<Occupant> {
    let percentage = (chance * 100.0) as u64;
    if tile.name() == "floor" && odds(rng, percentage, 100) {
        Some(options.choose(rng).clone())
    } else {
        None
    }
}
