use std::io::fs::{PathExtensions};
use std::slice::{SliceExt};
use std::collections::{HashMap};
use std::rc::Rc;
use opengl_graphics::{Texture};

use config::{TomlConfig};
use sprite::{Sprite};

pub struct SpriteSheet {
    pub sprites: HashMap<String, Sprite>
}

impl SpriteSheet {

    /// the spritesheet configuration file must have the same base
    /// file name as the spritesheet itself.
    fn location(path: &Path) -> Option<Path> {
        let mut new_path = path.clone();
        new_path.set_extension("toml");
        if new_path.exists() && new_path.is_file() {
            Some(new_path)
        } else {
            None
        }
    }

    pub fn new(filepath: &Path) -> SpriteSheet {
        let texture_data = Texture::from_path(filepath).unwrap();
        let texture = Rc::new(texture_data);
        let toml_path = SpriteSheet::location(filepath).expect("No spritesheet configuration file.");
        let config = TomlConfig::process_spritesheet(&toml_path);
        // convert all the coordinates to sprites on this texture
        let sprites = config.iter().map(|(name, &ref rects)| {
            let height = rects[0].get_height();
            let width = rects[0].get_width();
            let images = rects.iter().map(|&ref v| v.to_image()).collect();
            let sprite = Sprite::new(texture.clone(), images, height, width);
            (name.clone(), sprite)
        }).collect();
        SpriteSheet {
            sprites: sprites
        }
    }
}
