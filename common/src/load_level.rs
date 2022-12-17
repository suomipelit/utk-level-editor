use crate::context::Context;
use crate::event::{Event, Keycode};
use crate::render::{Renderer, Texture};
use crate::types::*;
use crate::util::{get_bottom_text_position, get_title_position};
use crate::EventResult;

pub trait LevelLister {
    fn refresh(&mut self);
    fn len(&self) -> usize;
    fn level_name(&self, index: usize) -> &str;
    fn load_level(&self, index: usize) -> Vec<u8>;
}

pub struct LoadLevelState<L: LevelLister> {
    level_lister: L,
    selected: usize,
}

impl<L: LevelLister> LoadLevelState<L> {
    pub fn new(level_lister: L) -> Self {
        LoadLevelState {
            level_lister,
            selected: 0,
        }
    }

    pub fn enter(&mut self) {
        self.level_lister.refresh();
        self.selected = 0;
    }

    pub fn handle_event<T: Texture>(
        &mut self,
        context: &mut Context<T>,
        event: Event,
    ) -> EventResult {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Keycode::Escape,
            } => return EventResult::ChangeMode(Mode::Editor),
            Event::Window { .. } => {
                return EventResult::ChangeMode(Mode::Editor);
            }
            Event::KeyDown { keycode, .. } => match keycode {
                Keycode::Down => {
                    if self.selected < self.level_lister.len() - 1 {
                        self.selected += 1;
                    }
                }
                Keycode::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    }
                }
                Keycode::Return | Keycode::KpEnter => {
                    if self.level_lister.len() > 0 {
                        let level_data = self.level_lister.load_level(self.selected);
                        context.level.deserialize(&level_data).unwrap();
                        let level_name = self.level_lister.level_name(self.selected).to_string();
                        context.saved_level_name = Some(level_name.clone());
                        context.level_save_name =
                            level_name.strip_suffix(".LEV").unwrap().to_string();
                    }
                    return EventResult::ChangeMode(Mode::Editor);
                }
                _ => return EventResult::EventIgnored,
            },
            _ => return EventResult::EventIgnored,
        }
        EventResult::KeepMode
    }

    pub fn render<R: Renderer>(&mut self, renderer: &mut R, context: &Context<R::Texture>) {
        renderer.clear_screen();
        let text_position = (40, 60);
        context
            .font
            .render_text(renderer, "LOAD LEVEL:", get_title_position(&context.font));
        let line_spacing = 20;
        for x in 0..self.level_lister.len() {
            if self.selected == x {
                context.font.render_text(
                    renderer,
                    "*",
                    (
                        text_position.0 - 20,
                        text_position.1 + 3 + x as u32 * line_spacing,
                    ),
                );
            }
            context.font.render_text(
                renderer,
                &self.level_lister.level_name(x),
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
