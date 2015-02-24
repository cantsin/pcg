
#[derive(Clone, Debug)]
pub struct Statistic {
    pub iteration: u32,
    pub ranking: f64
}

impl Statistic {
    pub fn new() -> Statistic {
        Statistic {
            iteration: 0,
            ranking: -1.0
        }
    }
}

pub trait Statistics {
    fn set_iteration(&mut self, v: u32);
    fn get_iteration(&self) -> u32;
    fn set_ranking(&mut self, v: f64);
    fn get_ranking(&self) -> f64;
}
