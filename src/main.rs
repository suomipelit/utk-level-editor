extern crate sdl2;

use crate::context::Textures;
use crate::level::Level;
use crate::types::NextMode::*;
use sdl2::image::{InitFlag, LoadTexture};
mod context;
mod editor;
mod general_level_info;
mod help;
mod level;
mod load_level;
mod random_item_editor;
mod render;
mod tile_selector;
mod types;
mod util;
use context::Context;
use types::*;
use util::*;

pub fn main() {
    let sdl = sdl2::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG);
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window(
            "Ultimate Tapan Kaikki - Level Editor",
            RESOLUTION_X,
            RESOLUTION_Y,
        )
        .position_centered()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let font = ttf_context
        .load_font("./assets/TheJewishBitmap.ttf", 24)
        .unwrap();
    let selected_icon = render::get_font_texture(&texture_creator, &font, "*");
    let mut context = Context {
        sdl,
        canvas,
        texture_creator: &texture_creator,
        font,
        textures: Textures {
            floor: texture_creator.load_texture("./assets/FLOOR1.PNG").unwrap(),
            walls: texture_creator.load_texture("./assets/WALLS1.PNG").unwrap(),
            shadows: texture_creator
                .load_texture("./assets/SHADOWS_ALPHA.PNG")
                .unwrap(),
            selected_icon,
            saved_level_name: None,
        },
        level: Level::get_default_level((32, 22)),
        selected_tile_id: 0,
        texture_type_selected: TextureType::FLOOR,
        texture_type_scrolled: TextureType::FLOOR,
        mouse: (0, 0),
        level_save_name: String::new(),
        trigonometry: Trigonometry::new(),
        automatic_shadows: true,
    };

    let mut next_mode = NextMode::Editor;

    'running: loop {
        next_mode = match next_mode {
            Editor => editor::exec(&mut context),
            TileSelect => tile_selector::exec(&mut context),
            Help => help::exec(&mut context),
            GeneralLevelInfo => general_level_info::exec(&mut context),
            RandomItemEditor(game_type) => random_item_editor::exec(&mut context, game_type),
            LoadLevel => load_level::exec(&mut context),
            Quit => break 'running,
        }
    }
}
