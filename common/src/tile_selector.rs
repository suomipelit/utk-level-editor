use crate::context::Context;
use crate::event::{Event, Keycode, MouseButton};
use crate::load_level::LevelLister;
use crate::render::{
    get_texture_rect, get_texture_render_size, highlight_selected_tile, Renderer, RendererColor,
    Texture,
};
use crate::types::*;
use crate::util::*;
use crate::EventResult;

pub struct TileSelectState;

impl TileSelectState {
    pub fn new() -> Self {
        TileSelectState
    }

    pub fn handle_event<L: LevelLister, T: Texture>(
        &mut self,
        context: &mut Context<L, T>,
        event: Event,
    ) -> EventResult {
        match event {
            Event::Quit
            | Event::KeyDown {
                keycode: Keycode::Escape,
            } => return EventResult::ChangeMode(Mode::Editor),
            Event::Window { .. } => {
                return EventResult::ChangeMode(Mode::Editor);
            }
            Event::KeyDown { keycode } => match keycode {
                Keycode::Space => {
                    return EventResult::ChangeMode(Mode::Editor);
                }
                Keycode::PageDown | Keycode::Down => {
                    context.texture_type_scrolled =
                        if context.texture_type_scrolled == TextureType::Floor {
                            TextureType::Walls
                        } else if context.texture_type_scrolled == TextureType::Walls {
                            TextureType::Shadow
                        } else {
                            TextureType::Floor
                        };
                }
                Keycode::PageUp | Keycode::Up => {
                    context.texture_type_scrolled =
                        if context.texture_type_scrolled == TextureType::Floor {
                            TextureType::Shadow
                        } else if context.texture_type_scrolled == TextureType::Shadow {
                            TextureType::Walls
                        } else {
                            TextureType::Floor
                        };
                }
                _ => return EventResult::EventIgnored,
            },
            Event::MouseMotion { x, y, .. } => {
                context.mouse.0 = x as u32;
                context.mouse.1 = y as u32;
            }
            Event::MouseButtonDown {
                button: MouseButton::Left,
                ..
            } => {
                let texture_selected = match &context.texture_type_scrolled {
                    TextureType::Floor => &context.textures.floor,
                    TextureType::Walls => &context.textures.walls,
                    TextureType::Shadow => &context.textures.shadows,
                };
                let (texture_width, texture_height) =
                    get_texture_render_size(texture_selected, context.graphics.render_multiplier);
                let clicked_tile_id = get_tile_id_from_coordinates(
                    &context.graphics,
                    &limit_coordinates(&context.mouse, &(texture_width, texture_height)),
                    texture_width / context.graphics.get_render_size(),
                    None,
                );
                if clicked_tile_id < get_number_of_tiles_in_texture(texture_selected) {
                    context.selected_tile_id = clicked_tile_id;
                    context.texture_type_selected = context.texture_type_scrolled;
                    return EventResult::ChangeMode(Mode::Editor);
                }
            }
            _ => return EventResult::EventIgnored,
        }
        EventResult::KeepMode
    }

    pub fn render<L: LevelLister, R: Renderer>(
        &mut self,
        renderer: &mut R,
        context: &Context<L, R::Texture>,
    ) {
        let texture_selected = match context.texture_type_scrolled {
            TextureType::Floor => &context.textures.floor,
            TextureType::Walls => &context.textures.walls,
            TextureType::Shadow => &context.textures.shadows,
        };
        let render_multiplier = context.graphics.render_multiplier;
        let dst = get_texture_rect(texture_selected, render_multiplier);
        renderer.fill_rect(&dst, RendererColor::LightGrey);
        renderer.render_texture(&texture_selected, None, dst);
        let (texture_width, texture_height) =
            get_texture_render_size(texture_selected, render_multiplier);
        let highlighted_id = get_tile_id_from_coordinates(
            &context.graphics,
            &limit_coordinates(&context.mouse, &(texture_width, texture_height)),
            context.graphics.get_x_tiles_per_screen(),
            None,
        );
        highlight_selected_tile(
            renderer,
            &context.graphics,
            highlighted_id,
            RendererColor::White,
        );
        if context.texture_type_selected == context.texture_type_scrolled {
            let coordinates = get_tile_coordinates(
                context.selected_tile_id,
                texture_width / context.graphics.render_multiplier,
            );
            let render_multiplier = context.graphics.render_multiplier;
            let screen_tile_id = get_tile_id_from_coordinates(
                &context.graphics,
                &(
                    coordinates.0 * render_multiplier,
                    coordinates.1 * render_multiplier,
                ),
                context.graphics.get_x_tiles_per_screen(),
                None,
            );
            highlight_selected_tile(
                renderer,
                &context.graphics,
                screen_tile_id,
                RendererColor::Red,
            );
        }
        let active_text = match context.texture_type_scrolled {
            TextureType::Floor => "floor blocks (PAGEGUP/DOWN)",
            TextureType::Walls => "wall blocks (PAGEGUP/DOWN)",
            TextureType::Shadow => "shadows (PAGEGUP/DOWN) - clear with RIGHT CLICK",
        };
        context.font.render_text(
            renderer,
            active_text,
            get_bottom_text_position(&context.font, context.graphics.resolution_y),
        );
    }
}
