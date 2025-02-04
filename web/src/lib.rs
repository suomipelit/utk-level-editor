mod render;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::render::{CanvasRenderer, CanvasTexture};
use common::context::{Context, Textures};
use common::editor::LevelWriter;
use common::event::Event;
use common::fn2::FN2;
use common::font::Font;
use common::graphics::Graphics;
use common::level::Level;
use common::load_level::LevelLister;
use common::types::{TextureType, Trigonometry};
use common::{RunState, State, TextInput};
use log::Level as LogLevel;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub struct WebImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

#[wasm_bindgen]
impl WebImage {
    pub fn new(width: u32, height: u32, data: &[u8]) -> Self {
        Self {
            width,
            height,
            data: data.to_vec(),
        }
    }
}

#[wasm_bindgen]
pub struct LevelEditor {
    renderer: CanvasRenderer,
    state: State<WebLevelWriter>,
    context: Context<WebLevelLister, CanvasTexture>,
    text_input: WebTextInput,
}

#[wasm_bindgen]
impl LevelEditor {
    pub fn new(
        floor_texture: WebImage,
        walls_texture: WebImage,
        shadows_alpha_texture: WebImage,
        font_data: &[u8],
        show_file_upload: js_sys::Function,
        hide_file_upload: js_sys::Function,
    ) -> Self {
        console_log::init_with_level(LogLevel::Debug).unwrap();

        let width = 640u32;
        let height = 400u32;

        let graphics = Graphics::new((width, height), 1);

        let mut renderer = CanvasRenderer::new(width, height);
        let fn2 = FN2::parse(font_data);
        let font = Font::new(&mut renderer, &fn2, 1);
        let textures = Textures {
            floor: renderer.create_texture_rgba(
                floor_texture.width,
                floor_texture.height,
                &floor_texture.data,
            ),
            walls: renderer.create_texture_rgba(
                walls_texture.width,
                walls_texture.height,
                &walls_texture.data,
            ),
            shadows: renderer.create_texture_rgba(
                shadows_alpha_texture.width,
                shadows_alpha_texture.height,
                &shadows_alpha_texture.data,
            ),
        };

        let context = Context {
            graphics,
            fn2,
            font,
            textures,
            level: Level::get_default_level((32, 22)),
            level_lister: WebLevelLister::new(show_file_upload, hide_file_upload),
            selected_tile_id: 0,
            texture_type_selected: TextureType::Floor,
            texture_type_scrolled: TextureType::Floor,
            mouse: (0, 0),
            level_save_name: String::new(),
            saved_level_name: None,
            trigonometry: Trigonometry::new(),
            automatic_shadows: true,
        };
        let text_input = WebTextInput { enabled: false };
        let state: State<WebLevelWriter> = State::new();
        Self {
            renderer,
            state,
            context,
            text_input,
        }
    }

    pub fn screen(&self) -> *const u32 {
        self.renderer.pixels()
    }
    pub fn screen_width(&self) -> u32 {
        self.renderer.width()
    }
    pub fn screen_height(&self) -> u32 {
        self.renderer.height()
    }

    fn handle_event(&mut self, event: Event) -> bool {
        let run_state = self
            .state
            .handle_event(&mut self.context, &mut self.text_input, event);
        match run_state {
            RunState::Run { needs_render } => needs_render,
            RunState::Quit => false,
        }
    }

    pub fn mouse_move(&mut self, x: u32, y: u32) -> bool {
        self.handle_event(Event::MouseMotion { x, y })
    }
    pub fn mouse_down(&mut self, button: MouseButton) -> bool {
        self.handle_event(Event::MouseButtonDown {
            button: button.into(),
        })
    }
    pub fn mouse_up(&mut self, button: MouseButton) -> bool {
        self.handle_event(Event::MouseButtonUp {
            button: button.into(),
        })
    }
    pub fn key_down(&mut self, key: Option<Keycode>, text: Option<String>) -> bool {
        if self.text_input.enabled {
            if let Some(text) = text {
                return self.handle_event(Event::TextInput { text });
            }
        }
        if let Some(key) = key {
            self.handle_event(Event::KeyDown {
                keycode: key.into(),
            })
        } else {
            false
        }
    }
    pub fn add_level_file(&mut self, name: String, data: &[u8]) {
        self.context.level_lister.add_file(name, data);
    }

