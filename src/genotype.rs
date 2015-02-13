use dungeon::{Dungeon};
use evaluation::{EvaluationFn};

pub trait GenoType {
    fn mutate(&mut self);
    fn generate(&self) -> Dungeon;
    fn last(&self) -> Dungeon;

    fn evaluate(&self, dungeon: &Dungeon, strategies: &[EvaluationFn]) -> f64 {
        strategies.iter().fold(1.0, |accum, f| accum * f(dungeon))
    }
}
