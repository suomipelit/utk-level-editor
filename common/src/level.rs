use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::collections::HashMap;

use crate::types::*;
use crate::util::*;

pub const TILE_SIZE: u32 = 20;

#[derive(Clone, Copy)]
pub enum CrateClass {
    Weapon = 0,
    Bullet = 1,
    Energy = 2,
}

impl CrateClass {
    pub fn from_u32(value: u32) -> CrateClass {
        match value {
            0 => CrateClass::Weapon,
            1 => CrateClass::Bullet,
            2 => CrateClass::Energy,
            _ => panic!("Unknown value: {}", value),
        }
    }
}
pub const ALL_CRATES: &[&str] = &[
    "pistol",
    "shotgun",
    "uzi",
    "auto rifle",
    "grenade launcher",
    "auto grenadier",
    "heavy launcher",
    "auto shotgun",
    "c4-activator",
    "flame thrower",
    "mine dropper",
    "9mm bullets (50)",
    "12mm bullets (50)",
    "shotgun shells (20)",
    "light grenades (15)",
    "medium grenades (10)",
    "heavy grenades (5)",
    "c4-explosives (5)",
    "gas (50)",
    "mines (5)",
    "energy",
];

pub fn weapon_crates() -> &'static [&'static str] {
    &ALL_CRATES[..=10]
}

pub fn bullet_crates() -> &'static [&'static str] {
    &ALL_CRATES[11..=19]
}

pub fn energy_crates() -> &'static [&'static str] {
    &ALL_CRATES[20..=20]
}

pub fn crates(cls: CrateClass) -> &'static [&'static str] {
    match cls {
        CrateClass::Weapon => weapon_crates(),
        CrateClass::Bullet => bullet_crates(),
        CrateClass::Energy => energy_crates(),
    }
}

const DIFF_WEAPONS: usize = 11;
const DIFF_BULLETS: usize = 9;
const DIFF_ENEMIES: usize = 8;

const VERSION: u32 = 5;

type Position = (u32, u32);

pub struct GeneralInfo {
    pub comment: String, // max 19 characters + \0 termination
    pub time_limit: u32,
    pub enemy_table: [u32; DIFF_ENEMIES as usize],
}

#[derive(Clone, Copy, Debug)]
pub struct Steam {
    pub range: u8,  // 0-6
    pub angle: u16, // 0-355 degress in 5 degree steps. 0 is downwards, direction counter clockwise.
}

pub struct CrateSet {
    pub weapons: [u32; DIFF_WEAPONS as usize],
    pub bullets: [u32; DIFF_BULLETS as usize],
    pub energy: u32,
}

pub struct RandomCrates {
    pub normal: CrateSet,
    pub deathmatch: CrateSet,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StaticCrate {
    Normal,
    Deathmatch,
}

#[derive(Clone, Copy)]
pub struct StaticCrateType {
    pub crate_variant: StaticCrate,
    pub crate_class: CrateClass,
    pub crate_type: u8,
}

pub struct Crates {
    pub random: RandomCrates,
    pub staticc: HashMap<Position, StaticCrateType>,
}

pub struct Level {
    pub tiles: Tiles,
    pub p1_position: Position,
    pub p2_position: Position,
    pub scroll: Position,
    pub spotlights: HashMap<Position, u8>, // 0-9 intensity
    pub steams: HashMap<Position, Steam>,
    pub general_info: GeneralInfo,
    pub crates: Crates,
}

#[derive(Debug)]
pub enum FileTypeError {
    InvalidVersion,
    InvalidLevelSize,
}

#[derive(Debug)]
pub enum DeserializationError {
    IOError(std::io::Error),
    ContentError(FileTypeError),
}

impl From<std::io::Error> for DeserializationError {
    fn from(e: std::io::Error) -> Self {
        DeserializationError::IOError(e)
    }
}

impl From<FileTypeError> for DeserializationError {
    fn from(e: FileTypeError) -> Self {
        DeserializationError::ContentError(e)
    }
}

impl Level {
    pub fn get_default_level(size: (u8, u8)) -> Level {
        let mut level = Level {
            tiles: Level::init_default_level(size),
            p1_position: (1, 1),
            p2_position: (1, 3),
            scroll: (0, 0),
            spotlights: HashMap::new(),
            steams: HashMap::new(),
            general_info: GeneralInfo {
                comment: "Rust UTK editor".to_string(),
                time_limit: 60,
                enemy_table: [1, 0, 0, 0, 0, 1, 0, 0],
            },
            crates: Crates {
                random: RandomCrates {
                    normal: CrateSet {
                        weapons: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        bullets: [1, 0, 0, 0, 0, 0, 0, 0, 0],
                        energy: 1,
                    },
                    deathmatch: CrateSet {
                        weapons: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        bullets: [1, 0, 0, 0, 0, 0, 0, 0, 0],
                        energy: 1,
                    },
                },
                staticc: HashMap::new(),
            },
        };
        level.create_shadows();
        level
    }

