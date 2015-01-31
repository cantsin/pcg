use std::collections::{HashMap};
use opengl_graphics::{Gl, Texture};
use graphics::{Image};

/// sprites can have several images (they must be the same height/width).
pub struct Sprite<'a> {
    pub texture: &'a Texture,
    pub images: Vec<Image>,
    pub height: i32,
    pub width: i32,
    pub index: usize
}

impl Sprite<'static> {

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.images.len();
    }

    pub fn draw(&self, gl: &mut Gl, x: i32, y: i32) {
        let viewport = [x, y, self.width, self.height];
        let image = self.images[self.index];
        gl.draw(viewport, |c, gl| {
            image.draw(self.texture, &c, gl);
        });
    }
}

/// the sprite "area" on the texture.
#[derive(Clone, Debug)]
pub struct SpriteRect {
    pub h: i32,
    pub w: i32,
    x: i32,
    y: i32
}

impl SpriteRect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> SpriteRect {
        SpriteRect { x: x, y: y, w: w, h: h }
    }

    pub fn to_image(&self) -> Image {
        Image {
            color: None,
            rectangle: None,
            source_rectangle: Some([self.x * self.w,
                                    self.y * self.h,
                                    self.w,
                                    self.h])
        }
    }
}
