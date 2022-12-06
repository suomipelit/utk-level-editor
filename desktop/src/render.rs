use common::render::{Color, Point, Rect, Renderer, RendererColor, Texture};
use sdl2::image::LoadTexture;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureQuery;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};
use std::collections::HashMap;

pub struct SdlTexture {
    width: u32,
    height: u32,
    index: usize,
}

impl Texture for SdlTexture {
    fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

pub struct SdlRenderer<'a> {
    canvas: &'a mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    textures: HashMap<usize, sdl2::render::Texture<'a>>,
    texture_index: usize,
}

impl<'a> SdlRenderer<'a> {
    pub fn new(
        canvas: &'a mut Canvas<Window>,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Self {
        Self {
            canvas,
            texture_creator,
            textures: HashMap::new(),
            texture_index: 0,
        }
    }

    fn add_texture(&mut self, texture: sdl2::render::Texture<'a>) -> usize {
        let index = self.texture_index;
        self.textures.insert(index, texture);
        self.texture_index += 1;
        index
    }

    pub fn load_texture(&mut self, path: &str) -> SdlTexture {
        let texture = self.texture_creator.load_texture(path).unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        SdlTexture {
            width,
            height,
            index: self.add_texture(texture),
        }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}

impl<'a> Renderer for SdlRenderer<'a> {
    type Texture = SdlTexture;

    fn create_texture(&mut self, width: u32, height: u32, pixels: &[Color]) -> Self::Texture {
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
        let texture = surface.unwrap().as_texture(self.texture_creator).unwrap();
        SdlTexture {
            width,
            height,
            index: self.add_texture(texture),
        }
    }

    fn clear_screen(&mut self) {
        self.canvas
            .set_draw_color(to_sdl_color(&RendererColor::Black));
        self.canvas.clear();
    }

    fn draw_rect(&mut self, rect: &Rect, color: &RendererColor) {
        self.canvas.set_draw_color(to_sdl_color(color));
        self.canvas
            .draw_line(
                to_sdl_point(rect.top_left()),
                to_sdl_point(rect.bottom_left()),
            )
            .unwrap();
        self.canvas
            .draw_line(
                to_sdl_point(rect.top_left()),
                to_sdl_point(rect.top_right()),
            )
            .unwrap();
        self.canvas
            .draw_line(
                to_sdl_point(rect.top_right()),
                to_sdl_point(rect.bottom_right()),
            )
            .unwrap();
        self.canvas
            .draw_line(
                to_sdl_point(rect.bottom_left()),
                to_sdl_point(rect.bottom_right()),
            )
            .unwrap();
    }

    fn draw_circle(&mut self, center: Point, radius: u32, color: &RendererColor) {
        self.canvas.set_draw_color(to_sdl_color(color));

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
                self.canvas
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

    fn render_texture(&mut self, texture: &Self::Texture, src: Option<Rect>, dst: Rect) {
        let t = self.textures.get(&texture.index).unwrap();
        self.canvas
            .copy(t, src.map(to_sdl_rect), Some(to_sdl_rect(dst)))
            .unwrap();
    }

    fn fill_and_render_texture(
        &mut self,
        color: RendererColor,
        texture: &Self::Texture,
        dst: Rect,
    ) {
        self.canvas.set_draw_color(to_sdl_color(&color));
        self.canvas.fill_rect(Some(to_sdl_rect(dst))).unwrap();

        let t = self.textures.get(&texture.index).unwrap();
        self.canvas.copy(t, None, Some(to_sdl_rect(dst))).unwrap();
    }

    fn window_size(&self) -> (u32, u32) {
        self.canvas.window().size()
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
