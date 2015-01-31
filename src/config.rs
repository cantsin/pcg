use std::io::{File};
use std::str::{FromStr};
use std::slice::{SliceExt};
use std::collections::{HashMap, BTreeMap};
use toml::{Parser, Value, Table};

use sprite::{SpriteRect};

const DEFAULT_TILE_SIZE: i64 = 16;

type Coords = (i32, i32);

pub struct TomlConfig;

impl TomlConfig {

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

    fn get_coords(name: &String, table: &Table) -> Vec<Coords> {
        let mut coords = vec![];
        for (name, value) in table.iter() {
            match value.type_str() {
                "array" => {
                    let values = value.as_slice().unwrap();
                    let (x, y) = TomlConfig::get_coord(name, values);
                    coords.push((x, y));
                }
                _ => {} // ignore
            }
        }
        coords
    }

    /// given a TOML configuration file, extract the relevant
    /// spritesheet information. returns a hashmap of `Sprite`s.
    pub fn process_spritesheet(toml_path: &Path) -> HashMap<String, Vec<SpriteRect>> {
        let mut toml_file = File::open(toml_path);
        match toml_file.read_to_string() {
            Err(why) => panic!("Could not read configuration file {}: {}", toml_path.display(), why),
            Ok(contents) => {
                let value = Parser::new(contents.as_slice()).parse().expect("Configuration file is not valid TOML.");
                let sprites = value.get("sprites").expect("Configuration file does not have `sprites` entry.");
                let sprites_table = sprites.as_table().expect("`sprites` entry is not a TOML table.");
                let tile_width = TomlConfig::defaults(sprites_table, "tile_width", DEFAULT_TILE_SIZE);
                let tile_height = TomlConfig::defaults(sprites_table, "tile_height", DEFAULT_TILE_SIZE);
                let mut sprites = HashMap::new();
                for (name, value) in sprites_table.iter() {
                    match value.type_str() {
                        "array" => {
                            let values = value.as_slice().unwrap();
                            let (x, y) = TomlConfig::get_coord(name, values);
                            let rect = SpriteRect::new(x, y, tile_width as i32, tile_height as i32);
                            sprites.insert(name.clone(), vec![rect]);
                        }
                        "table" => {
                            let table = value.as_table().unwrap();
                            // look for a local tile_width or tile_height
                            let tile_width = TomlConfig::defaults(table, "tile_width", tile_width);
                            let tile_height = TomlConfig::defaults(table, "tile_height", tile_height);
                            let coords = TomlConfig::get_coords(name, table);
                            let rects = coords.iter().map(|&(x, y)| {
                                SpriteRect::new(x, y, tile_width as i32, tile_height as i32)
                            }).collect();
                            sprites.insert(name.clone(), rects);
                        }
                        _ => panic!("unknown TOML type {:?}", name)
                    }
                }
                sprites
            }
        }
    }
}
