use std::collections::{HashMap};
use opengl_graphics::{Texture};
use graphics::{Image};

use config::{TomlConfig};
use sprite::{SpriteCategory};

pub struct SpriteSheet {
    pub texture: Texture,
    pub sprites: HashMap<String, SpriteCategory>
}

impl SpriteSheet {

    pub fn new(filepath: &Path) -> SpriteSheet {
        let texture = Texture::from_path(filepath).unwrap();
        let sprites = TomlConfig::process_spritesheet(filepath);
        SpriteSheet {
            texture: texture,
            sprites: sprites
        }
    }

    pub fn get_unique_sprite(&self, category: &str, name: &str) -> Option<Image> {
        match self.sprites.get(category) {
            Some(&SpriteCategory::Unique(ref result)) => {
                match result.get(name) {
                    Some(&ref sprite) => Some(sprite.to_image()),
                    _ => None
                }
            }
            _ => None
        }
    }

    pub fn get_sequenced_sprite(&self, category: &str, id: u32) -> Option<Image> {
        match self.sprites.get(category) {
            Some(&SpriteCategory::Sequence(ref result)) => {
                let n = id as usize % result.len();
                let sprite = result.get(n).unwrap();
                Some(sprite.to_image())
            }
            _ => None
        }
    }
}
