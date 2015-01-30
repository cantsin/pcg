use std::collections::{HashMap};
use opengl_graphics::{Gl, Texture};
use graphics::{Image};

struct Sprite<'a> {
    texture: &'a Texture,
    images: Vec<Image>,
    height: i32,
    width: i32,
    index: usize
}

impl Sprite<'static> {

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.images.len();
    }

    pub fn draw(&self, mut gl: Gl, x: i32, y: i32) {
        let viewport = [x, y, self.width, self.height];
        let image = self.images[self.index];
        gl.draw(viewport, |c, gl| {
            image.draw(self.texture, &c, gl);
        });
    }
}

/// sprite categories are equivalent to TOML blocks.
#[derive(Clone, Debug)]
pub enum SpriteCategory {
    Unique(HashMap<String, SpriteRect>),
    Sequence(Vec<SpriteRect>)
}

impl SpriteCategory {
    pub fn to_unique(&self) -> HashMap<String, SpriteRect> {
        match self {
            &SpriteCategory::Unique(ref hm) => hm.clone(),
            _ => panic!("not an unique sprite")
        }
    }

    pub fn to_sequence(&self) -> Vec<SpriteRect> {
        match self {
            &SpriteCategory::Sequence(ref v) => v.clone(),
            _ => panic!("not a sequenced sprite")
        }
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
