use sdl2::image::LoadTexture;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Texture as SdlTexture;
use sdl2::render::TextureQuery;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};
use std::cell::{RefCell, RefMut};

use crate::util::*;
use crate::Graphics;

pub enum RendererColor {
    Black,
    White,
    Red,
    Blue,
    LightBlue,
    LightGreen,
    LightGrey,
}

fn get_sdl_color(color: &RendererColor) -> Color {
    match &color {
        RendererColor::Black => Color::from((0, 0, 0)),
        RendererColor::White => Color::from((255, 255, 255)),
        RendererColor::Red => Color::from((255, 0, 0)),
        RendererColor::Blue => Color::from((0, 0, 255)),
        RendererColor::LightBlue => Color::from((100, 100, 255)),
        RendererColor::LightGreen => Color::from((100, 255, 100)),
        RendererColor::LightGrey => Color::from((200, 200, 200)),
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

impl From<Color> for sdl2::pixels::Color {
    fn from(color: Color) -> Self {
        Self::RGBA(color.r, color.g, color.b, color.a)
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

impl From<Point> for sdl2::rect::Point {
    fn from(point: Point) -> Self {
        Self::new(point.x, point.y)
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

impl From<Rect> for sdl2::rect::Rect {
    fn from(rect: Rect) -> Self {
        sdl2::rect::Rect::new(rect.x, rect.y, rect.width, rect.height)
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

pub struct SdlRenderer {
    canvas: RefCell<Canvas<Window>>,
    texture_creator: TextureCreator<WindowContext>,
}

impl SdlRenderer {
    pub fn new(window: Window) -> Self {
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        Self {
            canvas: RefCell::new(canvas),
            texture_creator,
        }
    }

    pub fn present(&self) {
        self.canvas_mut().present();
    }

    fn canvas_mut(&self) -> RefMut<Canvas<Window>> {
        self.canvas.borrow_mut()
    }
}

impl<'a> Renderer<'a> for SdlRenderer {
    type Texture = SdlTexture<'a>;

    fn load_texture(&'a self, path: &str) -> Self::Texture {
        self.texture_creator.load_texture(path).unwrap()
    }

    fn create_texture(&'a self, width: u32, height: u32, pixels: &[Color]) -> Self::Texture {
        let mut data = Vec::with_capacity(pixels.len() * 4);
        for pixel in pixels {
            data.push(pixel.r);
            data.push(pixel.g);
            data.push(pixel.b);
            data.push(pixel.a);
        }
        let surface = Surface::from_data(
            &mut data,
            width,
            height,
            width * 4,
            PixelFormatEnum::ABGR8888,
        );
        return surface.unwrap().as_texture(&self.texture_creator).unwrap();
    }

    fn clear_screen(&self) {
        self.canvas_mut()
            .set_draw_color(get_sdl_color(&RendererColor::Black));
        self.canvas_mut().clear();
    }

    fn highlight_selected_tile(&self, graphics: &Graphics, id: u32, color: &RendererColor) {
        let render_size = graphics.get_render_size();
        let render_multiplier = graphics.render_multiplier;
        let (x_logical, y_logical) = get_tile_coordinates(
            id,
            graphics.get_x_tiles_per_screen() * graphics.tile_size,
            graphics.tile_size,
        );
        let x = x_logical * render_multiplier;
        let y = y_logical * render_multiplier;
        self.draw_rect(
            &Rect::new(x as i32, y as i32, render_size, render_size),
            color,
        );
    }

    fn draw_rect(&self, rect: &Rect, color: &RendererColor) {
        let mut canvas = self.canvas_mut();
        canvas.set_draw_color(get_sdl_color(color));
        canvas
            .draw_line(rect.top_left(), rect.bottom_left())
            .unwrap();
        canvas.draw_line(rect.top_left(), rect.top_right()).unwrap();
        canvas
            .draw_line(rect.top_right(), rect.bottom_right())
            .unwrap();
        canvas
            .draw_line(rect.bottom_left(), rect.bottom_right())
            .unwrap();
    }

    fn draw_circle(&self, center: Point, radius: u32, color: &RendererColor) {
        let mut canvas = self.canvas_mut();
        canvas.set_draw_color(get_sdl_color(color));

        // https://stackoverflow.com/a/48291620
        let diameter = (radius * 2) as i32;
        let mut x = (radius - 1) as i32;
        let mut y = 0;
        let mut tx = 1;
        let mut ty = 1;
        let mut error = tx - diameter;

        while x >= y {
            for (cx, cy) in [
                (x, -y),
                (x, y),
                (-x, -y),
                (-x, y),
                (y, -x),
                (y, x),
                (-y, -x),
                (-y, x),
            ] {
                canvas
                    .draw_point(Point::new(center.x + cx, center.y + cy))
                    .unwrap();
            }

            if error <= 0 {
                y += 1;
                error += ty;
                ty += 2;
            }

            if error > 0 {
                x -= 1;
                tx += 2;
                error += tx - diameter;
            }
        }
    }

    fn render_texture(&self, texture: &Self::Texture, src: Option<Rect>, dst: Rect) {
        self.canvas_mut()
            .copy(texture, src.map(|r| r.into()), Some(dst.into()))
            .unwrap();
    }

    fn fill_and_render_texture(&self, color: RendererColor, texture: &Self::Texture, dst: Rect) {
        let mut canvas = self.canvas_mut();
        canvas.set_draw_color(get_sdl_color(&color));
        canvas.fill_rect(Some(dst.into())).unwrap();
        canvas.copy(texture, None, Some(dst.into())).unwrap();
    }

    fn get_texture_size(texture: &Self::Texture) -> (u32, u32) {
        let TextureQuery { width, height, .. } = texture.query();
        (width, height)
    }

    fn window_size(&self) -> (u32, u32) {
        self.canvas_mut().window().size()
    }
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
