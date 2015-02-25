
#[derive(Clone, Debug)]
pub struct Statistic {
    pub iteration: u32,
    pub fitness: f64
}

impl Statistic {
    pub fn new(iteration: u32, fitness: f64) -> Statistic {
        Statistic {
            iteration: iteration,
            fitness: fitness
        }
    }

    pub fn empty() -> Statistic {
        Statistic {
            iteration: 0,
            fitness: -1.0
        }
    }
}
