use std::path::{Path, PathBuf};
use std::io::{Read};
use std::fs::{File};
use std::slice::{SliceExt};
use std::collections::{HashMap, HashSet};
use toml::{Parser, Value, Table, decode};
use rustc_serialize::{Decodable};

use sprite::{SpriteRect};
use util::{Coords};

pub struct Config {
    content: Table
}

impl Config {
    pub fn new(config_path: &Path) -> Config {
        let mut config_file = File::open(config_path).unwrap();
        let mut contents = String::new();
        match config_file.read_to_string(&mut contents) {
            Err(why) => panic!("Could not read configuration file {}: {}", config_path.display(), why),
            _ => ()
        };
        let value = Parser::new(contents.as_slice()).parse().expect("Configuration file is not valid TOML.");
        Config {
            content: value
        }
    }

    pub fn get_table<'a>(&'a self, table: Option<&'a Table>, name: &str) -> &Table {
        let lookup = table.unwrap_or(&self.content);
        let value = lookup.get(name).expect(format!("`{}` was not found.", name).as_slice());
        value.as_table().expect(format!("`{}` is not a TOML.", name).as_slice())
    }

    pub fn get_listing(&self, table: &Table, excluded: Vec<&str>) -> Vec<String> {
        let invalid: HashSet<&str> = excluded.into_iter().collect();
        table.keys().cloned().filter(|k| !invalid.contains(k.as_slice())).collect()
    }

    pub fn get_integer<'a>(&'a self, table: &'a Table, name: &str) -> i64 {
        let value = table.get(name).expect(format!("`{}` was not found.", name).as_slice());
        value.as_integer().expect(format!("`{}` is not an integer.", name).as_slice())
    }

    pub fn get_char<'a>(&'a self, table: &'a Table, name: &str) -> char {
        let value = table.get(name).expect(format!("`{}` was not found.", name).as_slice());
        let contents = value.as_str().expect(format!("`{}` is not a char.", name).as_slice());
        if contents.len() != 1 {
            panic!("{} is a string, but expected a character.", name);
        }
        contents.chars().next().unwrap()
    }

    pub fn get_string<'a>(&'a self, table: &'a Table, name: &str) -> &str {
        let value = table.get(name).expect(format!("`{}` was not found.", name).as_slice());
        value.as_str().expect(format!("`{}` is not a string.", name).as_slice())
    }

    pub fn get_array<T: Decodable>(&self, table: &Table, name: &str) -> Vec<T> {
        let value = table.get(name).expect(format!("`{}` was not found.", name).as_slice());
        let arr = value.as_slice().expect(format!("`{}` is not an array.", name).as_slice());
        arr.iter().map(|v| decode(v.clone()).unwrap()).collect()
    }

    pub fn get_default<T: Decodable>(&self, table: &Table, name: &str, val: T) -> T {
        match table.get(name) {
            Some(value) => decode(value.clone()).unwrap_or(val),
            None => val
        }
    }
}

const DEFAULT_TILE_SIZE: i64 = 16;

pub struct SpriteConfig;

impl SpriteConfig {

    /// override the default value if the given table location exists.
    fn defaults(what: &Table, attribute: &str, default: i64) -> i64 {
        let wrapped_default = Value::Integer(default);
        let attr = what.get(attribute).unwrap_or(&wrapped_default);
        attr.as_integer().expect(format!("`{}` must be an integer", attribute).as_slice())
    }

    /// helper function to obtain sprite coordinates
    fn get_coord(name: &String, values: &[Value]) -> Coords {
        if values.len() != 2 {
            panic!("attribute {:?} has too many values.", name);
        }
        let x = values[0].as_integer().expect("`x` must be an integer") as i32;
        let y = values[1].as_integer().expect("`y` must be an integer") as i32;
        (x, y)
    }

    fn get_coords(table: &Table) -> Vec<Coords> {
        let mut coords = vec![];
        for (name, value) in table.iter() {
            match value.type_str() {
                "array" => {
                    let values = value.as_slice().unwrap();
                    let (x, y) = SpriteConfig::get_coord(name, values);
                    coords.push((x, y));
                }
                _ => {
                    match name.as_slice() {
                        "tile_width" | "tile_height" => {} // ignore
                        _ => panic!("unknown TOML type {:?}", name)
                    }
                }
            }
        }
        coords
    }

    /// given a TOML configuration file, extract the relevant
    /// spritesheet information. returns a hashmap of `Sprite`s.
    pub fn process_spritesheet(toml_path: &PathBuf) -> HashMap<String, Vec<SpriteRect>> {
        let mut toml_file = File::open(toml_path).unwrap();
        let mut contents = String::new();
        match toml_file.read_to_string(&mut contents) {
            Err(why) => panic!("Could not read configuration file {}: {}", toml_path.display(), why),
            _ => ()
        };
        let value = Parser::new(contents.as_slice()).parse().expect("Configuration file is not valid TOML.");
        let sprites = value.get("sprites").expect("Configuration file does not have `sprites` entry.");
        let sprites_table = sprites.as_table().expect("`sprites` entry is not a TOML table.");
        let tile_width = SpriteConfig::defaults(sprites_table, "tile_width", DEFAULT_TILE_SIZE);
        let tile_height = SpriteConfig::defaults(sprites_table, "tile_height", DEFAULT_TILE_SIZE);
        let mut sprites = HashMap::new();
        for (name, value) in sprites_table.iter() {
            match value.type_str() {
                "array" => {
                    let values = value.as_slice().unwrap();
                    let (x, y) = SpriteConfig::get_coord(name, values);
                    let rect = SpriteRect::new(x, y, tile_width as i32, tile_height as i32);
                    sprites.insert(name.clone(), vec![rect]);
                }
                "table" => {
                    let table = value.as_table().unwrap();
                    // look for a local tile_width or tile_height
                    let tile_width = SpriteConfig::defaults(table, "tile_width", tile_width);
                    let tile_height = SpriteConfig::defaults(table, "tile_height", tile_height);
                    let coords = SpriteConfig::get_coords(table);
                    let rects = coords.iter().map(|&(x, y)| {
                        SpriteRect::new(x, y, tile_width as i32, tile_height as i32)
                    }).collect();
                    sprites.insert(name.clone(), rects);
                }
                _ => {
                    match name.as_slice() {
                        "tile_width" | "tile_height" => {} // ignore
                        _ => panic!("unknown TOML type {:?}", name)
                    }
                }
            }
        }
        sprites
    }
}
