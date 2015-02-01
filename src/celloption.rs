use std::rand::{Rng, sample};

pub trait CellOption {
    fn new(String) -> Self;
    fn name(&self) -> String;
}

#[derive(Clone)]
pub struct Tile { data: String }

impl CellOption for Tile {
    fn new(data: String) -> Tile {
        Tile {
            data: data
        }
    }

    fn name(&self) -> String {
        self.data.clone()
    }
}

#[derive(Clone)]
pub enum Item { Data(String) }

#[derive(Clone)]
pub enum Occupant { Data(String) }

pub struct CellOptions<T> {
    options: Vec<T>
}

impl<T: CellOption> CellOptions<T> {
    pub fn new(names: &[&str]) -> CellOptions<T> {
        CellOptions {
            options: names.iter().map(|&name| CellOption::new(String::from_str(name))).collect()
        }
    }

    pub fn choose<R: Rng>(&self, rng: &mut R) -> &T {
        assert!(self.options.len() > 0, "Cannot retrieve random cell tile.");
        sample(rng, self.options.iter(), 1).into_iter().next().unwrap().clone()
    }
}
