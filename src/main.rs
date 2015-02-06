#![allow(unstable)]
#![feature(box_syntax)]

extern crate toml;
extern crate shader_version;
extern crate input;
//extern crate sprite;
extern crate event;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
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

use std::cell::RefCell;
use opengl_graphics::{Gl};
use sdl2_window::Sdl2Window;
use input::Button::Keyboard;
use input::keyboard::Key;
use event::*;

use celloption::{CellOptions, CellOption, Tile, Item, Occupant};
use spritesheet::{SpriteSheet};
use dungeon::{Dungeon, DungeonCells, SurroundingCells};
use config::{Config};
use genotype::{GenoType, RandomSeed};
use mu_lambda::{MuLambda};

const TOML_CONFIG: &'static str = "src/config.toml";

fn main() {

    let config_path = Path::new(TOML_CONFIG);
    let config = Config::new(&config_path);
    let vars = config.get_table(None, "main");
    let window_width = config.get_default(vars, "window_width", 800);
    let window_height = config.get_default(vars, "window_height", 800);
    let tiles_width = config.get_default(vars, "tiles_width", 50);
    let tiles_height = config.get_default(vars, "tiles_height", 50);

    let opengl = shader_version::OpenGL::_3_2;
    let window = Sdl2Window::new(opengl,
                                 event::WindowSettings {
                                     title: "PCG".to_string(),
                                     size: [window_width, window_height],
                                     fullscreen: false,
                                     exit_on_esc: true,
                                     samples: 0,
                                 });
    let ref mut gl = Gl::new(opengl);

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
    let genotype = match strategy {
        "RandomSeed" => {
            RandomSeed::new(tiles_width, tiles_height, cell_tiles, cell_items, cell_occupants)
        }
        _ => panic!(format!("Strategy {} could not be found.", strategy))
    };

    let mulambda = MuLambda::new(iterations, mu, lambda, genotype.clone());

    let dungeon = genotype.generate();

    let mut dc = DungeonCells::new(&dungeon);

    let spritesheet_path = Path::new(spritesheet_location);
    let spritesheet = SpriteSheet::new(&spritesheet_path);

    let window = RefCell::new(window);
    for e in event::events(&window) {
        e.render(|_| {
            for cell in dc {
                match cell.tile {
                    Some(ref val) => {
                        let sprite = spritesheet.sprites.get(&val.name()).unwrap();
                        sprite.draw(gl, cell.x as i32 * 16, cell.y as i32 * 16);
                    }
                    None => ()
                }
            }
        });

        if let Some(Keyboard(key)) = e.press_args() {
            if key == Key::J {
            }
            println!("Pressed keyboard key '{:?}'", key);
        };
    }
}
