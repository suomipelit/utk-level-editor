use crate::context::Context;
use crate::event::{Event, Keycode};
use crate::render::{Renderer, Texture};
use crate::types::*;
use crate::util::{get_bottom_text_position, get_title_position};
use crate::EventResult;

pub trait LevelLister {
    fn refresh(&mut self);
    fn reset(&mut self);
    fn len(&self) -> usize;
    fn level_name(&self, index: usize) -> &str;
    fn load_level(&self, index: usize) -> Vec<u8>;
}

pub struct LoadLevelState {
    selected: usize,
}

impl LoadLevelState {
    pub fn new() -> Self {
        LoadLevelState { selected: 0 }
    }

    pub fn enter<L: LevelLister, T: Texture>(&mut self, context: &mut Context<L, T>) {
        context.level_lister.refresh();
        self.selected = 0;
    }

    pub fn handle_event<L: LevelLister, T: Texture>(
        &mut self,
        context: &mut Context<L, T>,
        event: Event,
    ) -> EventResult {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Keycode::Escape,
            } => {
                context.level_lister.reset();
                return EventResult::ChangeMode(Mode::Editor);
            }
            Event::Window { .. } => {
                return EventResult::ChangeMode(Mode::Editor);
            }
            Event::KeyDown { keycode, .. } => match keycode {
                Keycode::Down => {
                    if self.selected < context.level_lister.len() - 1 {
                        self.selected += 1;
                    }
                }
                Keycode::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    }
                }
                Keycode::Return | Keycode::KpEnter => {
                    if context.level_lister.len() > 0 {
                        let level_data = context.level_lister.load_level(self.selected);
                        context.level.deserialize(&level_data).unwrap();
                        let level_name = context.level_lister.level_name(self.selected).to_string();
                        context.saved_level_name = Some(level_name.clone());
                        context.level_save_name =
                            level_name.strip_suffix(".LEV").unwrap().to_string();
                    }
                    context.level_lister.reset();
                    return EventResult::ChangeMode(Mode::Editor);
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
        let text_position = (40, 60);
        context
            .font
            .render_text(renderer, "LOAD LEVEL:", get_title_position(&context.font));
        let line_spacing = context.font.px(10);
        for x in 0..context.level_lister.len() {
            if self.selected == x {
                context.font.render_text(
                    renderer,
                    "*",
                    (
                        text_position.0 - context.font.px(10),
                        text_position.1 + context.font.px(1) + x as u32 * line_spacing,
                    ),
                );
            }
            context.font.render_text(
                renderer,
                context.level_lister.level_name(x),
                (text_position.0, text_position.1 + line_spacing * x as u32),
            );
        }
        context.font.render_text(
            renderer,
            "ENTER to select or ESC to exit",
            get_bottom_text_position(&context.font, context.graphics.resolution_y),
        );
    }
}
