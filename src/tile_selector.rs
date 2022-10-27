extern crate sdl2;

use crate::context_util::resize;
use crate::types::*;
use crate::util::*;
use crate::Context;
use crate::NextMode::*;
use crate::{render, Renderer};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;

pub fn exec<'a>(renderer: &'a Renderer, context: &mut Context<'a>) -> NextMode {
    let floor_blocks_text_texture =
        renderer.create_text_texture(&context.font, "floor blocks (PAGEGUP/DOWN)");
    let wall_blocks_text_texture =
        renderer.create_text_texture(&context.font, "wall blocks (PAGEGUP/DOWN)");
    let shadow_blocks_text_texture = renderer.create_text_texture(
        &context.font,
        "shadows (PAGEGUP/DOWN) - clear with RIGHT CLICK",
    );
    let mut event_pump = context.sdl.event_pump().unwrap();
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Editor,
                Event::Window { win_event, .. } => {
                    if resize(renderer, context, win_event) {
                        return Editor;
                    }
                }
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::Space => {
                        return Editor;
                    }
                    Keycode::PageDown => {
                        context.texture_type_scrolled =
                            if context.texture_type_scrolled == TextureType::FLOOR {
                                TextureType::WALLS
                            } else if context.texture_type_scrolled == TextureType::WALLS {
                                TextureType::SHADOW
                            } else {
                                TextureType::FLOOR
                            }
                    }
                    Keycode::PageUp => {
                        context.texture_type_scrolled =
                            if context.texture_type_scrolled == TextureType::FLOOR {
                                TextureType::SHADOW
                            } else if context.texture_type_scrolled == TextureType::SHADOW {
                                TextureType::WALLS
                            } else {
                                TextureType::FLOOR
                            }
                    }
                    _ => {}
                },
                Event::MouseMotion { x, y, .. } => {
                    context.mouse.0 = x as u32;
                    context.mouse.1 = y as u32;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    let texture_selected = match &context.texture_type_scrolled {
                        TextureType::FLOOR => &context.textures.floor,
                        TextureType::WALLS => &context.textures.walls,
                        TextureType::SHADOW => &context.textures.shadows,
                    };
                    let (texture_width, texture_height) = render::get_texture_render_size(
                        texture_selected,
                        context.graphics.render_multiplier,
                    );
                    let clicked_tile_id = get_tile_id_from_coordinates(
                        &context.graphics,
                        &limit_coordinates(&context.mouse, &(texture_width, texture_height)),
                        texture_width / context.graphics.get_render_size(),
                        None,
                    );
                    if clicked_tile_id
                        < get_number_of_tiles_in_texture(
                            texture_selected,
                            context.graphics.tile_size,
                        )
                    {
                        context.selected_tile_id = clicked_tile_id;
                        context.texture_type_selected = context.texture_type_scrolled;
                        return Editor;
                    }
                }
                _ => {}
            }
        }

        renderer.clear_screen(Color::from((0, 0, 0)));
        let texture_selected = match context.texture_type_scrolled {
            TextureType::FLOOR => &context.textures.floor,
            TextureType::WALLS => &context.textures.walls,
            TextureType::SHADOW => &context.textures.shadows,
        };
        let render_multiplier = context.graphics.render_multiplier;
        let dst = render::get_texture_rect(texture_selected, render_multiplier);
        renderer.fill_and_render_texture(Color::from((200, 200, 200)), texture_selected, dst);
        let (texture_width, texture_height) =
            render::get_texture_render_size(texture_selected, render_multiplier);
        let highlighted_id = get_tile_id_from_coordinates(
            &context.graphics,
            &limit_coordinates(&context.mouse, &(texture_width, texture_height)),
            context.graphics.get_x_tiles_per_screen(),
            None,
        );
        renderer.highlight_selected_tile(
            &context.graphics,
            highlighted_id,
            &render::RendererColor::White,
        );
        if context.texture_type_selected == context.texture_type_scrolled {
            let coordinates = get_tile_coordinates(
                context.selected_tile_id,
                texture_width / context.graphics.render_multiplier,
                context.graphics.tile_size,
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
            renderer.highlight_selected_tile(
                &context.graphics,
                screen_tile_id,
                &render::RendererColor::Red,
            );
        }
        let active_text = match context.texture_type_scrolled {
            TextureType::FLOOR => &floor_blocks_text_texture,
            TextureType::WALLS => &wall_blocks_text_texture,
            TextureType::SHADOW => &shadow_blocks_text_texture,
        };
        renderer.render_text_texture_coordinates(
            active_text,
            get_bottom_text_position(context.graphics.resolution_y),
            context.graphics.get_render_size(),
            None,
        );
        renderer.render_and_wait();
    }
}
