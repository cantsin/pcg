use dungeon::{Dungeon, SurroundingCells, Surrounding};

pub type EvaluationFn = Box<Fn(&Dungeon) -> f64 + 'static + Send + Sync + Copy>;

pub fn check_1x1_rooms(dungeon: &Dungeon) -> f64 {
    let mut hits = 0;
    for i in 0..dungeon.width {
        for j in 0..dungeon.height {
            let ref cell = dungeon.cells[i][j];
            if cell.is_empty() {
                let sc = SurroundingCells::new(dungeon, cell, Surrounding::Cardinal);
                // check to see if all walls surround us.
                let surrounded = sc.fold(true, |accum, c| accum && !c.is_empty());
                if surrounded {
                    hits += 1;
                }
            }
        }
    }
    hits as f64
}
