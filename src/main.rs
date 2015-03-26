#![feature(os, step_by, old_path, path_ext, core, box_syntax, box_patterns, collections)]
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
    pub mod entry;
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
use sdl2_window::{Sdl2Window};
use window::{WindowSettings};
use event::{Ups, MaxFps};
use quack::{Set};
use graphics::{color};

use std::cell::{RefCell};
use std::path::{Path};
use std::old_path::Path as OldPath;

use util::config::{Config};

use chapter2::entry::{chapter2_entry};

const TOML_CONFIG: &'static str = "src/chapter2/chapter2.toml";

fn main() {

    let config_path = Path::new(TOML_CONFIG);
    let config = Config::new(config_path);
    let vars = config.get_table(None, "main");
    let window_width = config.get_default(vars, "window_width", 800);
    let window_height = config.get_default(vars, "window_height", 800);
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

    let cb = chapter2_entry(&config);
    let window = RefCell::new(window);
    for e in event::events(&window).set(Ups(fps)).set(MaxFps(fps)) {
        graphics::clear(color::BLACK, gl);
        cb(gl, &mut face, e);
    }
}
