use crate::context::Context;
use crate::event::{Event, Keycode};
use crate::load_level::LevelLister;
use crate::render::{Renderer, Texture};
use crate::types::*;
use crate::util::get_bottom_text_position;
use crate::{EventResult, TextInput};

enum Value {
    Comment,
    TimeLimit,
    Number(usize),
}

struct ConfigOption {
    text: &'static str,
    value: Value,
}

fn load_value_text<L: LevelLister, T: Texture>(
    context: &Context<L, T>,
    value: &Value,
) -> Option<String> {
    let string = match value {
        Value::Number(number) => context.level.general_info.enemy_table[*number].to_string(),
        Value::TimeLimit => format!("{} seconds", context.level.general_info.time_limit),
        Value::Comment => context.level.general_info.comment.to_string(),
    };
    if !string.is_empty() {
        Some(string)
    } else {
        None
    }
}

fn sanitize_level_comment_input(new_text: &str, target_text: &mut String) {
    if (new_text.chars().all(char::is_alphanumeric) || new_text.chars().all(char::is_whitespace))
        && (target_text.len() + new_text.len() <= 19)
    {
        *target_text += new_text;
    }
}

pub struct GeneralLevelInfoState {
    options: [ConfigOption; 10],
    selected: usize,
}

impl GeneralLevelInfoState {
    pub fn new() -> Self {
        let options = [
            ConfigOption {
                text: "level comment:",
                value: Value::Comment,
            },
            ConfigOption {
                text: "time limit:",
                value: Value::TimeLimit,
            },
            ConfigOption {
                text: "pistol boys:",
                value: Value::Number(0),
            },
            ConfigOption {
                text: "shotgun maniacs:",
                value: Value::Number(1),
            },
            ConfigOption {
                text: "uzi rebels:",
                value: Value::Number(2),
            },
            ConfigOption {
                text: "commandos:",
                value: Value::Number(3),
            },
            ConfigOption {
                text: "granade mofos:",
                value: Value::Number(4),
            },
            ConfigOption {
                text: "civilians:",
                value: Value::Number(5),
            },
            ConfigOption {
                text: "punishers:",
                value: Value::Number(6),
            },
            ConfigOption {
                text: "flamers:",
                value: Value::Number(7),
            },
        ];
        GeneralLevelInfoState {
            options,
            selected: 0,
        }
    }

    pub fn enter<I: TextInput>(&mut self, text_input: &mut I) {
        self.selected = 0;
        self.enable_text_editing_if_needed(text_input);
    }

    pub fn handle_event<L: LevelLister, T: Texture, I: TextInput>(
        &mut self,
        context: &mut Context<L, T>,
        text_input: &mut I,
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
            Event::TextInput { text, .. } => {
                if let Value::Comment = self.options[self.selected].value {
                    sanitize_level_comment_input(&text, &mut context.level.general_info.comment)
                }
            }
            Event::KeyDown { keycode, .. } => match keycode {
                Keycode::Down => {
                    if self.selected < self.options.len() - 1 {
                        self.selected += 1;
                        self.enable_text_editing_if_needed(text_input);
                    }
                }
                Keycode::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                        self.enable_text_editing_if_needed(text_input);
                    }
                }
                Keycode::Right => match self.options[self.selected].value {
                    Value::Number(index) => context.level.general_info.enemy_table[index] += 1,
                    Value::TimeLimit => context.level.general_info.time_limit += 10,
                    _ => return EventResult::EventIgnored,
                },
                Keycode::Left => match self.options[self.selected].value {
                    Value::Number(index) => {
                        let value = &mut context.level.general_info.enemy_table[index];
                        if *value > 0 {
                            *value -= 1;
                        }
                    }
                    Value::TimeLimit => {
                        let value = &mut context.level.general_info.time_limit;
                        if *value > 0 {
                            *value -= 10;
                        }
                    }
                    _ => return EventResult::EventIgnored,
                },
                Keycode::Backspace => {
                    if let Value::Comment = self.options[self.selected].value {
                        context.level.general_info.comment.pop();
                    }
                }
                _ => return EventResult::EventIgnored,
            },
            _ => return EventResult::EventIgnored,
        }
        EventResult::KeepMode
    }

    pub fn render<L: LevelLister, R: Renderer>(
        &mut self,
        renderer: &mut R,
        context: &Context<L, R::Texture>,
    ) {
        let mut option_position = (context.font.px(20), context.font.px(10));
        let mut value_position = (context.font.px(150), option_position.1);
        for x in 0..self.options.len() {
            let option = &self.options[x];
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
                .render_text(renderer, option.text, option_position);
            let value_text = &load_value_text(context, &option.value);
            match value_text {
                Some(text) => context.font.render_text(renderer, text, value_position),
                None => (),
            };
            option_position.1 += context.font.line_height() + context.font.px(2);
            value_position.1 = option_position.1;
        }
        context.font.render_text(
            renderer,
            "press ESC to exit",
            get_bottom_text_position(&context.font, context.graphics.resolution_y),
        );
    }

    fn enable_text_editing_if_needed<I: TextInput>(&self, text_input: &mut I) {
        match self.options[self.selected].value {
            Value::Comment => text_input.start(),
            _ => text_input.stop(),
        }
    }
}
