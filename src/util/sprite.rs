use opengl_graphics::{Gl, Texture};
use graphics::{Image, Line, default_draw_state};
use viewport::{Viewport};
use std::rc::Rc;

/// sprites can have several images (they must be the same height/width).
pub struct Sprite {
    texture: Rc<Texture>,
    images: Vec<Image>,
    height: i32,
    width: i32,
}

impl Sprite {

    pub fn new(texture: Rc<Texture>, images: Vec<Image>, height: i32, width: i32) -> Sprite {
        Sprite {
            texture: texture,
            images: images,
            height: height,
            width: width,
        }
    }

    pub fn draw(&self, gl: &mut Gl, viewport: Viewport, x: i32, y: i32, index: usize) {
        //let viewport = [x, y, self.width, self.height];
        let idx = index % self.images.len();
        let image = self.images[idx];
        gl.draw(viewport, |c, gl| {
            image.draw(&*self.texture, default_draw_state(), c.transform, gl);
        });
    }

    /// draw a red 'X'.
    pub fn missing(gl: &mut Gl, viewport: Viewport, x: i32, y: i32, w: i32, h: i32) {
        let line = Line::new([1.0, 0.0, 0.0, 1.0], 1.0);
        //gl.draw([x, y, w, h], |c, gl| {
        gl.draw(viewport, |c, gl| {
            line.draw([0.0, 0.0, w as f64, h as f64], default_draw_state(), c.transform, gl);
            line.draw([w as f64, 0.0, 0.0, h as f64], default_draw_state(), c.transform, gl);
        });
    }
}

/// the sprite "area" on the texture.
#[derive(Clone, Debug)]
pub struct SpriteRect {
    h: i32,
    w: i32,
    x: i32,
    y: i32
}

impl SpriteRect {

    pub fn new(x: i32, y: i32, w: i32, h: i32) -> SpriteRect {
        SpriteRect { x: x, y: y, w: w, h: h }
    }

    pub fn get_width(&self) -> i32 { self.w }

    pub fn get_height(&self) -> i32 { self.h }

    pub fn to_image(&self) -> Image {
        Image {
            color: None,
            rectangle: None,
            source_rectangle: Some([self.x * self.w,
                                    self.y * self.h,
                                    self.x * self.w + self.w * 64,
                                    self.y * self.h + self.h * 64])
        }
    }
}
