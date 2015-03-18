use std::collections::{HashSet};

use cell::{Cell};
use dungeon::{Dungeon, DungeonCells, SurroundingCells, Surrounding};

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
        (false, false) => 2.0,
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
            // check to see if doors have exactly two walls or gaps abutting them
            if cell.has_attribute("door") {
                let mut count = 0;
                for sc in SurroundingCells::new(dungeon, cell, Surrounding::Cardinal) {
                    if sc.tile.is_none() || sc.has_attribute("wall") {
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

pub type Coords = HashSet<(u32, u32)>;

// helper function.
fn is_accessible(cell: &Cell) -> bool {
    cell.has_attribute("floor") || cell.has_attribute("door")
}

// recursively traverse the dungeon (depth-first)
fn search(dungeon: &Dungeon, visited: &Coords, x: u32, y: u32) -> Coords {
    let ref cell = dungeon.cells[x as usize][y as usize];
    let mut local = visited.clone();
    assert!(!local.contains(&(x, y)));
    local.insert((x, y));
    for sc in SurroundingCells::new(dungeon, cell, Surrounding::AllDirections) {
        let new_x = sc.x;
        let new_y = sc.y;
        if is_accessible(&sc) && !local.contains(&(new_x, new_y)) {
            let results = search(&dungeon, &local, new_x, new_y);
            let search_results: Coords = local.union(&results).cloned().collect();
            local = search_results.clone();
        }
    }
    local.clone()
}

pub fn rooms_are_accessible(dungeon: &Dungeon) -> f64 {
    let mut hits = 0;
    let dc = DungeonCells::new(&dungeon);
    let mut visited: Coords = HashSet::new();
    let floors: Coords = dc.filter(|ref cell| is_accessible(cell)).map(|cell| (cell.x, cell.y)).collect();
    while {
        let remaining: Coords = floors.difference(&visited).cloned().collect();
        match remaining.len() {
            0 => false,
            _ => {
                let &(x, y) = remaining.iter().next().unwrap();
                let swept = search(&dungeon, &visited, x, y);
                visited = visited.union(&swept).cloned().collect();
                let total: Coords = visited.intersection(&floors).cloned().collect();
                total.len() != floors.len()
            }
        }
    }
    {
        hits += 1;
    }
    hits as f64
}
