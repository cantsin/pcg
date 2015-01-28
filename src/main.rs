#![allow(unstable)]

extern crate toml;
extern crate shader_version;
extern crate input;
extern crate sprite;
extern crate event;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

mod spritesheet;
mod dungeon;
mod cell;

use std::{rand};
use std::cell::RefCell;
use opengl_graphics::{Gl};
use sdl2_window::Sdl2Window;
// use input::Button::Keyboard;
// use input::keyboard::Key;
use event::RenderEvent;
use graphics::{clear};

//use cell::{CellOccupant};
use dungeon::{Dungeon};
use spritesheet::{SpriteSheet};

fn main() {
    let tiles_width = 50us;
    let tiles_height = 50us;
    let tile_width = 16;
    let tile_height = 16;
    let spritesheet_filename = "./assets/16x16_Jerom_CC-BY-SA-3.0_0.png";

    let screen_width = tiles_width * tile_width;
    let screen_height = tiles_height * tile_height;

    let opengl = shader_version::OpenGL::_3_2;
    let window = Sdl2Window::new(opengl,
                                 event::WindowSettings {
                                     title: "PCG".to_string(),
                                     size: [screen_width as u32, screen_height as u32],
                                     fullscreen: false,
                                     exit_on_esc: true,
                                     samples: 0,
                                 });
    let ref mut gl = Gl::new(opengl);
    let spritesheet = SpriteSheet::new(spritesheet_filename, 16, 16);
    // let treasure = spritesheet.get_sprite("treasure").unwrap();
    // let monster = spritesheet.get_sprite("monster").unwrap();
    let trap = spritesheet.get_sprite("trap").expect("trap");

    // randomly generate a map.
    let mut dungeon = Dungeon::new(tiles_width, tiles_height);
    for i in 0..tiles_width {
        for j in 0..tiles_height {
            //dungeon.cells[i][j].add(rand::random::<CellOccupant>());
        }
    }

    let window = RefCell::new(window);
    for e in event::events(&window) {
        e.render(|args| {
            gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                clear([1.0; 4], gl);
                trap.draw(&spritesheet.texture, &c, gl);
            });
        });
    }
}
