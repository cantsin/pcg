#![feature(os, step_by, old_path, path_ext, core, box_syntax, collections)]
#![forbid(unused_typecasts)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

extern crate rand;
extern crate toml;
extern crate shader_version;
extern crate input;
extern crate event;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate window;
extern crate freetype;
extern crate threadpool;
extern crate quack;
extern crate "rustc-serialize" as rustc_serialize;

pub mod util {
    pub mod util;
    pub mod config;
    pub mod sprite;
    pub mod spritesheet;
    pub mod text;
}

pub mod chapter2 {
    pub mod dungeon;
    pub mod cell;
    pub mod celloption;
    pub mod genotype;
    pub mod statistics;
    pub mod mu_lambda;
    pub mod evaluation;
    pub mod random_seed;
    pub mod list_of_walls;
    pub mod wall_patterns;
    pub mod desirable_properties;
    pub mod phenotype;
}

use opengl_graphics::{Gl};
use graphics::{RelativeTransform, color};
use sdl2_window::{Sdl2Window};
use window::{WindowSettings};
use input::Button::{Keyboard};
use input::keyboard::{Key};
use event::{Ups, MaxFps, RenderEvent, PressEvent};
use quack::{Set};

use std::os::{num_cpus};
use std::cell::{RefCell};
use std::path::{Path};
use std::old_path::Path as OldPath;

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
use util::text::{render_text};
use util::sprite::{Sprite};
use util::config::{Config};

const TOML_CONFIG: &'static str = "src/config.toml";

fn main() {

    let config_path = Path::new(TOML_CONFIG);
    let config = Config::new(config_path);
    let vars = config.get_table(None, "main");
    let window_width = config.get_default(vars, "window_width", 800);
    let window_height = config.get_default(vars, "window_height", 800);
    let tile_width = config.get_integer(vars, "tile_width") as i32;
    let tile_height = config.get_integer(vars, "tile_height") as i32;
    let tiles_width = config.get_default(vars, "tiles_width", 50);
    let tiles_height = config.get_default(vars, "tiles_height", 50);
    let animation_speed = config.get_default(vars, "animation_speed", 10);
    let threads = config.get_default(vars, "threads", num_cpus() * 2);
    let font_name = config.get_string(vars, "font");
    let font_size = config.get_default(vars, "font_size", 14);
    let fps = config.get_default(vars, "fps", 10);

    let opengl = shader_version::OpenGL::_3_2;
    let window = Sdl2Window::new(opengl,
                                 WindowSettings {
                                     title: "PCG".to_string(),
                                     size: [window_width, window_height],
                                     fullscreen: false,
                                     exit_on_esc: true,
                                     samples: 0,
                                 });
    let ref mut gl = Gl::new(opengl);

    let ft = freetype::Library::init().unwrap();
    let font = OldPath::new(font_name);
    let mut face = ft.new_face(&font, 0).unwrap();
    face.set_pixel_sizes(0, font_size).unwrap();

    let spritesheet_name = config.get_string(vars, "spritesheet");
    let spritesheets = config.get_table(None, "spritesheets");
    let spritesheet_config = config.get_table(Some(spritesheets), spritesheet_name);
    let spritesheet_location = config.get_string(spritesheet_config, "path");

    let cell_data = config.get_table(Some(spritesheet_config), "cells");
    let tiles: Vec<String> = config.get_array(cell_data, "tiles");
    let items: Vec<String> = config.get_array(cell_data, "items");
    let occupants: Vec<String> = config.get_array(cell_data, "occupants");
    let cell_tiles: CellOptions<Tile> = CellOptions::new(tiles.as_slice());
    let cell_items: CellOptions<Item> = CellOptions::new(items.as_slice());
    let cell_occupants: CellOptions<Occupant> = CellOptions::new(occupants.as_slice());
    let occupant_chance = config.get_float(spritesheet_config, "occupant_chance");

    let mulambda_vars = config.get_table(None, "mu-lambda");
    let mu = config.get_default(mulambda_vars, "mu", 100);
    let lambda = config.get_default(mulambda_vars, "lambda", 100);
    let mutation = config.get_default(mulambda_vars, "mutation", 0.33);
    let iterations = config.get_default(mulambda_vars, "iterations", 100);
    let strategy = config.get_string(mulambda_vars, "strategy");
    let evaluations: Vec<String> = config.get_array(mulambda_vars, "evaluations");
    let evaluation_fns: Vec<EvaluationFn> = evaluations.iter().map(|eval| {
        match eval.as_slice() {
            "check_1x1_rooms" => box check_1x1_rooms as EvaluationFn,
            "has_entrance_exit" => box has_entrance_exit as EvaluationFn,
            "doors_are_useful" => box doors_are_useful as EvaluationFn,
            "rooms_are_accessible" => box rooms_are_accessible as EvaluationFn,
            _ => panic!("Evaluation function {} could not be found.", eval)
        }
    }).collect();
    let evaluation_weights: Vec<f64> = config.get_array(mulambda_vars, "evaluation_weights");

    let seed = Seed::new(tiles_width, tiles_height, cell_tiles, cell_items, cell_occupants, occupant_chance);

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

    let spritesheet_path = Path::new(spritesheet_location);
    let spritesheet = SpriteSheet::new(&spritesheet_path);

    let mut frame = 0;
    let mut choice = 0;
    let window = RefCell::new(window);
    for e in event::events(&window).set(Ups(fps)).set(MaxFps(fps)) {
        graphics::clear(color::BLACK, gl);
        let ref current = winners[choice as usize];
        let &(ref dungeon, ref statistic) = current;
        let seconds = frame / animation_speed;
        e.render(|_| {
            let dc = DungeonCells::new(&dungeon);
            for cell in dc {
                let x = cell.x as i32 * tile_width;
                let y = cell.y as i32 * tile_height;
                match cell.tile {
                    Some(ref val) => {
                        let sprite = spritesheet.sprites.get(&val.name()).unwrap();
                        sprite.draw(gl, x, y, seconds);
                    }
                    None => {
                        Sprite::missing(gl, x, y, tile_width, tile_height);
                    }
                }
                match cell.occupant {
                    Some(ref val) => {
                        let sprite = spritesheet.sprites.get(&val.name()).unwrap();
                        sprite.draw(gl, x, y, seconds);
                    }
                    None => ()
                }
            }
            let info = format!("Dungeon no. #{} (born on iteration {}, ranking {})",
                               choice,
                               statistic.iteration,
                               statistic.fitness);
            gl.draw([0, 0, window_width as i32, window_height as i32], |c, gl| {
                let transform = c.transform.trans(10.0, 10.0);
                render_text(&mut face, gl, transform, info.as_slice());
            });
        });

        if let Some(Keyboard(key)) = e.press_args() {
            if key == Key::Left {
                choice -= 1;
                if choice < 0 {
                    choice = (winners.len() - 1) as isize;
                    frame = 0;
                }
            }
            else if key == Key::Right {
                choice += 1;
                choice %= winners.len() as isize;
                frame = 0;
            }
        };

        frame += 1;
    }
}
