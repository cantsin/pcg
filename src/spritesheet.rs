use std::collections::{BTreeMap};
use opengl_graphics::{Texture};
use graphics::{Image};
use toml::{Parser, Value};
use std::io::{File};

pub struct SpriteSheet<'a> {
    pub texture: Texture,
    pub tile_width: u32,
    pub tile_height: u32,
    /// each spritesheet must have a corresponding toml file that
    /// allows us to retrieve sprite tiles by name.
    pub mapping: BTreeMap<String, Value>
}

impl SpriteSheet<'static> {

    pub fn new(path: &str, tile_width: u32, tile_height: u32) -> SpriteSheet {
        let filepath = Path::new(path);
        let texture = Texture::from_path(&filepath).unwrap();

        // obtain the mapping from the corresponding toml file.
        let mut toml_filepath = filepath.clone();
        toml_filepath.set_extension("toml");
        let mut toml_file = File::open(&toml_filepath);
        let contents = String::from_utf8(toml_file.read_to_end().unwrap()).unwrap();
        let value = Parser::new(contents.as_slice()).parse().unwrap();
        let spritesheet = value.get("spritesheet").unwrap();
        let mapping = spritesheet.as_table().unwrap();

        // TODO pre-process toml.

        SpriteSheet {
            texture: texture,
            tile_width: tile_width,
            tile_height: tile_height,
            mapping: mapping.clone()
        }
    }

    pub fn get_sprite(&self, name: &str) -> Option<Image> {
        match self.mapping.get(name) {
            Some(&ref val) => {
                let coords = &val.as_slice().unwrap();
                let ref x = coords[0].as_integer().unwrap() as i32;
                let ref y = coords[1].as_integer().unwrap() as i32;
                let w = self.tile_width as i32;
                let h = self.tile_height as i32;
                Some(Image {
                    color: None,
                    rectangle: None,
                    source_rectangle: Some([x * w, y * h, w, h])
                })
            }
            _ => None
        }
    }
}