    fn init_default_level(size: (u8, u8)) -> Tiles {
        let level_size_x = size.0;
        let level_size_y = size.1;

        let mut tiles = Vec::new();

        // First row ...
        {
            let mut row = Vec::new();
            for x in 0..level_size_x {
                row.push(if x == 0 {
                    Tile {
                        texture_type: TextureType::Walls,
                        id: 0,
                        shadow: 0,
                    }
                } else if x == level_size_x - 1 {
                    Tile {
                        texture_type: TextureType::Walls,
                        id: 2,
                        shadow: 0,
                    }
                } else {
                    Tile {
                        texture_type: TextureType::Walls,
                        id: 1,
                        shadow: 0,
                    }
                });
            }
            tiles.push(row);
        }

        // .. all but final row ...
        for _y in 1..level_size_y - 1 {
            let mut row = Vec::new();

            for x in 0..level_size_x {
                row.push(if x == 0 || x == level_size_x - 1 {
                    Tile {
                        texture_type: TextureType::Walls,
                        id: 16,
                        shadow: 0,
                    }
                } else {
                    Tile {
                        texture_type: TextureType::Floor,
                        id: 0,
                        shadow: 0,
                    }
                });
            }
            tiles.push(row);
        }

        // ... and final row!
        {
            let mut row = Vec::new();
            for x in 0..level_size_x {
                row.push(if x == 0 {
                    Tile {
                        texture_type: TextureType::Walls,
                        id: 32,
                        shadow: 0,
                    }
                } else if x == level_size_x - 1 {
                    Tile {
                        texture_type: TextureType::Walls,
                        id: 18,
                        shadow: 0,
                    }
                } else {
                    Tile {
                        texture_type: TextureType::Walls,
                        id: 1,
                        shadow: 0,
                    }
                });
            }
            tiles.push(row);
        }
        tiles
    }

    fn get_tile_index(&self, pointed_tile: u32) -> (usize, usize) {
        (
            pointed_tile as usize % self.tiles[0].len(),
            pointed_tile as usize / self.tiles[0].len(),
        )
    }

    pub fn put_tile_to_level(
        &mut self,
        pointed_tile: u32,
        selected_tile_id: Option<u32>,
        selected_texture: &TextureType,
    ) {
        let (x, y) = self.get_tile_index(pointed_tile);
        if y < self.tiles.len() && x < self.tiles[0].len() {
            if *selected_texture != TextureType::Shadow {
                self.tiles[y][x] = Tile {
                    texture_type: *selected_texture,
                    id: selected_tile_id.unwrap(),
                    shadow: self.tiles[y][x].shadow,
                }
            } else {
                self.tiles[y][x].shadow = match selected_tile_id {
                    Some(id) => id + 1,
                    None => 0,
                };
            }
        }
    }

