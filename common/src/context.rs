use crate::fn2::FN2;
use crate::font::Font;
use crate::graphics::Graphics;
use crate::level::Level;
use crate::load_level::LevelLister;
use crate::render::Texture;
use crate::types::{TextureType, Trigonometry};

pub struct Textures<T: Texture> {
    pub floor: T,
    pub walls: T,
    pub shadows: T,
}

pub struct Context<L: LevelLister, T: Texture> {
    pub graphics: Graphics,
    pub fn2: FN2,
    pub font: Font<T>,
    pub textures: Textures<T>,
    pub level: Level,
    pub level_lister: L,
    pub selected_tile_id: u32,
    pub texture_type_selected: TextureType,
    pub texture_type_scrolled: TextureType,
    pub mouse: (u32, u32),
    pub level_save_name: String,
    pub saved_level_name: Option<String>,
    pub trigonometry: Trigonometry,
    pub automatic_shadows: bool,
}
