use std::collections::{HashMap};
use opengl_graphics::{Gl, Texture};
use graphics::{Image};
use std::rc::Rc;

/// sprites can have several images (they must be the same height/width).
pub struct Sprite {
    texture: Rc<Texture>,
    images: Vec<Image>,
    height: i32,
    width: i32,
    index: usize
}

impl Sprite {

    pub fn new(texture: Rc<Texture>, images: Vec<Image>, height: i32, width: i32) -> Sprite {
        Sprite {
            texture: texture,
            images: images,
            height: height,
            width: width,
            index: 0
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.images.len();
    }

    pub fn draw(&self, gl: &mut Gl, x: i32, y: i32) {
        let viewport = [x, y, self.width, self.height];
        let image = self.images[self.index];
        gl.draw(viewport, |c, gl| {
            image.draw(&*self.texture, &c, gl);
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
                                    self.w,
                                    self.h])
        }
    }
}
