#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TextureType {
    Floor = 0,
    Walls = 1,
    Shadow = 2,
}

impl TextureType {
    pub fn from_u32(value: u32) -> TextureType {
        match value {
            0 => TextureType::Floor,
            1 => TextureType::Walls,
            2 => TextureType::Shadow,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub(crate) texture_type: TextureType,
    pub(crate) id: u32,
    pub(crate) shadow: u32,
}

pub type Tiles = Vec<Vec<Tile>>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameType {
    Normal,
    Deathmatch,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Editor,
    TileSelect,
    Help,
    GeneralLevelInfo,
    RandomItemEditor(GameType),
    LoadLevel,
}

pub struct Trigonometry {
    pub(crate) sin: [f32; 360],
    pub(crate) cos: [f32; 360],
}
