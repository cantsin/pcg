use freetype::{Face};
use opengl_graphics::{GlGraphics};
use input::{Event, Button, RenderEvent, PressEvent};
use input::keyboard::{Key};
use num_cpus::{get};

use std::path::{Path};
use std::sync::atomic::{AtomicUsize, AtomicIsize, Ordering, ATOMIC_USIZE_INIT, ATOMIC_ISIZE_INIT};

use chapter2::celloption::{CellOptions, CellOption, Tile, Item, Occupant};
use chapter2::dungeon::{Dungeon, DungeonCells};
use chapter2::genotype::{Genotype};
use chapter2::random_seed::{RandomSeed};
use chapter2::list_of_walls::{ListOfWalls};
use chapter2::wall_patterns::{WallPatterns};
use chapter2::desirable_properties::{DesirableProperties};
use chapter2::mu_lambda::{MuLambda};
use chapter2::evaluation::{EvaluationFn, check_1x1_rooms, has_entrance_exit, doors_are_useful, rooms_are_accessible};
use chapter2::phenotype::{Seed};
use chapter2::statistics::{Statistic};

use util::spritesheet::{SpriteSheet};
use util::sprite::{Sprite};
use util::text::{render_text};
use util::config::{Config};

static FRAME: AtomicUsize = ATOMIC_USIZE_INIT;
static CHOICE: AtomicIsize = ATOMIC_ISIZE_INIT;

