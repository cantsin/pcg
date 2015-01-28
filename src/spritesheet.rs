use std::io::fs::{PathExtensions};
use std::io::{File};
use std::collections::{BTreeMap, HashMap};
use std::collections::btree_map::{Keys};
use std::str::FromStr;
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
    Unique(String, String),
    Sequence(String, u32)
}

struct Sprite {
    t: SpriteType,
    h: u32,
    w: u32,
    x: u32,
    y: u32
}

const default_tile_size: i64 = 16;

type Coords = (u32, u32);

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

    /// override the default value if the given table location exists.
    fn defaults(what: &Table, attribute: &str, default: i64) -> i64 {
        let wrapped_default = Value::Integer(default);
        let attr = what.get(attribute).unwrap_or(&wrapped_default);
        attr.as_integer().expect(format!("`{}` must be an integer", attribute).as_slice())
    }

    /// given a TOML configuration file, extract the relevant
    /// spritesheet information. returns a vector of `Sprite`s.
    fn process(toml_path: &Path) {
        let mut toml_file = File::open(toml_path);
        match toml_file.read_to_string() {
            Err(why) => panic!("Could not read configuration file {}: {}", toml_path.display(), why),
            Ok(contents) => {
                let value = Parser::new(contents.as_slice()).parse().expect("Configuration file is not valid TOML.");
                let sprites = value.get("sprites").expect("Configuration file does not have `sprites` entry.");
                let sprites_table = sprites.as_table().expect("`sprites` entry is not a TOML table.");
                let tile_width = TomlConfig::defaults(sprites_table, "tile_width", default_tile_size);
                let tile_height = TomlConfig::defaults(sprites_table, "tile_height", default_tile_size);
                let mut attributes = sprites_table.iter()
                    .filter(|&(_, v)| v.type_str() == "table")
                    .map(|(k, v)| (k, v.as_table().unwrap()));
                for (name, values) in attributes {
                    // look for a local tile_width or tile_height
                    let tile_width = TomlConfig::defaults(values, "tile_width", tile_width);
                    let tile_height = TomlConfig::defaults(values, "tile_height", tile_height);
                    TomlConfig::process_sprite(name, values, tile_width as u32, tile_height as u32);
                }
            }
        }
    }

    /// auxiliary function. given a TOML table, convert it to a
    /// straight up HashMap of tile_name, coords.
    fn to_coords(what: &Table) -> HashMap<&String, Coords> {
        let mut result: HashMap<&String, Coords> = HashMap::new();
        let mut filtered = what
            .iter()
            .filter(|&(_, v)| v.type_str() == "array")
            .map(|(k, v)| (k, v.as_slice().unwrap()));
        for (k, v) in filtered {
            if v.len() != 2 {
                panic!("attribute {:?} is not a coordinate.", v);
            }
            let x = v[0].as_integer().expect("`x` must be an integer") as u32;
            let y = v[1].as_integer().expect("`y` must be an integer") as u32;
            result.insert(k, (x, y));
        }
        result
    }

    // determine what kind of sprite we have.
    fn process_sprite(name: &String, ids: &Table, w: u32, h: u32) -> Vec<Sprite> {
        let info = TomlConfig::to_coords(ids);
        if info.len() == 0 {
            println!("Warning: `{}` has no valid sprite information.", name);
            vec![]
        } else {
            let first = info.keys().next().unwrap();
            let seq: Option<u32> = FromStr::from_str(first.as_slice());
            match seq {
                Some(_) => {
                    info.iter().map(|(k, &(x, y))| {
                        let id = FromStr::from_str(k.as_slice()).unwrap();
                        Sprite { t: SpriteType::Sequence(name.clone(), id), x: x, y: y, w: w, h: h }
                    }).collect()
                },
                _ => {
                    info.iter().map(|(&k, &(x, y))| {
                        Sprite { t: SpriteType::Unique(name.clone(), k.clone()), x: x, y: y, w: w, h: h }
                    }).collect()
                }
            }
        }
    }
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