    pub fn put_spotlight_to_level(&mut self, level_coordinates: &Position, spotlight: u8) {
        if spotlight < 10 {
            self.spotlights.insert(*level_coordinates, spotlight);
        }
    }

    pub fn get_spotlight_from_level(&self, level_coordinates: &Position) -> u8 {
        *self.spotlights.get(level_coordinates).unwrap()
    }

    pub fn delete_spotlight_if_near(
        &mut self,
        level_coordinates: &Position,
        render_multiplier: u32,
    ) {
        let mut to_be_removed = Vec::new();
        {
            let distances: Vec<_> = self
                .spotlights
                .iter()
                .map(|(spotlight_coordinates, &spotlight)| {
                    let distance =
                        get_distance_between_points(level_coordinates, spotlight_coordinates);
                    (spotlight_coordinates, spotlight, distance)
                })
                .collect();
            for spotlight in distances {
                if get_spotlight_render_radius(&spotlight.1) as f64
                    >= spotlight.2 * render_multiplier as f64
                {
                    to_be_removed.push(*spotlight.0);
                }
            }
        }
        for key in to_be_removed {
            self.spotlights.remove(&key);
        }
    }

    pub fn put_steam_to_level(&mut self, level_coordinates: &Position, steam: &Steam) {
        if steam.range < 7 {
            self.steams.insert(*level_coordinates, *steam);
        }
    }

    pub fn get_steam_from_level(&self, level_coordinates: &Position) -> Steam {
        *self.steams.get(level_coordinates).unwrap()
    }

    pub fn delete_steam_if_near(&mut self, level_coordinates: &Position, render_multiplier: u32) {
        let mut to_be_removed = Vec::new();
        {
            let distances: Vec<_> = self
                .steams
                .iter()
                .map(|(steam_coordinates, &_steam)| {
                    let distance =
                        get_distance_between_points(level_coordinates, steam_coordinates);
                    (steam_coordinates, distance)
                })
                .collect();
            for steam in distances {
                if get_steam_render_radius() as f64 >= steam.1 * render_multiplier as f64 {
                    to_be_removed.push(*steam.0);
                }
            }
        }
        for key in to_be_removed {
            self.steams.remove(&key);
        }
    }

    pub fn put_crate_to_level(
        &mut self,
        level_coordinates: &Position,
        crate_item: &StaticCrateType,
    ) {
        self.crates.staticc.insert(*level_coordinates, *crate_item);
    }

    pub fn get_crate_from_level(&self, level_coordinates: &Position) -> &StaticCrateType {
        self.crates.staticc.get(level_coordinates).unwrap()
    }

    pub fn delete_crate_if_near(&mut self, level_coordinates: &Position, render_multiplier: u32) {
        let mut to_be_removed = Vec::new();
        for crate_coordinates in self.crates.staticc.keys() {
            if check_box_click(
                level_coordinates,
                crate_coordinates,
                get_crate_render_size() / render_multiplier,
            ) {
                to_be_removed.push(*crate_coordinates);
            }
        }
        for key in to_be_removed {
            self.crates.staticc.remove(&key);
        }
    }

