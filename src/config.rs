use std::io::{File};
use std::io::fs::{PathExtensions};
use std::str::{FromStr};
use std::slice::{SliceExt};
use std::collections::{HashMap, BTreeMap};
use toml::{Parser, Value, Table};

use sprite::{SpriteCategory, SpriteRect};

const DEFAULT_TILE_SIZE: i64 = 16;

type Coords = (i32, i32);

pub struct TomlConfig;

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

    /// override the default value if the given table location exists.
    fn defaults(what: &Table, attribute: &str, default: i64) -> i64 {
        let wrapped_default = Value::Integer(default);
        let attr = what.get(attribute).unwrap_or(&wrapped_default);
        attr.as_integer().expect(format!("`{}` must be an integer", attribute).as_slice())
    }

    /// given a TOML configuration file, extract the relevant
    /// spritesheet information. returns a hashmap of `Sprite`s.
    pub fn process_spritesheet(filepath: &Path) -> HashMap<String, SpriteCategory> {
        let toml_path = TomlConfig::location(filepath).expect("No spritesheet configuration file.");
        let mut toml_file = File::open(&toml_path);
        match toml_file.read_to_string() {
            Err(why) => panic!("Could not read configuration file {}: {}", toml_path.display(), why),
            Ok(contents) => {
                let value = Parser::new(contents.as_slice()).parse().expect("Configuration file is not valid TOML.");
                let sprites = value.get("sprites").expect("Configuration file does not have `sprites` entry.");
                let sprites_table = sprites.as_table().expect("`sprites` entry is not a TOML table.");
                let tile_width = TomlConfig::defaults(sprites_table, "tile_width", DEFAULT_TILE_SIZE);
                let tile_height = TomlConfig::defaults(sprites_table, "tile_height", DEFAULT_TILE_SIZE);
                let mut attributes = sprites_table.iter()
                    .filter(|&(_, v)| v.type_str() == "table")
                    .map(|(k, v)| (k, v.as_table().unwrap()));
                let mut sprites = HashMap::new();
                for (name, values) in attributes {
                    // look for a local tile_width or tile_height
                    let tile_width = TomlConfig::defaults(values, "tile_width", tile_width);
                    let tile_height = TomlConfig::defaults(values, "tile_height", tile_height);
                    let sprite = TomlConfig::process_sprite(name, values, tile_width as i32, tile_height as i32);
                    sprites.insert(name.clone(), sprite);
                }
                sprites
            }
        }
    }

    /// auxiliary function. flatten the table and convert appropriate
    /// entries to coordinates.
    fn to_coords(what: &Table) -> BTreeMap<String, Coords> {
        let mut result: BTreeMap<String, Coords> = BTreeMap::new();
        let mut filtered = what
            .iter()
            .filter(|&(_, v)| v.type_str() == "array")
            .map(|(k, v)| (k, v.as_slice().unwrap()));
        for (k, v) in filtered {
            if v.len() != 2 {
                panic!("attribute {:?} is not a coordinate.", v);
            }
            let x = v[0].as_integer().expect("`x` must be an integer") as i32;
            let y = v[1].as_integer().expect("`y` must be an integer") as i32;
            result.insert(k.clone(), (x, y));
        }
        result
    }

    // determine what kind of sprite we have.
    fn process_sprite(name: &String, ids: &Table, w: i32, h: i32) -> SpriteCategory {
        let info = TomlConfig::to_coords(ids);
        if info.len() == 0 {
            println!("Warning: `{}` has no valid sprite information.", name);
            SpriteCategory::Sequence(vec![])
        } else {
            let first = info.keys().next().unwrap();
            let seq: Option<i32> = FromStr::from_str(first.as_slice());
            match seq.is_some() {
                true => {
                    let seq2 = info.iter().map(|(_, &(x, y))| {
                        SpriteRect::new(x, y, w, h)
                    }).collect();
                    SpriteCategory::Sequence(seq2)
                },
                _ => {
                    let map = info.iter().map(|(k, &(x, y))| {
                        let rect = SpriteRect::new(x, y, w, h);
                        (k.clone(), rect)
                    }).collect();
                    SpriteCategory::Unique(map)
                }
            }
        }
    }
}
