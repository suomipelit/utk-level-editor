use crate::graphics::Graphics;
use crate::util::get_tile_coordinates;

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

pub trait Texture {
    fn size(&self) -> (u32, u32);
}

pub trait Renderer {
    type Texture: Texture;

    fn create_texture(&mut self, width: u32, height: u32, data: &[Color]) -> Self::Texture;
    fn clear_screen(&mut self);
    fn draw_rect(&mut self, rect: &Rect, color: &RendererColor);
    fn draw_circle(&mut self, center: Point, radius: u32, color: &RendererColor);
    fn render_texture(&mut self, texture: &Self::Texture, src: Option<Rect>, dst: Rect);
    fn fill_and_render_texture(&mut self, color: RendererColor, texture: &Self::Texture, dst: Rect);
    fn window_size(&self) -> (u32, u32);
}

pub fn get_texture_rect<T: Texture>(texture: &T, render_multiplier: u32) -> Rect {
    let (width, height) = get_texture_render_size(texture, render_multiplier);
    Rect::new(0, 0, width, height)
}

pub fn get_texture_render_size<T: Texture>(texture: &T, render_multiplier: u32) -> (u32, u32) {
    let (width, height) = texture.size();
    (width * render_multiplier, height * render_multiplier)
}

pub fn highlight_selected_tile<R: Renderer>(
    renderer: &mut R,
    graphics: &Graphics,
    id: u32,
    color: &RendererColor,
) {
    let render_size = graphics.get_render_size();
    let render_multiplier = graphics.render_multiplier;
    let (x_logical, y_logical) = get_tile_coordinates(
        id,
        graphics.get_x_tiles_per_screen() * graphics.tile_size,
        graphics.tile_size,
    );
    let x = x_logical * render_multiplier;
    let y = y_logical * render_multiplier;
    renderer.draw_rect(
        &Rect::new(x as i32, y as i32, render_size, render_size),
        color,
    );
}
