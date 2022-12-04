mod render;

use sdl2::image::InitFlag;
use sdl2::keyboard::TextInputUtil;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::time::Duration;

use crate::render::{SdlRenderer, SdlTexture};
use common::context::{Context, Textures};
use common::editor::LevelWriter;
use common::event::{Event, Keycode, MouseButton, WindowEvent};
use common::fn2::FN2;
use common::font::Font;
use common::graphics::Graphics;
use common::level::Level;
use common::load_level::LevelLister;
use common::render::Renderer;
use common::types::{TextureType, Trigonometry};
use common::{RunState, State, TextInput};

struct SdlTextInput(TextInputUtil);

impl TextInput for SdlTextInput {
    fn start(&self) {
        self.0.start();
    }

    fn stop(&self) {
        self.0.stop();
    }
}

pub fn main() {
    let sdl = sdl2::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG);
    let video_subsystem = sdl.video().unwrap();
    let graphics = Graphics::new();
    let window = video_subsystem
        .window(
            "Ultimate Tapan Kaikki - Level Editor",
            graphics.resolution_x,
            graphics.resolution_y,
        )
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let renderer = SdlRenderer::new(window);
    let fn2 = {
        let mut font_data = Vec::new();
        File::open("assets/TETRIS.FN2")
            .expect("Failed to open assets/TETRIS.FN2")
            .read_to_end(&mut font_data)
            .unwrap();
        FN2::parse(&font_data)
    };
    let font = Font::new(&renderer, &fn2);
    let textures = get_textures(&renderer);
    let mut context = Context {
        graphics,
        fn2,
        font,
        textures,
        level: Level::get_default_level((32, 22)),
        selected_tile_id: 0,
        texture_type_selected: TextureType::Floor,
        texture_type_scrolled: TextureType::Floor,
        mouse: (0, 0),
        level_save_name: String::new(),
        saved_level_name: None,
        trigonometry: Trigonometry::new(),
        automatic_shadows: true,
    };
    let text_input = SdlTextInput(video_subsystem.text_input());

    let level_lister = DirectoryLevelLister::new();
    let mut state: State<DirectoryLevelLister, FileLevelWriter> = State::new(level_lister);
    loop {
        for sdl_event in event_pump.poll_iter() {
            if let Some(event) = convert_event(sdl_event) {
                if let Event::Window { win_event } = event {
                    resize(&renderer, &mut context, win_event);
                }
                match state.handle_event(&mut context, &text_input, event) {
                    RunState::Quit => return,
                    RunState::Run => {}
                }
            }
        }
        state.render(&renderer, &context);
        renderer.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn refresh<'a>(
    renderer: &'a SdlRenderer,
    context: &mut Context<SdlTexture<'a>>,
    window_size: (u32, u32),
) {
    context.graphics.resolution_x = window_size.0;
    context.graphics.resolution_y = window_size.1;
    context.font = Font::new(renderer, &context.fn2);
    context.textures = get_textures(renderer);
}

pub fn resize<'a>(
    renderer: &'a SdlRenderer,
    context: &mut Context<SdlTexture<'a>>,
    event: WindowEvent,
) {
    match event {
        WindowEvent::Resized { width, height } => {
            refresh(renderer, context, (width, height));
        }
        WindowEvent::Maximized => {
            refresh(renderer, context, renderer.window_size());
        }
    }
}

pub fn get_textures<'a>(renderer: &'a SdlRenderer) -> Textures<SdlTexture<'a>> {
    Textures {
        floor: renderer.load_texture("assets/FLOOR1.PNG"),
        walls: renderer.load_texture("assets/WALLS1.PNG"),
        shadows: renderer.load_texture("assets/SHADOWS_ALPHA.PNG"),
    }
}

fn convert_event(event: sdl2::event::Event) -> Option<Event> {
    use sdl2::event::Event as SdlEvent;
    use sdl2::event::WindowEvent as SdlWindowEvent;

    match event {
        SdlEvent::Quit { .. } => Some(Event::Quit),
        SdlEvent::Window { win_event, .. } => match win_event {
            SdlWindowEvent::Resized(w, h) => {
                if w >= 0 && h >= 0 {
                    Some(Event::Window {
                        win_event: WindowEvent::Resized {
                            width: w as u32,
                            height: h as u32,
                        },
                    })
                } else {
                    None
                }
            }
            SdlWindowEvent::Maximized => Some(Event::Window {
                win_event: WindowEvent::Maximized,
            }),
            _ => None,
        },
        SdlEvent::KeyDown {
            keycode: Some(sdl_keycode),
            ..
        } => convert_keycode(sdl_keycode).map(|keycode| Event::KeyDown { keycode }),
        SdlEvent::MouseButtonDown { mouse_btn, .. } => {
            convert_mouse_button(mouse_btn).map(|button| Event::MouseButtonDown { button })
        }
        SdlEvent::MouseButtonUp { mouse_btn, .. } => {
            convert_mouse_button(mouse_btn).map(|button| Event::MouseButtonUp { button })
        }
        SdlEvent::MouseMotion { x, y, .. } => {
            if x >= 0 && y >= 0 {
                Some(Event::MouseMotion {
                    x: x as u32,
                    y: y as u32,
                })
            } else {
                None
            }
        }
        SdlEvent::TextInput { text, .. } => Some(Event::TextInput { text }),
        _ => None,
    }
}

