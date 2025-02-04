use crate::context::Context;
use crate::event::Event;
use crate::load_level::LevelLister;
use crate::render::Renderer;
use crate::types::Mode;
use crate::EventResult;

const LINES: [&str; 16] = [
    "ESC - quit",
    "F1   - this help",
    "F2   - save level",
    "F3   - load level",
    "F4   - create new level",
    "F6   - enable/disable automatic shadows",
    "F7   - edit general level variables",
    "F8/F9 - edit random crates for normal/dm games",
    " ",
    "- EDITOR -",
    "Q/W  - place/delete spotlights",
    "A/S  - place/delete steams",
    "Z/X/C - place/delete crates",
    "1/2  - place pl1/pl2 start",
    "SPACE - tile selection/editing mode",
    "ARROW KEYS - move viewport",
];

const WINDOW_LINES: [&str; 3] = [" ", "- WINDOW -", "+/- adjust rendering size"];

pub struct HelpState;

impl HelpState {
    pub fn new() -> Self {
        HelpState
    }

    pub fn handle_event(&self, event: Event) -> EventResult {
        match event {
            Event::Quit => EventResult::ChangeMode(Mode::Editor),
            Event::KeyDown { .. } => EventResult::ChangeMode(Mode::Editor),
            Event::Window { .. } => EventResult::ChangeMode(Mode::Editor),
            _ => EventResult::EventIgnored,
        }
    }

    pub fn render<L: LevelLister, R: Renderer>(
        &self,
        renderer: &mut R,
        context: &Context<L, R::Texture>,
    ) {
        let font = &context.font;
        let mut position = 6;
        for line_text in &LINES {
            font.render_text(renderer, line_text, (10, position));
            position += font.line_height() + 2;
        }
        if context.graphics.supports_scaling {
            for line_text in &WINDOW_LINES {
                font.render_text(renderer, line_text, (10, position));
                position += font.line_height() + 2;
            }
        }
    }
}
