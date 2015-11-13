#![feature(step_by, convert, box_syntax, box_patterns, vec_push_all)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

extern crate rand;
extern crate toml;
extern crate shader_version;
extern crate input;
extern crate event_loop;
extern crate graphics;
extern crate window;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate freetype;
extern crate threadpool;
extern crate rustc_serialize;
extern crate docopt;
extern crate num_cpus;

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

pub mod chapter3 {
    pub mod entry;
}

use opengl_graphics::{GlGraphics};
use sdl2_window::{Sdl2Window};
use window::{WindowSettings, Size};
use event_loop::{Events};
use docopt::{Docopt};
use graphics::{color};
use freetype::{Face};
use input::{Event};

use std::path::{Path};

use util::config::{Config};

use chapter2::entry::{chapter2_entry};
use chapter3::entry::{chapter3_entry};

static USAGE: &'static str = "
Usage: pcg <chapter>
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_chapter: String,
}

type ChapterCallback = Box<Fn(&Config) -> Box<Fn(&mut GlGraphics, &mut Face, Event) -> ()>>;

fn main() {

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    let (chapter_config, chapter_callback): (&str, ChapterCallback) = match &args.arg_chapter[..] {
        "chapter2" => ("src/chapter2/chapter2.toml", box chapter2_entry),
        "chapter3" => ("src/chapter3/chapter3.toml", box chapter3_entry),
        _ => panic!("Could not find chapter.")
    };

    let config_path = Path::new(chapter_config);
    let config = Config::new(config_path);
    let vars = config.get_table(None, "main");
    let window_width = config.get_default(vars, "window_width", 800);
    let window_height = config.get_default(vars, "window_height", 800);
    let font_name = config.get_string(vars, "font");
    let font_size = config.get_default(vars, "font_size", 14);
    let fps = config.get_default(vars, "fps", 10);

    let opengl = shader_version::OpenGL::V3_2;
    let size = Size { width: window_width, height: window_height };
    let settings = WindowSettings::new("PCG", (window_width, window_height))
        .exit_on_esc(true)
        .opengl(opengl);
        //.build()
        //.unwrap();
    let window = Sdl2Window::new(settings).unwrap();
    let ref mut gl = GlGraphics::new(opengl);
    let ft = freetype::Library::init().unwrap();
    let font = Path::new(font_name);
    let mut face = ft.new_face(&font, 0).unwrap();
    face.set_pixel_sizes(0, font_size).unwrap();

    let cb = chapter_callback(&config);
    // TODO fps
    for e in window.events() {
        graphics::clear(color::BLACK, gl);
        cb(gl, &mut face, e);
    }
}
