use dungeon::{Dungeon, SurroundingCells};

type EvaluationFn = Box<Fn(&Dungeon) -> f64 + 'static + Send + Sync + Copy>;

pub struct Evaluation {
    evaluation: EvaluationFn
}

impl Evaluation {
    pub fn new(evaluation: EvaluationFn) -> Evaluation {
        Evaluation { evaluation: evaluation }
    }

    pub fn call(&self, dungeon: &Dungeon) -> f64 {
        (*self.evaluation)(dungeon)
    }
}

pub fn check_1x1_rooms(dungeon: &Dungeon) -> f64 {
    let mut hits = 0;
    for i in 0..dungeon.width {
        for j in 0..dungeon.height {
            let ref cell = dungeon.cells[i][j];
            if cell.is_empty() {
                let sc = SurroundingCells::new(dungeon, cell, false);
                // check to see if all walls surround us.
                let surrounded = sc.fold(true, |accum, c| accum && c.is_empty());
                if surrounded {
                    hits += 1;
                }
            }
        }
    }
    hits as f64
}