    pub fn create_shadows(&mut self) {
        for y in (0..self.tiles.len()).rev() {
            for x in 0..self.tiles[y].len() {
                if self.tiles[y][x].texture_type != TextureType::Walls {
                    let on_right = if x < self.tiles[y].len() - 1 {
                        self.tiles[y][x + 1].texture_type
                    } else {
                        TextureType::Floor
                    };
                    let on_top_right = if y > 0 && x < self.tiles[y].len() - 1 {
                        self.tiles[y - 1][x + 1].texture_type
                    } else {
                        TextureType::Floor
                    };
                    let on_top = if y > 0 {
                        self.tiles[y - 1][x].texture_type
                    } else {
                        TextureType::Floor
                    };
                    self.tiles[y][x].shadow = if on_top_right == TextureType::Walls
                        || (on_right == TextureType::Walls && on_top == TextureType::Walls)
                    {
                        1
                    } else if on_top == TextureType::Walls {
                        3
                    } else if on_right == TextureType::Walls {
                        2
                    } else {
                        0
                    };
                } else {
                    self.tiles[y][x].shadow = 0;
                }
            }
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.write_u32::<LittleEndian>(VERSION)
            .expect("Failed to write version");
        data.write_u32::<LittleEndian>(self.tiles[0].len() as u32)
            .expect("Failed to write x size");
        data.write_u32::<LittleEndian>(self.tiles.len() as u32)
            .expect("Failed to write y size");
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[0].len() {
                data.write_u32::<LittleEndian>(self.tiles[y][x].texture_type as u32)
                    .expect("Failed to write block type");
                data.write_u32::<LittleEndian>(self.tiles[y][x].id)
                    .expect("Failed to write block num");
                data.write_u32::<LittleEndian>(self.tiles[y][x].shadow)
                    .expect("Failed to write block shadow");
            }
        }

        data.write_u32::<LittleEndian>(self.p1_position.0)
            .expect("Failed to write p1 start x");
        data.write_u32::<LittleEndian>(self.p1_position.1)
            .expect("Failed to write p1 start y");
        data.write_u32::<LittleEndian>(self.p2_position.0)
            .expect("Failed to write p2 start x");
        data.write_u32::<LittleEndian>(self.p2_position.1)
            .expect("Failed to write p2 start y");

        data.write_u32::<LittleEndian>(self.spotlights.len() as u32)
            .expect("Failed to write spot amount");

        for (coordinates, spotlight) in &self.spotlights {
            data.write_u32::<LittleEndian>(coordinates.0)
                .expect("Failed to write spotlight x position");
            data.write_u32::<LittleEndian>(coordinates.1)
                .expect("Failed to write spotlight y position");
            data.write_u32::<LittleEndian>(*spotlight as u32)
                .expect("Failed to write spotlight intensity");
        }

        data.write_u32::<LittleEndian>(self.steams.len() as u32)
            .expect("Failed to write steam amount");

        for (coordinates, steam) in &self.steams {
            data.write_u32::<LittleEndian>(coordinates.0)
                .expect("Failed to write steam x position");
            data.write_u32::<LittleEndian>(coordinates.1)
                .expect("Failed to write steam y position");
            data.write_u32::<LittleEndian>(steam.angle as u32)
                .expect("Failed to write steam angle");
            data.write_u32::<LittleEndian>(steam.range as u32)
                .expect("Failed to write steam range");
        }
        data.extend_from_slice(self.general_info.comment.as_bytes());
        for _ in 0..20 - self.general_info.comment.len() {
            data.write_u8(0).expect("Failed to write comment padding");
        }
        data.write_u32::<LittleEndian>(self.general_info.time_limit)
            .expect("Failed to write time limit");
        for enemy_amount in self.general_info.enemy_table {
            data.write_u32::<LittleEndian>(enemy_amount)
                .expect("Failed to write normal game enemies");
        }
        for weapon_amount in self.crates.random.normal.weapons {
            data.write_u32::<LittleEndian>(weapon_amount)
                .expect("Failed to write normal game weapons");
        }
        for bullet_amount in self.crates.random.normal.bullets {
            data.write_u32::<LittleEndian>(bullet_amount)
                .expect("Failed to write normal game bullets");
        }
        data.write_u32::<LittleEndian>(self.crates.random.normal.energy)
            .expect("Failed to write normal game energy crates");
        for weapon_amount in self.crates.random.deathmatch.weapons {
            data.write_u32::<LittleEndian>(weapon_amount)
                .expect("Failed to write deathmatch game weapons");
        }
        for bullet_amount in self.crates.random.deathmatch.bullets {
            data.write_u32::<LittleEndian>(bullet_amount)
                .expect("Failed to write deathmatch game bullets");
        }
        data.write_u32::<LittleEndian>(self.crates.random.deathmatch.energy)
            .expect("Failed to write deathmatch game energy crates");

        let normal_static_crates: HashMap<Position, StaticCrateType> = self
            .crates
            .staticc
            .clone()
            .into_iter()
            .filter(|(_coordinates, crate_item)| crate_item.crate_variant == StaticCrate::Normal)
            .collect();
        data.write_u32::<LittleEndian>(normal_static_crates.len() as u32)
            .expect("Failed to write normal game crate amount");
        for (coordinates, crate_item) in &normal_static_crates {
            data.write_u32::<LittleEndian>(crate_item.crate_class as u32)
                .expect("Failed to write normal game static crate class");
            data.write_u32::<LittleEndian>(crate_item.crate_type as u32)
                .expect("Failed to write normal game static crate type");
            data.write_u32::<LittleEndian>(coordinates.0)
                .expect("Failed to write normal game static crate x position");
            data.write_u32::<LittleEndian>(coordinates.1)
                .expect("Failed to write normal game static crate y position");
        }

        let deathmatch_static_crates: HashMap<Position, StaticCrateType> = self
            .crates
            .staticc
            .clone()
            .into_iter()
            .filter(|(_coordinates, crate_item)| {
                crate_item.crate_variant == StaticCrate::Deathmatch
            })
            .collect();
        data.write_u32::<LittleEndian>(deathmatch_static_crates.len() as u32)
            .expect("Failed to write deathmatch game crate amount");
        for (coordinates, crate_item) in &deathmatch_static_crates {
            data.write_u32::<LittleEndian>(crate_item.crate_class as u32)
                .expect("Failed to write deathmatch game static crate class");
            data.write_u32::<LittleEndian>(crate_item.crate_type as u32)
                .expect("Failed to write deathmatch game static crate type");
            data.write_u32::<LittleEndian>(coordinates.0)
                .expect("Failed to write deathmatch game static crate x position");
            data.write_u32::<LittleEndian>(coordinates.1)
                .expect("Failed to write deathmatch game static crate y position");
        }
        data
    }

