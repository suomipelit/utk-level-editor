use crate::context::Context;
use crate::editor::{EditorState, LevelWriter};
use crate::event::Event;
use crate::general_level_info::GeneralLevelInfoState;
use crate::help::HelpState;
use crate::load_level::{LevelLister, LoadLevelState};
use crate::random_item_editor::RandomItemEditorState;
use crate::render::{Renderer, Texture};
use crate::tile_selector::TileSelectState;
use crate::types::Mode;

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

pub enum RunState {
    Run,
    Quit,
}

pub struct State<L: LevelLister, W: LevelWriter> {
    mode: Mode,
    editor: EditorState<W>,
    tile_select: TileSelectState,
    help: HelpState,
    general_level_info: GeneralLevelInfoState,
    random_item_editor: RandomItemEditorState,
    load_level: LoadLevelState<L>,
}

impl<L: LevelLister, W: LevelWriter> State<L, W> {
    pub fn new(level_lister: L) -> Self {
        Self {
            mode: Mode::Editor,
            editor: EditorState::new(),
            tile_select: TileSelectState::new(),
            help: HelpState::new(),
            general_level_info: GeneralLevelInfoState::new(),
            random_item_editor: RandomItemEditorState::new(),
            load_level: LoadLevelState::new(level_lister),
        }
    }

    pub fn handle_event<T: Texture, I: TextInput>(
        &mut self,
        context: &mut Context<T>,
        text_input: &I,
        event: Event,
    ) -> RunState {
        let prev_mode = self.mode;
        self.mode = match self.mode {
            Mode::Editor => self.editor.handle_event(context, text_input, event),
            Mode::TileSelect => self.tile_select.handle_event(context, event),
            Mode::Help => self.help.handle_event(event),
            Mode::GeneralLevelInfo => self
                .general_level_info
                .handle_event(context, text_input, event),
            Mode::RandomItemEditor(game_mode) => self
                .random_item_editor
                .handle_event(context, text_input, game_mode, event),
            Mode::LoadLevel => self.load_level.handle_event(context, event),
            Mode::Quit => Mode::Quit,
        };
        if self.mode != prev_mode && self.mode == Mode::LoadLevel {
            self.load_level.enter()
        }
        match self.mode {
            Mode::Quit => RunState::Quit,
            _ => RunState::Run,
        }
    }

    pub fn render<'a, R: Renderer<'a>>(&mut self, renderer: &'a R, context: &Context<R::Texture>) {
        match self.mode {
            Mode::Editor => self.editor.render(renderer, context),
            Mode::TileSelect => self.tile_select.render(renderer, context),
            Mode::Help => self.help.render(renderer, context),
            Mode::GeneralLevelInfo => self.general_level_info.render(renderer, context),
            Mode::RandomItemEditor(game_type) => {
                self.random_item_editor.render(renderer, context, game_type)
            }
            Mode::LoadLevel => self.load_level.render(renderer, context),
            Mode::Quit => {}
        };
    }
}
