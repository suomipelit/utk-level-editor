extern crate sdl2;

use crate::context_util::resize;
use crate::level::Level;
use crate::types::*;
use crate::util::{get_bottom_text_position, TITLE_POSITION};
use crate::NextMode::*;
use crate::{Context, Renderer};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Texture;

fn load_text<'a>(renderer: &'a Renderer, context: &Context, text: &str) -> Texture<'a> {
    renderer.create_text_texture(&context.font, text)
}

fn get_value(level: &Level, game_type: &GameType, index: usize) -> u32 {
    let crates = match game_type {
        GameType::Normal => &level.crates.random.normal,
        GameType::Deathmatch => &level.crates.random.deathmatch,
    };
    if index < crates.weapons.len() {
        crates.weapons[index]
    } else {
        let index = index - crates.weapons.len();
        if index < crates.bullets.len() {
            crates.bullets[index]
        } else {
            crates.energy
        }
    }
}

fn set_value(level: &mut Level, game_type: &GameType, index: usize, value: u32) {
    let crates = match game_type {
        GameType::Normal => &mut level.crates.random.normal,
        GameType::Deathmatch => &mut level.crates.random.deathmatch,
    };
    if index < crates.weapons.len() {
        crates.weapons[index] = value;
    } else {
        let index = index - crates.weapons.len();
        if index < crates.bullets.len() {
            crates.bullets[index] = value;
        } else {
            crates.energy = value;
        }
    }
}

pub fn exec<'a>(
    renderer: &'a Renderer,
    context: &mut Context<'a>,
    game_type: GameType,
) -> NextMode {
    let normal_game_instruction_text = &load_text(renderer, context, "NORMAL GAME CRATES");
    let deatchmatch_instruction_text = &load_text(renderer, context, "DEATHMATCH CRATES");
    let esc_instruction_text = &load_text(renderer, context, "press ESC to exit");
    let mut selected = 0;

    let mut event_pump = context.sdl.event_pump().unwrap();
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    context.sdl.video().unwrap().text_input().stop();
                    return Editor;
                }
                Event::Window { win_event, .. } => {
                    if resize(renderer, context, win_event) {
                        return Editor;
                    }
                }
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::Down => {
                        if selected < context.textures.crates.len() - 1 {
                            selected = selected + 1;
                        }
                    }
                    Keycode::Up => {
                        if selected > 0 {
                            selected = selected - 1;
                        }
                    }
                    Keycode::Right => {
                        let value = get_value(&context.level, &game_type, selected);
                        set_value(&mut context.level, &game_type, selected, value + 1);
                    }
                    Keycode::Left => {
                        let value = get_value(&context.level, &game_type, selected);
                        if value > 0 {
                            set_value(&mut context.level, &game_type, selected, value - 1);
                        }
                    }
                    _ => (),
                },
                _ => {}
            }
        }

        renderer.clear_screen(Color::from((0, 0, 0)));
        let render_size = context.graphics.get_render_size();

        renderer.render_text_texture_coordinates(
            match game_type {
                GameType::Normal => &normal_game_instruction_text,
                GameType::Deathmatch => &deatchmatch_instruction_text,
            },
            TITLE_POSITION,
            render_size,
            None,
        );

        let y = 50;
        let mut option_position = (40, y);
        let mut value_position = (280, option_position.1);
        for x in 0..context.textures.crates.len() {
            let option = &context.textures.crates[x];
            if selected == x {
                renderer.render_text_texture(
                    &context.textures.selected_icon,
                    option_position.0 - 20,
                    option_position.1 + 3,
                    render_size,
                    None,
                );
            }
            renderer.render_text_texture(
                &option,
                option_position.0,
                option_position.1,
                render_size,
                None,
            );
            let value_texture = renderer.create_text_texture(
                &context.font,
                &get_value(&context.level, &game_type, x).to_string(),
            );
            renderer.render_text_texture(
                &value_texture,
                value_position.0,
                value_position.1,
                render_size,
                None,
            );
            if x == 10 {
                option_position.1 = y;
                value_position.1 = option_position.1;
                option_position.0 = 330;
                value_position.0 = option_position.0 + 250;
            } else {
                option_position.1 += 20;
                value_position.1 = option_position.1;
            }
        }
        renderer.render_text_texture_coordinates(
            esc_instruction_text,
            get_bottom_text_position(context.graphics.resolution_y),
            render_size,
            None,
        );
        renderer.render_and_wait();
    }
}
