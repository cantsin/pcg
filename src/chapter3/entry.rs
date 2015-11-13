use freetype::{Face};
use opengl_graphics::{GlGraphics};
use input::{Event, RenderEvent};

use util::text::{render_text};
use util::config::{Config};

pub fn chapter3_entry(config: &Config) -> Box<Fn(&mut GlGraphics, &mut Face, Event) -> ()> {

    box move |gl: &mut GlGraphics, face: &mut Face, e: Event| {

        if let Some(args) = e.render_args() {
            let info = format!("chapter3 stub");
            render_text(face, gl, args.viewport(), 10.0, 410.0, &info[..]);
        };
    }
}
