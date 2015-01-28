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

use cell::{CellOccupant};
use dungeon::{Dungeon};
use spritesheet::{SpriteSheet};

fn main() {
    let opengl = shader_version::OpenGL::_3_2;
    let window = Sdl2Window::new(opengl,
                                 event::WindowSettings {
                                     title: "PCG".to_string(),
                                     size: [300, 300],
                                     fullscreen: false,
                                     exit_on_esc: true,
                                     samples: 0,
                                 });
    let ref mut gl = Gl::new(opengl);

    // test.
    let spritesheet = SpriteSheet::new("./assets/16x16_Jerom_CC-BY-SA-3.0_0.png", 16, 16);
    let sprite = spritesheet.get_sprite("floor").unwrap();
    let sprite2 = spritesheet.get_sprite("monster").unwrap();

    let dungeon = Dungeon::new(50, 50);

    // TODO. randomly generate a map.

    let window = RefCell::new(window);
    for e in event::events(&window) {
        e.render(|args| {
            gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                clear([1.0; 4], gl);
                sprite.draw(&spritesheet.texture, &c, gl);
            });
        });
    }
}
