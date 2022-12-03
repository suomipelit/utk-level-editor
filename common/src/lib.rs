pub mod context;
pub mod editor;
pub mod event;
pub mod fn2;
pub mod font;
pub mod general_level_info;
pub mod graphics;
pub mod help;
pub mod level;
pub mod load_level;
pub mod random_item_editor;
pub mod render;
pub mod tile_selector;
pub mod types;
pub mod util;

pub trait TextInput {
    fn start(&self);
    fn stop(&self);
}
