#![allow(unstable)]

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

use std::rand::{thread_rng, sample};
use std::cell::RefCell;
use opengl_graphics::{Gl};
use sdl2_window::Sdl2Window;
use input::Button::Keyboard;
use input::keyboard::Key;
use event::*;
use graphics::{clear};

use cell::{CellTile, CellOccupant};
use dungeon::{Dungeon};
use spritesheet::{SpriteSheet};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

fn main() {

    let opengl = shader_version::OpenGL::_3_2;
    let window = Sdl2Window::new(opengl,
                                 event::WindowSettings {
                                     title: "PCG".to_string(),
                                     size: [WINDOW_WIDTH, WINDOW_HEIGHT],
                                     fullscreen: false,
                                     exit_on_esc: true,
                                     samples: 0,
                                 });
    let ref mut gl = Gl::new(opengl);

    let spritesheet_path = Path::new("./assets/16x16_Jerom_CC-BY-SA-3.0_0.png");
    let spritesheet = SpriteSheet::new(&spritesheet_path);

    // read in config.toml
    // parse spreadsheet
    // parse dungeon cells
    // get cell.tiles, cell.occupants, cell.items as Vec<CellTile>, ...

    // // randomly generate a map.
    // let mut rng = thread_rng();
    // let tiles_width = 50us;
    // let tiles_height = 50us;
    // let mut dungeon = Dungeon::new(tiles_width, tiles_height);
    // for i in 0..tiles_width {
    //     for j in 0..tiles_height {
    //         let tile = sample(&mut rng, cell_tiles.iter(), 1);
    //         let occupant = sample(&mut rng, cell_occupants.iter(), 1);
    //         dungeon.cells[i][j].tile = Some(tile.into_iter().next().unwrap().clone());
    //         dungeon.cells[i][j].add(occupant.into_iter().next().unwrap());
    //     }
    // }

    let sprite = spritesheet.sprites.get("floor").unwrap();

    let window = RefCell::new(window);
    for e in event::events(&window) {
        e.render(|args| {
            sprite.draw(gl, 0, 0);

            // for i in 0..tiles_width {
            //     for j in 0..tiles_height {
            //         let ref tile = dungeon.cells[i][j].tile;
            //         let name = match *tile {
            //             Some(CellTile::Tile(ref name)) => name.as_slice(),
            //             None => "floor"
            //         };
            //         let sprite = spritesheet.get_unique_sprite("tiles", name).unwrap();
            //         gl.draw([i as i32 * 16, j as i32 * 16, 16, 16], |c, gl| {
            //             sprite.draw(&spritesheet.texture, &c, gl);
            //         });
            //     }
            // }
        });

        if let Some(Keyboard(key)) = e.press_args() {
            if key == Key::J {
            }
            println!("Pressed keyboard key '{:?}'", key);
        };
    }
}
