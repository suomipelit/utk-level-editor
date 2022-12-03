use common::graphics::Graphics;
use common::render::{Color, Point, Rect, Renderer, RendererColor};
use common::util::get_tile_coordinates;
use sdl2::image::LoadTexture;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Texture as SdlTexture;
use sdl2::render::TextureQuery;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};
use std::cell::RefCell;

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
        self.canvas.borrow_mut().present();
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
        let mut canvas = self.canvas.borrow_mut();
        canvas.set_draw_color(to_sdl_color(&RendererColor::Black));
        canvas.clear();
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
        let mut canvas = self.canvas.borrow_mut();
        canvas.set_draw_color(to_sdl_color(color));
        canvas
            .draw_line(
                to_sdl_point(rect.top_left()),
                to_sdl_point(rect.bottom_left()),
            )
            .unwrap();
        canvas
            .draw_line(
                to_sdl_point(rect.top_left()),
                to_sdl_point(rect.top_right()),
            )
            .unwrap();
        canvas
            .draw_line(
                to_sdl_point(rect.top_right()),
                to_sdl_point(rect.bottom_right()),
            )
            .unwrap();
        canvas
            .draw_line(
                to_sdl_point(rect.bottom_left()),
                to_sdl_point(rect.bottom_right()),
            )
            .unwrap();
    }

    fn draw_circle(&self, center: Point, radius: u32, color: &RendererColor) {
        let mut canvas = self.canvas.borrow_mut();
        canvas.set_draw_color(to_sdl_color(color));

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
                    .draw_point(sdl2::rect::Point::new(center.x + cx, center.y + cy))
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
        self.canvas
            .borrow_mut()
            .copy(texture, src.map(to_sdl_rect), Some(to_sdl_rect(dst)))
            .unwrap();
    }

    fn fill_and_render_texture(&self, color: RendererColor, texture: &Self::Texture, dst: Rect) {
        let mut canvas = self.canvas.borrow_mut();
        canvas.set_draw_color(to_sdl_color(&color));
        canvas.fill_rect(Some(to_sdl_rect(dst))).unwrap();
        canvas.copy(texture, None, Some(to_sdl_rect(dst))).unwrap();
    }

    fn get_texture_size(texture: &Self::Texture) -> (u32, u32) {
        let TextureQuery { width, height, .. } = texture.query();
        (width, height)
    }

    fn window_size(&self) -> (u32, u32) {
        self.canvas.borrow_mut().window().size()
    }
}

fn to_sdl_color(render_color: &RendererColor) -> sdl2::pixels::Color {
    let color = render_color.to_color();
    sdl2::pixels::Color::RGB(color.r, color.g, color.b)
}

fn to_sdl_point(point: Point) -> sdl2::rect::Point {
    sdl2::rect::Point::new(point.x, point.y)
}

fn to_sdl_rect(rect: Rect) -> sdl2::rect::Rect {
    sdl2::rect::Rect::new(rect.x, rect.y, rect.width, rect.height)
}
