use rand::{Rng, sample};
use std::marker::{PhantomData};

/// cell options store data for a particular cell type.
/// currently we only store the tile name for rendering.

pub trait CellOption {
    fn new(String) -> Self;
    fn name(&self) -> String;
}

#[derive(Clone, Debug)]
struct CellData<A> {
    data: String,
    _marker: PhantomData<A>
}

impl<A> CellOption for CellData<A> {
    fn new(data: String) -> CellData<A> {
        CellData {
            data: data,
            _marker: PhantomData
        }
    }

    fn name(&self) -> String {
        self.data.clone()
    }
}

#[derive(Clone, Debug)]
pub struct CellOptions<T> {
    options: Vec<T>
}

impl<T: CellOption> CellOptions<T> {
    pub fn new(names: &[String]) -> CellOptions<T> {
        let options = names.iter().map(|name| CellOption::new(name.clone())).collect();
        CellOptions {
            options: options
        }
    }

    pub fn get(&self, name: &str) -> Option<&T> {
        let options: Vec<&T> = self.options.iter().filter(|&opt| {
            &opt.name()[..] == name
        }).collect();
        match options.is_empty() {
            true => None,
            false => Some(options[0].clone())
        }
    }

    pub fn choose<R: Rng>(&self, rng: &mut R) -> &T {
        assert!(self.options.len() > 0, "Cannot choose random cell option.");
        sample(rng, self.options.iter(), 1).into_iter().next().unwrap()
    }
}

#[derive(Clone, Debug)]
struct _Tile;

#[derive(Clone, Debug)]
struct _Item;

#[derive(Clone, Debug)]
struct _Occupant;

pub type Tile = CellData<_Tile>;
pub type Item = CellData<_Item>;
pub type Occupant = CellData<_Occupant>;
