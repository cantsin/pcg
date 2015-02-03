use dungeon::{Dungeon};

pub trait GenoType {
    fn mutate();
    fn generate() -> Dungeon;

    fn evaluate<F: Fn(&Dungeon) -> f64>(dungeon: &Dungeon, strategies: &[F]) -> f64 {
        strategies.iter().fold(1.0, |accum, f| accum * f(dungeon))
    }
}
