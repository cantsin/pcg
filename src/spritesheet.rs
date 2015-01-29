use std::io::{File};
use std::io::fs::{PathExtensions};
use std::collections::{HashMap};
use std::str::{FromStr};
use opengl_graphics::{Texture};
use graphics::{Image};
use toml::{Parser, Value, Table};

pub struct SpriteSheet {
    pub texture: Texture,
    pub sprites: HashMap<String, SpriteCategory>
}

/// sprite categories are equivalent to TOML blocks.
#[derive(Clone, Debug)]
enum SpriteCategory {
    Unique(HashMap<String, SpriteRect>),
    Sequence(Vec<SpriteRect>)
}

/// the sprite "area" on the texture.
#[derive(Clone, Debug)]
struct SpriteRect {
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
    /// spritesheet information. returns a hashmap of `Sprite`s.
    fn process(toml_path: &Path) -> HashMap<String, SpriteCategory> {
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
                let mut sprites = HashMap::new();
                for (name, values) in attributes {
                    // look for a local tile_width or tile_height
                    let tile_width = TomlConfig::defaults(values, "tile_width", tile_width);
                    let tile_height = TomlConfig::defaults(values, "tile_height", tile_height);
                    let sprite = TomlConfig::process_sprite(name, values, tile_width as u32, tile_height as u32);
                    sprites.insert(name.clone(), sprite);
                }
                sprites
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
    fn process_sprite(name: &String, ids: &Table, w: u32, h: u32) -> SpriteCategory {
        let info = TomlConfig::to_coords(ids);
        if info.len() == 0 {
            println!("Warning: `{}` has no valid sprite information.", name);
            SpriteCategory::Sequence(vec![])
        } else {
            let first = info.keys().next().unwrap();
            let seq: Option<u32> = FromStr::from_str(first.as_slice());
            match seq {
                Some(_) => {
                    // TODO: order by id.
                    let seq = info.iter().map(|(k, &(x, y))| {
                        let id: u32 = FromStr::from_str(k.as_slice()).unwrap();
                        let rect = SpriteRect { x: x, y: y, w: w, h: h };
                        rect
                        }).collect();
                    SpriteCategory::Sequence(seq)
                },
                _ => {
                    let map = info.iter().map(|(&k, &(x, y))| {
                        let rect = SpriteRect { x: x, y: y, w: w, h: h };
                        (k.clone(), rect)
                    }).collect();
                    SpriteCategory::Unique(map)
                }
            }
        }
    }
}

impl SpriteSheet {

    pub fn new(path: &str, tile_width: u32, tile_height: u32) -> SpriteSheet {
        let filepath = Path::new(path);
        let texture = Texture::from_path(&filepath).unwrap();
        let toml_filepath = TomlConfig::location(&filepath).expect("No spritesheet configuration file.");
        let sprites = TomlConfig::process(&toml_filepath);
        SpriteSheet {
            texture: texture,
            sprites: sprites
        }
    }

    pub fn get_sprite(&self, name: &str) -> Option<Image> {
        None
        // TODO: implement.
        // match self.mapping.get(name) {
        //     Some(&ref val) => {
        //         let coords = &val.as_slice().expect("coords");
        //         let ref x = coords[0].as_integer().expect("x") as i32;
        //         let ref y = coords[1].as_integer().expect("y") as i32;
        //         let w = self.tile_width as i32;
        //         let h = self.tile_height as i32;
        //         Some(Image {
        //             color: None,
        //             rectangle: None,
        //             source_rectangle: Some([x * w, y * h, w, h])
        //         })
        //     }
        //     _ => None
        // }
    }
}