    pub fn frame(&mut self) {
        self.state.render(&mut self.renderer, &self.context);
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum MouseButton {
    Left,
    Right,
}

impl From<MouseButton> for common::event::MouseButton {
    fn from(button: MouseButton) -> Self {
        match button {
            MouseButton::Left => Self::Left,
            MouseButton::Right => Self::Right,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Keycode {
    Escape,
    Backspace,
    Return,
    Space,
    PageDown,
    PageUp,
    Up,
    Down,
    Left,
    Right,
    KpEnter,
    KpMinus,
    KpPlus,
    Minus,
    Plus,
    A,
    C,
    Q,
    S,
    W,
    X,
    Y,
    Z,
    Num1,
    Num2,
    F1,
    F2,
    F3,
    F4,
    F6,
    F7,
    F8,
    F9,
}

impl From<Keycode> for common::event::Keycode {
    fn from(key: Keycode) -> Self {
        match key {
            Keycode::Escape => Self::Escape,
            Keycode::Backspace => Self::Backspace,
            Keycode::Return => Self::Return,
            Keycode::Space => Self::Space,
            Keycode::PageDown => Self::PageDown,
            Keycode::PageUp => Self::PageUp,
            Keycode::Up => Self::Up,
            Keycode::Down => Self::Down,
            Keycode::Left => Self::Left,
            Keycode::Right => Self::Right,
            Keycode::KpEnter => Self::KpEnter,
            Keycode::KpMinus => Self::KpMinus,
            Keycode::KpPlus => Self::KpPlus,
            Keycode::Minus => Self::Minus,
            Keycode::Plus => Self::Plus,
            Keycode::A => Self::A,
            Keycode::C => Self::C,
            Keycode::Q => Self::Q,
            Keycode::S => Self::S,
            Keycode::W => Self::W,
            Keycode::X => Self::X,
            Keycode::Y => Self::Y,
            Keycode::Z => Self::Z,
            Keycode::Num1 => Self::Num1,
            Keycode::Num2 => Self::Num2,
            Keycode::F1 => Self::F1,
            Keycode::F2 => Self::F2,
            Keycode::F3 => Self::F3,
            Keycode::F4 => Self::F4,
            Keycode::F6 => Self::F6,
            Keycode::F7 => Self::F7,
            Keycode::F8 => Self::F8,
            Keycode::F9 => Self::F9,
        }
    }
}

struct WebTextInput {
    enabled: bool,
}

impl TextInput for WebTextInput {
    fn start(&mut self) {
        self.enabled = true;
    }

    fn stop(&mut self) {
        self.enabled = false;
    }
}

struct WebLevelLister {
    show_file_upload: js_sys::Function,
    hide_file_upload: js_sys::Function,
    files: Vec<(String, Vec<u8>)>,
}

impl WebLevelLister {
    fn new(show_file_upload: js_sys::Function, hide_file_upload: js_sys::Function) -> Self {
        Self {
            show_file_upload,
            hide_file_upload,
            files: Vec::new(),
        }
    }

    fn add_file(&mut self, name: String, data: &[u8]) {
        self.files.push((name, data.to_vec()));
    }
}

impl LevelLister for WebLevelLister {
    fn refresh(&mut self) {
        self.show_file_upload.call0(&JsValue::NULL).unwrap();
    }

    fn reset(&mut self) {
        self.hide_file_upload.call0(&JsValue::NULL).unwrap();
    }

    fn len(&self) -> usize {
        self.files.len()
    }

    fn level_name(&self, index: usize) -> &str {
        self.files[index].0.as_str()
    }

    fn load_level(&self, index: usize) -> Vec<u8> {
        self.files[index].1.clone()
    }
}

#[wasm_bindgen(module = "/src/js/file.js")]
extern "C" {
    fn write_file(name: &str, data: &[u8]);
}

struct WebLevelWriter;

impl LevelWriter for WebLevelWriter {
    fn write(filename: &str, level_data: &[u8]) {
        write_file(filename, level_data);
    }
}
