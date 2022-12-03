use crate::graphics::Graphics;

pub enum RendererColor {
    Black,
    White,
    Red,
    Blue,
    LightBlue,
    LightGreen,
    LightGrey,
}

impl RendererColor {
    pub fn to_color(&self) -> Color {
        match self {
            RendererColor::Black => Color::from((0, 0, 0)),
            RendererColor::White => Color::from((255, 255, 255)),
            RendererColor::Red => Color::from((255, 0, 0)),
            RendererColor::Blue => Color::from((0, 0, 255)),
            RendererColor::LightBlue => Color::from((100, 100, 255)),
            RendererColor::LightGreen => Color::from((100, 255, 100)),
            RendererColor::LightGrey => Color::from((200, 200, 200)),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b, a: 255 }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn top_left(&self) -> Point {
        Point::new(self.x, self.y)
    }

    pub fn top_right(&self) -> Point {
        Point::new(self.x + self.width as i32, self.y)
    }

    pub fn bottom_left(&self) -> Point {
        Point::new(self.x, self.y + self.height as i32)
    }

    pub fn bottom_right(&self) -> Point {
        Point::new(self.x + self.width as i32, self.y + self.height as i32)
    }
}

pub trait Renderer<'a> {
    type Texture;

    fn load_texture(&'a self, path: &str) -> Self::Texture;
    fn create_texture(&'a self, width: u32, height: u32, data: &[Color]) -> Self::Texture;
    fn clear_screen(&self);
    fn highlight_selected_tile(&self, graphics: &Graphics, id: u32, color: &RendererColor);
    fn draw_rect(&self, rect: &Rect, color: &RendererColor);
    fn draw_circle(&self, center: Point, radius: u32, color: &RendererColor);
    fn render_texture(&self, texture: &Self::Texture, src: Option<Rect>, dst: Rect);
    fn fill_and_render_texture(&self, color: RendererColor, texture: &Self::Texture, dst: Rect);
    fn get_texture_size(texture: &Self::Texture) -> (u32, u32);
    fn window_size(&self) -> (u32, u32);
}

pub fn get_texture_rect<'a, R: Renderer<'a>>(texture: &R::Texture, render_multiplier: u32) -> Rect {
    let (width, height) = get_texture_render_size::<R>(texture, render_multiplier);
    Rect::new(0, 0, width, height)
}

pub fn get_texture_render_size<'a, R: Renderer<'a>>(
    texture: &R::Texture,
    render_multiplier: u32,
) -> (u32, u32) {
    let (width, height) = R::get_texture_size(texture);
    (width * render_multiplier, height * render_multiplier)
}
