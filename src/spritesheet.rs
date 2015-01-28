
use std::collections::{HashMap};
use opengl_graphics::{Texture};
use graphics::{Image};

pub struct SpriteSheet<'a> {
    pub path: &'a str,
    pub texture: Texture,
    pub tile_width: u32,
    pub tile_height: u32,
    pub mapping: HashMap<&'a str,(i32,i32)>
}

impl SpriteSheet<'static> {

    pub fn new(path: &str, tile_width: u32, tile_height: u32) -> SpriteSheet {
        let filepath = Path::new(path);
        let texture = Texture::from_path(&filepath).unwrap();

        // TODO. toml?
        let mut texmap = HashMap::new();
        texmap.insert("Floor"      , (3, 6));
        texmap.insert("Wall"       , (2, 3));
        texmap.insert("Entrance"   , (2, 4));
        texmap.insert("Exit"       , (2, 5));
        texmap.insert("Door"       , (1, 2));
        texmap.insert("Monster"    , (23,1));
        texmap.insert("Treasure"   , (9, 5));
        texmap.insert("Trap"       , (2, 9));
        texmap.insert("Teleporter" , (7, 5));

        SpriteSheet {
            path: path,
            texture: texture,
            tile_width: tile_width,
            tile_height: tile_height,
            mapping: texmap
        }
    }

    pub fn get_sprite(&self, name: &str) -> Option<Image> {
        match self.mapping.get(name) {
            Some(&(x, y)) => {
                Some(Image {
                    color: None,
                    rectangle: None,
                    source_rectangle: Some([x, y, self.tile_width as i32, self.tile_height as i32])
                })
            }
            _ => None
        }
    }
}
