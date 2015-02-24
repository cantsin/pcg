use dungeon::{Dungeon};
use evaluation::{EvaluationFn};

use rand::{ThreadRng};

pub trait GenoType: Send + Clone {
    fn mutate(&mut self, rng: &mut ThreadRng);
    fn generate(&mut self, rng: &mut ThreadRng) -> Dungeon;
    fn last(&self) -> Dungeon;

    fn evaluate(&self, dungeon: &Dungeon, strategies: &[EvaluationFn]) -> f64 {
        strategies.iter().fold(1.0, |accum, f| accum * f(dungeon))
    }
}
