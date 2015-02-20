use freetype;
use opengl_graphics::{Gl, Texture};
use graphics::{RelativeTransform, Image, color};

pub fn render_text(face: &mut freetype::Face, gl: &mut Gl, x: i32, y: i32, text: &str) {
    gl.draw([0, 0, 400, 416], |c, gl| {
        let mut x = x;
        let mut y = y;
        for ch in text.chars() {
            face.load_char(ch as usize, freetype::face::RENDER).unwrap();
            let g = face.glyph();
            let bitmap = g.bitmap();
            let texture = Texture::from_memory_alpha(bitmap.buffer(),
                                                     bitmap.width() as u32,
                                                     bitmap.rows() as u32).unwrap();
            Image::colored(color::WHITE).draw(&texture,
                                              &c.trans((x + g.bitmap_left()) as f64, (y - g.bitmap_top()) as f64),
                                              gl);
            x += (g.advance().x >> 6) as i32;
            y += (g.advance().y >> 6) as i32;
        }
    });
}