pub fn chapter2_entry(config: &Config) -> Box<Fn(&mut GlGraphics, &mut Face, Event) -> ()> {

    // load a whole bunch of configurable options
    let vars = config.get_table(None, "main");
    let tile_width = config.get_integer(vars, "tile_width") as i32;
    let tile_height = config.get_integer(vars, "tile_height") as i32;
    let tiles_width = config.get_default(vars, "tiles_width", 50);
    let tiles_height = config.get_default(vars, "tiles_height", 50);
    let animation_speed = config.get_default(vars, "animation_speed", 10);
    let threads = config.get_default(vars, "threads", get() * 2);

    let spritesheet_name = config.get_string(vars, "spritesheet");
    let spritesheets = config.get_table(None, "spritesheets");
    let spritesheet_config = config.get_table(Some(spritesheets), spritesheet_name);
    let spritesheet_location = config.get_string(spritesheet_config, "path");

    let cell_data = config.get_table(Some(spritesheet_config), "cells");
    let tiles: Vec<String> = config.get_array(cell_data, "tiles");
    let items: Vec<String> = config.get_array(cell_data, "items");
    let occupants: Vec<String> = config.get_array(cell_data, "occupants");
    let cell_tiles: CellOptions<Tile> = CellOptions::new(&tiles[..]);
    let cell_items: CellOptions<Item> = CellOptions::new(&items[..]);
    let cell_occupants: CellOptions<Occupant> = CellOptions::new(&occupants[..]);
    let occupant_chance = config.get_float(spritesheet_config, "occupant_chance");

    let mulambda_vars = config.get_table(None, "mu-lambda");
    let mu = config.get_default(mulambda_vars, "mu", 100);
    let lambda = config.get_default(mulambda_vars, "lambda", 100);
    let mutation = config.get_default(mulambda_vars, "mutation", 0.33);
    let iterations = config.get_default(mulambda_vars, "iterations", 100);
    let strategy = config.get_string(mulambda_vars, "strategy");
    let evaluations: Vec<String> = config.get_array(mulambda_vars, "evaluations");
    let evaluation_fns: Vec<EvaluationFn> = evaluations.iter().map(|eval| {
        match &eval[..] {
            "check_1x1_rooms" => box check_1x1_rooms as EvaluationFn,
            "has_entrance_exit" => box has_entrance_exit as EvaluationFn,
            "doors_are_useful" => box doors_are_useful as EvaluationFn,
            "rooms_are_accessible" => box rooms_are_accessible as EvaluationFn,
            _ => panic!("Evaluation function {} could not be found.", eval)
        }
    }).collect();
    let evaluation_weights: Vec<f64> = config.get_array(mulambda_vars, "evaluation_weights");

    let seed = Seed::new(tiles_width, tiles_height, cell_tiles, cell_items, cell_occupants, occupant_chance);

    let spritesheet_path = Path::new(spritesheet_location);
    let spritesheet = SpriteSheet::new(&spritesheet_path);

    // We cannot have trait objects that implement Clone or use
    // generic parameters. Instead, we use macros to make this section
    // a bit cleaner.
    macro_rules! mu_lambda_run (
        ($genotype:expr) => {{
            let mut mulambda = MuLambda::new(threads,
                                             iterations,
                                             mu,
                                             lambda,
                                             mutation,
                                             $genotype.clone(),
                                             evaluation_fns,
                                             evaluation_weights);
            let result = mulambda.run();
            result.into_iter().map(|(individual, statistic)| (individual.generate(), statistic)).collect()
        }}
        );
    let winners: Vec<(Dungeon, Statistic)> = match strategy {
        "RandomSeed" => {
            let genotype = RandomSeed::new(&seed);
            mu_lambda_run!(genotype)
        }
        "ListOfWalls" => {
            let genotype = ListOfWalls::new(&config, &seed);
            mu_lambda_run!(genotype)
        }
        "WallPatterns" => {
            let genotype = WallPatterns::new(&config, &seed);
            mu_lambda_run!(genotype)
        }
        "DesirableProperties" => {
            let genotype = DesirableProperties::new(&config, &seed);
            mu_lambda_run!(genotype)
        }
        _ => panic!("Strategy {} could not be found.", strategy)
    };

    box move |gl: &mut GlGraphics, face: &mut Face, e: Event| {
        let choice = CHOICE.load(Ordering::Relaxed);
        let frame = FRAME.load(Ordering::Relaxed);

        let ref current = winners[choice as usize];
        let &(ref dungeon, ref statistic) = current;
        let seconds = frame / animation_speed;
        if let Some(args) = e.render_args() {
            let dc = DungeonCells::new(&dungeon);
            for cell in dc {
                let x = cell.x as i32 * tile_width;
                let y = cell.y as i32 * tile_height;
                match cell.tile {
                    Some(ref val) => {
                        let sprite = spritesheet.sprites.get(&val.name()).unwrap();
                        sprite.draw(gl, args.viewport(), x, y, seconds);
                    }
                    None => {
                        Sprite::missing(gl, args.viewport(), x, y, tile_width, tile_height);
                    }
                }
                match cell.occupant {
                    Some(ref val) => {
                        let sprite = spritesheet.sprites.get(&val.name()).unwrap();
                        sprite.draw(gl, args.viewport(), x, y, seconds);
                    }
                    None => ()
                }
            }
            let info = format!("Dungeon no. #{} (born on iteration {}, ranking {})",
                               choice,
                               statistic.iteration,
                               statistic.fitness);
            render_text(face, gl, args.viewport(), 10.0, 410.0, &info[..]);
        };

        if let Some(Button::Keyboard(key)) = e.press_args() {
            let n = winners.len() as isize;
            if key == Key::Left {
                CHOICE.fetch_sub(1, Ordering::Relaxed);
                let new_choice = CHOICE.load(Ordering::Relaxed);
                if new_choice < 0 {
                    CHOICE.store(n - 1, Ordering::Relaxed);
                    FRAME.store(0, Ordering::Relaxed);
                }
            }
            else if key == Key::Right {
                CHOICE.fetch_add(1, Ordering::Relaxed);
                let new_choice = CHOICE.load(Ordering::Relaxed);
                CHOICE.store(new_choice % n, Ordering::Relaxed);
                FRAME.store(0, Ordering::Relaxed);
            }
        };

        FRAME.fetch_add(1, Ordering::Relaxed);
    }
}