fn convert_keycode(keycode: sdl2::keyboard::Keycode) -> Option<Keycode> {
    use sdl2::keyboard::Keycode as SdlKeycode;
    match keycode {
        SdlKeycode::Escape => Some(Keycode::Escape),
        SdlKeycode::Backspace => Some(Keycode::Backspace),
        SdlKeycode::Return => Some(Keycode::Return),
        SdlKeycode::Space => Some(Keycode::Space),
        SdlKeycode::PageDown => Some(Keycode::PageDown),
        SdlKeycode::PageUp => Some(Keycode::PageUp),
        SdlKeycode::Up => Some(Keycode::Up),
        SdlKeycode::Down => Some(Keycode::Down),
        SdlKeycode::Left => Some(Keycode::Left),
        SdlKeycode::Right => Some(Keycode::Right),
        SdlKeycode::KpEnter => Some(Keycode::KpEnter),
        SdlKeycode::KpMinus => Some(Keycode::KpMinus),
        SdlKeycode::KpPlus => Some(Keycode::KpPlus),
        SdlKeycode::Minus => Some(Keycode::Minus),
        SdlKeycode::Plus => Some(Keycode::Plus),
        SdlKeycode::A => Some(Keycode::A),
        SdlKeycode::C => Some(Keycode::C),
        SdlKeycode::Q => Some(Keycode::Q),
        SdlKeycode::S => Some(Keycode::S),
        SdlKeycode::W => Some(Keycode::W),
        SdlKeycode::X => Some(Keycode::X),
        SdlKeycode::Y => Some(Keycode::Y),
        SdlKeycode::Z => Some(Keycode::Z),
        SdlKeycode::Num1 => Some(Keycode::Num1),
        SdlKeycode::Num2 => Some(Keycode::Num2),
        SdlKeycode::F1 => Some(Keycode::F1),
        SdlKeycode::F2 => Some(Keycode::F2),
        SdlKeycode::F3 => Some(Keycode::F3),
        SdlKeycode::F4 => Some(Keycode::F4),
        SdlKeycode::F6 => Some(Keycode::F6),
        SdlKeycode::F7 => Some(Keycode::F7),
        SdlKeycode::F8 => Some(Keycode::F8),
        SdlKeycode::F9 => Some(Keycode::F9),
        _ => None,
    }
}

fn convert_mouse_button(button: sdl2::mouse::MouseButton) -> Option<MouseButton> {
    match button {
        sdl2::mouse::MouseButton::Left => Some(MouseButton::Left),
        sdl2::mouse::MouseButton::Right => Some(MouseButton::Right),
        _ => None,
    }
}

struct DirectoryLevelLister {
    files: Vec<String>,
}

impl DirectoryLevelLister {
    pub fn new() -> Self {
        Self { files: vec![] }
    }
}

impl LevelLister for DirectoryLevelLister {
    fn refresh(&mut self) {
        self.files = fs::read_dir("./")
            .unwrap()
            .filter_map(|entry_result| {
                let entry = entry_result.unwrap();
                let is_file = entry.metadata().unwrap().is_file();
                let filename = entry.file_name().into_string().unwrap();
                if is_file && filename.to_uppercase().ends_with(".LEV") {
                    Some(filename)
                } else {
                    None
                }
            })
            .collect();
    }

    fn len(&self) -> usize {
        self.files.len()
    }

    fn level_name(&self, index: usize) -> &str {
        &self.files[index]
    }

    fn load_level(&self, index: usize) -> Vec<u8> {
        let mut file = File::open(&self.files[index]).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        buffer
    }
}

struct FileLevelWriter;

impl LevelWriter for FileLevelWriter {
    fn write(level: &Level, filename: &str) {
        let level_data = level.serialize();
        let mut file = File::create(filename).unwrap();
        file.write_all(&level_data).unwrap();
    }
}
