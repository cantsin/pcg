use std::rand::{Rng};

pub type Coords = (i32, i32);

// Fisher-Yates algorithm
pub fn shuffle<'a, R: Rng, T: Clone>(rng: &mut R, array: &'a mut [T]) {
    let mut n = array.len();
    while n > 1 {
        let k = rng.gen_range(0, n);
        n -= 1;
        array.swap(n, k);
    }
}
