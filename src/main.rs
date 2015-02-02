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

mod config;
mod sprite;
mod spritesheet;
mod dungeon;
mod cell;
mod celloption;

use std::rand::{thread_rng};
use std::cell::RefCell;
use opengl_graphics::{Gl};
use sdl2_window::Sdl2Window;
use input::Button::Keyboard;
use input::keyboard::Key;
use event::*;

use celloption::{CellOptions, CellOption, Tile, Occupant};
use spritesheet::{SpriteSheet};
use dungeon::{Dungeon};
use config::{Config};

const TOML_CONFIG: &'static str = "src/config.toml";

fn main() {

    let config_path = Path::new(TOML_CONFIG);
    let config = Config::new(&config_path);
    let vars = config.get_table(None, "main");
    let window_width = config.get_default(vars, "window_width", 800);
    let window_height = config.get_default(vars, "window_height", 800);

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
    let occupants: Vec<String> = config.get_array(cell_data, "occupants");
    let cell_tiles: CellOptions<Tile> = CellOptions::new(tiles.as_slice());
    let cell_occupants: CellOptions<Occupant> = CellOptions::new(occupants.as_slice());

    // randomly generate a map.
    let mut rng = thread_rng();
    let tiles_width = 50us;
    let tiles_height = 50us;
    let mut dungeon = Dungeon::new(tiles_width, tiles_height);
    for i in 0..tiles_width {
        for j in 0..tiles_height {
            let tile = cell_tiles.choose(&mut rng).clone();
            dungeon.cells[i][j].tile = Some(tile);
            let occupants = cell_occupants.sample(&mut rng, 2);
            for occupant in occupants.iter() {
                dungeon.cells[i][j].add(*occupant);
            }
        }
    }

    let spritesheet_path = Path::new(spritesheet_location);
    let spritesheet = SpriteSheet::new(&spritesheet_path);

    let window = RefCell::new(window);
    for e in event::events(&window) {
        e.render(|_| {
            for i in 0..tiles_width {
                for j in 0..tiles_height {
                    let tile = dungeon.cells[i][j].tile.clone();
                    match tile {
                        Some(ref val) => {
                            let sprite = spritesheet.sprites.get(&val.name()).unwrap();
                            sprite.draw(gl, i as i32 * 16, j as i32 * 16);
                        }
                        None => ()
                    };
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
