use freetype::{Face};
use freetype::face::{RENDER};
use opengl_graphics::{Gl, Texture};
use graphics::{RelativeTransform, Image, color, default_draw_state};

pub fn render_text(face: &mut Face, gl: &mut Gl, xcoord: f64, ycoord: f64, text: &str) {
    // TODO: account for window size correctly
    gl.draw([0, 0, 400, 416], |c, gl| {
        let transform = c.transform.trans(xcoord, ycoord);
        let mut x = 0;
        let mut y = 0;
        for ch in text.chars() {
            face.load_char(ch as usize, RENDER).unwrap();
            let g = face.glyph();
            let bitmap = g.bitmap();
            let texture = Texture::from_memory_alpha(bitmap.buffer(),
                                                     bitmap.width() as u32,
                                                     bitmap.rows() as u32).unwrap();
            Image::colored(color::WHITE).draw(&texture,
                                              default_draw_state(),
                                              transform.trans((x + g.bitmap_left()) as f64, (y - g.bitmap_top()) as f64),
                                              gl);
            x += (g.advance().x >> 6) as i32;
            y += (g.advance().y >> 6) as i32;
        }
    });
}