    pub fn origo(&self, render_size: u32) -> (i32, i32) {
        (
            -((self.scroll.0 * render_size) as i32),
            -((self.scroll.1 * render_size) as i32),
        )
    }

    pub fn deserialize(&mut self, mut data: &[u8]) -> Result<(), DeserializationError> {
        self.scroll = (0, 0);
        self.spotlights.clear();
        self.steams.clear();
        self.general_info.comment = String::new();
        self.general_info.enemy_table.fill(0);
        self.crates.staticc = HashMap::new();
        self.crates.random.normal.weapons.fill(0);
        self.crates.random.normal.bullets.fill(0);
        self.crates.random.deathmatch.weapons.fill(0);
        self.crates.random.deathmatch.bullets.fill(0);

        let version: u32 = data.read_u32::<LittleEndian>()?;

        if version > VERSION {
            return Err(DeserializationError::ContentError(
                FileTypeError::InvalidVersion,
            ));
        }

        let x_size: u32 = data.read_u32::<LittleEndian>()?;
        if x_size < 1 {
            return Err(DeserializationError::ContentError(
                FileTypeError::InvalidLevelSize,
            ));
        }

        let y_size: u32 = data.read_u32::<LittleEndian>()?;
        if y_size < 1 {
            return Err(DeserializationError::ContentError(
                FileTypeError::InvalidLevelSize,
            ));
        }

        let mut tiles = Vec::new();
        for _ in 0..y_size {
            let mut row = Vec::new();
            for _ in 0..x_size {
                row.push(Tile {
                    texture_type: TextureType::from_u32(data.read_u32::<LittleEndian>()?),
                    id: data.read_u32::<LittleEndian>()?,
                    shadow: data.read_u32::<LittleEndian>()?,
                });
            }
            tiles.push(row);
        }
        self.tiles = tiles;

        self.p1_position.0 = data.read_u32::<LittleEndian>()?;
        self.p1_position.1 = data.read_u32::<LittleEndian>()?;
        self.p2_position.0 = data.read_u32::<LittleEndian>()?;
        self.p2_position.1 = data.read_u32::<LittleEndian>()?;

        let spotlight_amount = data.read_u32::<LittleEndian>()?;

        for _ in 0..spotlight_amount {
            let spotlight_x = data.read_u32::<LittleEndian>()?;
            let spotlight_y = data.read_u32::<LittleEndian>()?;
            self.spotlights.insert(
                (spotlight_x, spotlight_y),
                data.read_u32::<LittleEndian>()? as u8,
            );
        }

        let steam_amount = data.read_u32::<LittleEndian>()?;

        for _ in 0..steam_amount {
            let steam_x = data.read_u32::<LittleEndian>()?;
            let steam_y = data.read_u32::<LittleEndian>()?;
            self.steams.insert(
                (steam_x, steam_y),
                Steam {
                    angle: data.read_u32::<LittleEndian>()? as u16,
                    range: data.read_u32::<LittleEndian>()? as u8,
                },
            );
        }

        for _ in 0..20 {
            let c = data.read_u8()? as char;
            if c != '\0' {
                self.general_info.comment.push(c);
            }
        }

        self.general_info.time_limit = data.read_u32::<LittleEndian>()?;

        let number_of_enemy_types = if version >= 4 {
            DIFF_ENEMIES
        } else {
            DIFF_ENEMIES - 1
        } as usize;
        for enemy_number in 0..number_of_enemy_types {
            self.general_info.enemy_table[enemy_number] = data.read_u32::<LittleEndian>()?;
        }

        let number_of_weapons = if version == 1 {
            DIFF_WEAPONS - 2
        } else if version == 2 {
            DIFF_WEAPONS - 1
        } else {
            DIFF_WEAPONS
        } as usize;
        for weapon_number in 0..number_of_weapons {
            self.crates.random.normal.weapons[weapon_number] = data.read_u32::<LittleEndian>()?;
        }
        let number_of_bullets = if version == 1 {
            DIFF_BULLETS - 2
        } else if version == 2 {
            DIFF_BULLETS - 1
        } else {
            DIFF_BULLETS
        } as usize;
        for bullet_number in 0..number_of_bullets {
            self.crates.random.normal.bullets[bullet_number] = data.read_u32::<LittleEndian>()?;
        }
        self.crates.random.normal.energy = data.read_u32::<LittleEndian>()?;

        for weapon_number in 0..number_of_weapons {
            self.crates.random.deathmatch.weapons[weapon_number] =
                data.read_u32::<LittleEndian>()?;
        }
        for bullet_number in 0..number_of_bullets {
            self.crates.random.deathmatch.bullets[bullet_number] =
                data.read_u32::<LittleEndian>()?;
        }
        self.crates.random.deathmatch.energy = data.read_u32::<LittleEndian>()?;

        if version >= 5 {
            Level::deserialize_crates(data, &mut self.crates.staticc, StaticCrate::Normal)?;
            Level::deserialize_crates(data, &mut self.crates.staticc, StaticCrate::Deathmatch)?;
        }

        Ok(())
    }

    fn deserialize_crates(
        mut data: &[u8],
        crates: &mut HashMap<Position, StaticCrateType>,
        crate_variant: StaticCrate,
    ) -> Result<(), DeserializationError> {
        let number_of_crates = data.read_u32::<LittleEndian>()?;
        for _crate_index in 0..number_of_crates {
            let crate_item = StaticCrateType {
                crate_variant,
                crate_class: CrateClass::from_u32(data.read_u32::<LittleEndian>()?),
                crate_type: data.read_u32::<LittleEndian>()? as u8,
            };
            crates.insert(
                (
                    data.read_u32::<LittleEndian>()?,
                    data.read_u32::<LittleEndian>()?,
                ),
                crate_item,
            );
        }

        Ok(())
    }
}
