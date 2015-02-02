use std::rand::{Rng, sample};

/// cell options store data for a particular cell type.
/// currently we only store the tile name for rendering.

pub trait CellOption {
    fn new(String) -> Self;
    fn name(&self) -> String;
}

#[derive(Clone)]
struct CellData<A> { data: String }

impl<A> CellOption for CellData<A> {
    fn new(data: String) -> CellData<A> {
        CellData {
            data: data
        }
    }

    fn name(&self) -> String {
        self.data.clone()
    }
}

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
        assert!(self.options.len() > 0, "Cannot choose random cell option.");
        sample(rng, self.options.iter(), 1).into_iter().next().unwrap()
    }

    pub fn sample<R: Rng>(&self, rng: &mut R, n: usize) -> Vec<&T> {
        sample(rng, self.options.iter(), n)
    }
}

#[derive(Clone)]
struct _Tile;

#[derive(Clone)]
struct _Item;

#[derive(Clone)]
struct _Occupant;

pub type Tile = CellData<_Tile>;
pub type Item = CellData<_Item>;
pub type Occupant = CellData<_Occupant>;
