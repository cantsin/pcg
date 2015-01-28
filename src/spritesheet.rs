use std::io::fs::{PathExtensions};
use std::io::{File};
use std::collections::{BTreeMap};
use std::collections::btree_map::{Keys};
use opengl_graphics::{Texture};
use graphics::{Image};
use toml::{Parser, Value, Table};

/// Sprite sheets must have an associated configuration file (in TOML).
pub struct SpriteSheet {
    pub texture: Texture,
    pub sprites: Vec<Sprite>,
    pub tile_width: u32,
    pub tile_height: u32,
    /// each spritesheet must have a corresponding toml file that
    /// allows us to retrieve sprite tiles by name.
    pub mapping: BTreeMap<String, Value>
}

enum SpriteType {
    Unique(String),
    Sequence(String, u32)
}

struct Sprite {
    t: SpriteType, // TODO index by?
    h: u32,
    w: u32,
    x: u32,
    y: u32
}

const default_tile_size: i64 = 16;

struct TomlConfig;

impl TomlConfig {

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

    /// override the default value from the given table location if it exists.
    fn defaults(what: &Table, attribute: &str, default: i64) -> i64 {
        let wrapped_default = Value::Integer(default);
        let attr = what.get(attribute).unwrap_or(&wrapped_default);
        attr.as_integer().expect(format!("`{}` must be an integer", attribute).as_slice())
    }

    fn process(toml_path: &Path) {
        let mut toml_file = File::open(toml_path);
        match toml_file.read_to_string() {
            Err(why) => panic!("Could not read {}: {}", toml_path.display(), why),
            Ok(contents) => {
                let value = Parser::new(contents.as_slice()).parse().expect("Configuration file is not valid TOML.");
                let sprites = value.get("sprites").expect("Configuration file does not have `sprites` entry.");
                let sprites_table = sprites.as_table().expect("`sprites` entry is not a TOML table.");
                let tile_width = TomlConfig::defaults(sprites_table, "tile_width", default_tile_size);
                let tile_height = TomlConfig::defaults(sprites_table, "tile_height", default_tile_size);
                let mut attributes = sprites_table.iter()
                    .filter(|&(_, v)| v.type_str() == "table")
                    .map(|(k, v)| (k, v.as_table().unwrap()));
                for (name, v) in attributes {
                    // look for a local tile_width or tile_height
                    let tile_width = TomlConfig::defaults(v, "tile_width", tile_width);
                    let tile_height = TomlConfig::defaults(v, "tile_height", tile_height);
                    let info = v.iter().filter(|&(_, v)| v.type_str() == "table").collect();
                    TomlConfig::process_sprite(name, info, tile_width, tile_height);
                }
            }
        }
    }

    fn process_sprite(name: &String, ids: &Table, w: i64, h: i64) {

        let test: Vec<String> = ids.keys().cloned().collect();

        println!("{:?}", test);
    }
    // table: either a sequence or array of names (account for local h/w though)
        // tile_width (local)
        // sequences
        // groups
}

impl SpriteSheet {

    pub fn new(path: &str, tile_width: u32, tile_height: u32) -> SpriteSheet {
        let filepath = Path::new(path);
        let texture = Texture::from_path(&filepath).unwrap();

        // obtain the mapping from the corresponding toml file.
        let toml_filepath = TomlConfig::location(&filepath).expect("No spritesheet configuration file.");
        TomlConfig::process(&toml_filepath);
        println!("passed parsing 1");

        let mut toml_file = File::open(&toml_filepath);
        let contents = String::from_utf8(toml_file.read_to_end().unwrap()).unwrap();
        let value = Parser::new(contents.as_slice()).parse().expect("parsing");
        let spritesheet = value.get("sprites").expect("No sprites entry?");
        let mapping = spritesheet.as_table().expect("No table value?");
        println!("passed parsing");

        // TODO pre-process toml.

        SpriteSheet {
            texture: texture,
            tile_width: tile_width,
            tile_height: tile_height,
            sprites: vec![],
            mapping: mapping.clone()
        }
    }

    pub fn get_sprite(&self, name: &str) -> Option<Image> {
        match self.mapping.get(name) {
            Some(&ref val) => {
                let coords = &val.as_slice().expect("coords");
                let ref x = coords[0].as_integer().expect("x") as i32;
                let ref y = coords[1].as_integer().expect("y") as i32;
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
