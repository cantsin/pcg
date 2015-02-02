#![allow(unstable)]
#![feature(box_syntax)]
#![allow(dead_code)]

extern crate toml;
extern crate shader_version;
extern crate input;
//extern crate sprite;
extern crate event;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

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
use dungeon::{Dungeon};
use spritesheet::{SpriteSheet};

const TOML_CONFIG: &'static str = "config.toml";

fn main() {

    // read in config.toml
    let window_width = 800;
    let window_height = 800;
    let spritesheet_name = "dungeon";
    println!("Using spritesheet {}", spritesheet_name);

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

    // parse spritesheet
    // get cell.tiles, cell.occupants, cell.items...
    let spritesheet_path = Path::new("./assets/16x16_Jerom_CC-BY-SA-3.0_0.png");
    let spritesheet = SpriteSheet::new(&spritesheet_path);
    let tiles = ["floor", "wall", "entrance", "exit", "door"];
    let cell_tiles: CellOptions<Tile> = CellOptions::new(&tiles);
    let occupants = ["monster", "treasure", "trap", "teleporter"];
    let cell_occupants: CellOptions<Occupant> = CellOptions::new(&occupants);

    // randomly generate a map.
    let mut rng = thread_rng();
    let tiles_width = 50us;
    let tiles_height = 50us;
    let mut dungeon = Dungeon::new(tiles_width, tiles_height);
    for i in 0..tiles_width {
        for j in 0..tiles_height {
            let tile = cell_tiles.choose(&mut rng);
            dungeon.cells[i][j].tile = Some(tile.clone());
            let occupants = cell_occupants.sample(&mut rng, 2);
            for occupant in occupants.iter() {
                dungeon.cells[i][j].add(*occupant);
            }
        }
    }

    let window = RefCell::new(window);
    for e in event::events(&window) {
        e.render(|_| {
            for i in 0..tiles_width {
                for j in 0..tiles_height {
                    let tile = dungeon.cells[i][j].tile.clone();
                    let name = match tile {
                        Some(ref val) => val.name(),
                        None => String::from_str("floor")
                    };
                    let sprite = spritesheet.sprites.get(&name).unwrap();
                    sprite.draw(gl, i as i32 * 16, j as i32 * 16);
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
