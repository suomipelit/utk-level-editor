use crate::context::Context;
use crate::event::{Event, Keycode};
use crate::level::Level;
use crate::level::ALL_CRATES;
use crate::render::{Renderer, Texture};
use crate::types::*;
use crate::util::{get_bottom_text_position, get_title_position};
use crate::{EventResult, TextInput};

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

pub struct RandomItemEditorState {
    selected: usize,
}

impl RandomItemEditorState {
    pub fn new() -> Self {
        RandomItemEditorState { selected: 0 }
    }

    pub fn enter(&mut self) {
        self.selected = 0;
    }

    pub fn handle_event<T: Texture, I: TextInput>(
        &mut self,
        context: &mut Context<T>,
        text_input: &I,
        game_type: GameType,
        event: Event,
    ) -> EventResult {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Keycode::Escape,
            } => {
                text_input.stop();
                return EventResult::ChangeMode(Mode::Editor);
            }
            Event::Window { .. } => {
                return EventResult::ChangeMode(Mode::Editor);
            }
            Event::KeyDown { keycode, .. } => match keycode {
                Keycode::Down => {
                    if self.selected < ALL_CRATES.len() - 1 {
                        self.selected += 1;
                    }
                }
                Keycode::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    }
                }
                Keycode::Right => {
                    let value = get_value(&context.level, &game_type, self.selected);
                    set_value(&mut context.level, &game_type, self.selected, value + 1);
                }
                Keycode::Left => {
                    let value = get_value(&context.level, &game_type, self.selected);
                    if value > 0 {
                        set_value(&mut context.level, &game_type, self.selected, value - 1);
                    }
                }
                _ => return EventResult::EventIgnored,
            },
            _ => return EventResult::EventIgnored,
        }
        EventResult::KeepMode
    }

    pub fn render<R: Renderer>(
        &mut self,
        renderer: &mut R,
        context: &Context<R::Texture>,
        game_type: GameType,
    ) {
        renderer.clear_screen();

        context.font.render_text(
            renderer,
            match game_type {
                GameType::Normal => "NORMAL GAME CRATES",
                GameType::Deathmatch => "DEATHMATCH CRATES",
            },
            get_title_position(&context.font),
        );

        let y = context.font.px(25);
        let mut option_position = (context.font.px(20), y);
        let mut value_position = (context.font.px(140), option_position.1);
        for x in 0..ALL_CRATES.len() {
            if self.selected == x {
                context.font.render_text(
                    renderer,
                    "*",
                    (
                        option_position.0 - context.font.px(10),
                        option_position.1 + context.font.px(1),
                    ),
                );
            }
            context
                .font
                .render_text(renderer, ALL_CRATES[x], option_position);
            context.font.render_text(
                renderer,
                &get_value(&context.level, &game_type, x).to_string(),
                value_position,
            );
            if x == 10 {
                option_position.1 = y;
                value_position.1 = option_position.1;
                option_position.0 = context.font.px(165);
                value_position.0 = option_position.0 + context.font.px(125);
            } else {
                option_position.1 += context.font.px(10);
                value_position.1 = option_position.1;
            }
        }
        context.font.render_text(
            renderer,
            "press ESC to exit",
            get_bottom_text_position(&context.font, context.graphics.resolution_y),
        );
    }
}
