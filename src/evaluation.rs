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

pub fn has_entrance_exit(dungeon: &Dungeon) -> f64 {
    let mut has_entrance = false;
    let mut has_exit = false;
    for i in 0..dungeon.width {
        for j in 0..dungeon.height {
            let ref cell = dungeon.cells[i][j];
            if cell.has_attribute("entrance") {
                has_entrance = true;
            }
            if cell.has_attribute("exit") {
                has_exit = true;
            }
        }
    }
    match (has_entrance, has_exit) {
        (true, true) => 2.0,
        (false, true) => 1.0,
        (true, false) => 1.0,
        _ => 0.0,
    }
}

pub fn doors_are_useful(dungeon: &Dungeon) -> f64 {
    let mut hits = 0;
    for i in 0..dungeon.width {
        for j in 0..dungeon.height {
            let ref cell = dungeon.cells[i][j];
            // check to see if doors have exactly two walls abutting them
            if cell.has_attribute("door") {
                let mut count = 0;
                for sc in SurroundingCells::new(dungeon, cell, Surrounding::Cardinal) {
                    if sc.has_attribute("wall") {
                        count += 1;
                    }
                }
                if count != 2 {
                    hits += 1;
                }
            }
        }
    }
    hits as f64
}
