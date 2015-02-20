#![feature(io, os, core, path, std_misc, box_syntax, collections)]
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
extern crate "rustc-serialize" as rustc_serialize;

mod util;
mod config;
mod sprite;
mod spritesheet;
mod dungeon;
mod cell;
mod celloption;
mod genotype;
mod mu_lambda;
mod evaluation;
mod random_seed;
mod list_of_walls;
mod text;

use opengl_graphics::{Gl};
use graphics::{color};
use sdl2_window::{Sdl2Window};
use window::{WindowSettings};
use input::Button::{Keyboard};
use input::keyboard::{Key};
//use event::{RenderEvent, Event};
use event::*;

use std::os::{num_cpus};
use std::cell::{RefCell};

use celloption::{CellOptions, CellOption, Tile, Item, Occupant};
use spritesheet::{SpriteSheet};
use sprite::{Sprite};
use dungeon::{DungeonCells};
use config::{Config};
use genotype::{GenoType};
use random_seed::{RandomSeed};
use list_of_walls::{ListOfWalls};
use mu_lambda::{MuLambda};
use evaluation::{EvaluationFn, check_1x1_rooms};
use text::{render_text};

const TOML_CONFIG: &'static str = "src/config.toml";

fn main() {

    let config_path = Path::new(TOML_CONFIG);
    let config = Config::new(&config_path);
    let vars = config.get_table(None, "main");
    let window_width = config.get_default(vars, "window_width", 800);
    let window_height = config.get_default(vars, "window_height", 800);
    let tiles_width = config.get_default(vars, "tiles_width", 50);
    let tiles_height = config.get_default(vars, "tiles_height", 50);
    let threads = config.get_default(vars, "threads", num_cpus() * 2);
    let font_name = config.get_string(vars, "font");
    let font_size = config.get_default(vars, "font_size", 14);

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
    let font = Path::new(font_name);
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

    let mulambda_vars = config.get_table(None, "mu-lambda");
    let mu = config.get_default(mulambda_vars, "mu", 100);
    let lambda = config.get_default(mulambda_vars, "lambda", 100);
    let iterations = config.get_default(mulambda_vars, "iterations", 100);
    let strategy = config.get_string(mulambda_vars, "strategy");
    let evaluations: Vec<String> = config.get_array(mulambda_vars, "evaluations");
    let evaluation_fns: Vec<EvaluationFn> = evaluations.iter().map(|eval| {
        match eval.as_slice() {
            "check_1x1_rooms" => { box check_1x1_rooms as EvaluationFn }
            _ => panic!(format!("Evaluation function {} could not be found.", eval))
        }
    }).collect();

    let genotype = match strategy {
        // "RandomSeed" => {
        //     box RandomSeed::new(tiles_width, tiles_height, cell_tiles, cell_items, cell_occupants)
        // }
        "ListOfWalls" => {
            box ListOfWalls::new(tiles_width, tiles_height, cell_tiles, cell_items, cell_occupants)
        }
        _ => panic!(format!("Strategy {} could not be found.", strategy))
    };

    let mut mulambda = MuLambda::new(threads,
                                     iterations,
                                     mu,
                                     lambda,
                                     *genotype.clone(),
                                     evaluation_fns);
    let winners = mulambda.evaluate();

    let spritesheet_path = Path::new(spritesheet_location);
    let spritesheet = SpriteSheet::new(&spritesheet_path);

    let mut choice = 0is;
    let window = RefCell::new(window);
    for e in event::events(&window) {
        graphics::clear(color::BLACK, gl);
        let ref current = winners[choice as usize];
        let dungeon = current.last();
        e.render(|_| {
            let dc = DungeonCells::new(&dungeon);
            for cell in dc {
                let w = 16;
                let h = 16;
                let x = cell.x as i32 * w;
                let y = cell.y as i32 * h;
                match cell.tile {
                    Some(ref val) => {
                        let sprite = spritesheet.sprites.get(&val.name()).unwrap();
                        sprite.draw(gl, x, y);
                    }
                    None => {
                         Sprite::missing(gl, x, y, w, h);
                    }
                }
            }
            let info = format!("Dungeon no. #{} (born on iteration {}, ranking {})",
                               choice,
                               current.iteration,
                               current.ranking);
            render_text(&mut face, gl, 10, 10, info.as_slice());
        });

        if let Some(Keyboard(key)) = e.press_args() {
            if key == Key::Left {
                choice -= 1;
                if choice < 0 {
                    choice = (winners.len() - 1) as isize;
                }
            }
            else if key == Key::Right {
                choice += 1;
                choice %= winners.len() as isize;
            }
        };
    }
}
