use dungeon::{Dungeon};

pub type EvaluationFn = Box<Fn(&Dungeon) -> f64 + 'static>;
