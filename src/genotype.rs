use dungeon::{Dungeon};
use evaluation::{EvaluationFn};

use rand::{Rng};

pub trait Genotype: Send + Clone {
    /// initialize the genotype.
    fn initialize<R: Rng>(&self, _: &mut R) -> Self { self.clone() }
    /// mutate the genotype.
    fn mutate<R: Rng>(&mut self, _: &mut R) { }
    /// generate a phenotype.
    fn generate<R: Rng>(&self, rng: &mut R) -> Dungeon;

    fn evaluate(&self, dungeon: &Dungeon, strategies: &[EvaluationFn]) -> f64 {
        strategies.iter().fold(1.0, |accum, f| accum * f(dungeon))
    }
}
