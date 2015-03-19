use freetype;
use opengl_graphics::{Gl, Texture};
use graphics::{RelativeTransform, Image, color, default_draw_state};
use graphics::vecmath::{Matrix2d};

pub fn render_text(face: &mut freetype::Face, gl: &mut Gl, t: Matrix2d, text: &str) {
    let mut x = 0;
    let mut y = 0;
    for ch in text.chars() {
        face.load_char(ch as usize, freetype::face::RENDER).unwrap();
        let g = face.glyph();
        let bitmap = g.bitmap();
        let texture = Texture::from_memory_alpha(bitmap.buffer(),
                                                 bitmap.width() as u32,
                                                 bitmap.rows() as u32).unwrap();
        Image::colored(color::WHITE).draw(&texture,
                                          default_draw_state(),
                                          t.trans((x + g.bitmap_left()) as f64, (y - g.bitmap_top()) as f64),
                                          gl);
        x += (g.advance().x >> 6) as i32;
        y += (g.advance().y >> 6) as i32;
    }
}
