use common::render::{Color, Point, Rect, Renderer, RendererColor, Texture};

pub struct CanvasTexture {
    width: u32,
    height: u32,
    pixels: Vec<u32>,
}

impl Texture for CanvasTexture {
    fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

pub struct CanvasRenderer {
    width: u32,
    height: u32,
    screen: Vec<u32>,
}

impl CanvasRenderer {
    pub fn new(width: u32, height: u32) -> CanvasRenderer {
        CanvasRenderer {
            width,
            height,
            screen: vec![0; (width * height * 4) as usize],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> *const u32 {
        self.screen.as_ptr()
    }

    pub fn create_texture_rgba(&self, width: u32, height: u32, data: &[u8]) -> CanvasTexture {
        CanvasTexture {
            width,
            height,
            pixels: data
                .chunks(4)
                .map(|c| {
                    let r = c[0] as u32;
                    let g = c[1] as u32;
                    let b = c[2] as u32;
                    let a = c[3] as u32;
                    (a << 24) | (b << 16) | (g << 8) | r
                })
                .collect(),
        }
    }

    fn draw_horizontal_line(&mut self, y: i32, x1: i32, x2: i32, color: Color) {
        let c = color.to_u32();
        let (width, height) = (self.width as i32, self.height as i32);
        if y < 0 || y >= height {
            return;
        }
        if x1 > x2 {
            return self.draw_horizontal_line(y, x2, x1, color);
        }
        if x1 >= width || x2 < 0 {
            return;
        }
        let (x1, x2) = (x1.max(0), x2.min(width - 1));
        self.screen
            .iter_mut()
            .skip((y * width + x1) as usize)
            .take((x2 - x1 + 1) as usize)
            .for_each(|p| {
                *p = c;
            });
    }

    fn draw_vertical_line(&mut self, x: i32, y1: i32, y2: i32, color: Color) {
        let c = color.to_u32();
        let (width, height) = (self.width as i32, self.height as i32);
        if x < 0 || x >= width {
            return;
        }
        if y1 > y2 {
            return self.draw_horizontal_line(x, y2, y1, color);
        }
        if y1 >= height || y2 < 0 {
            return;
        }
        let (y1, y2) = (y1.max(0), y2.min(height - 1));
        let mut index = (y1 * width + x) as usize;
        for _ in y1..=y2 {
            self.screen[index] = c;
            index += width as usize;
        }
    }
}

impl Renderer for CanvasRenderer {
    type Texture = CanvasTexture;

    fn create_texture(&mut self, width: u32, height: u32, data: &[Color]) -> Self::Texture {
        let mut pixels = vec![255 << 24; (width * height) as usize];
        for (i, pixel) in data.iter().enumerate() {
            pixels[i] = pixel.to_u32();
        }
        CanvasTexture {
            width,
            height,
            pixels,
        }
    }

    fn clear_screen(&mut self) {
        let c = Color::from((0, 0, 0, 255)).to_u32();
        self.screen.fill(c);
    }

    fn draw_rect(&mut self, rect: &Rect, color: &RendererColor) {
        let color = color.to_color();
        self.draw_horizontal_line(rect.y, rect.x, rect.x + rect.width as i32 - 1, color);
        self.draw_horizontal_line(
            rect.y + rect.height as i32 - 1,
            rect.x,
            rect.x + rect.width as i32 - 1,
            color,
        );
        self.draw_vertical_line(rect.x, rect.y, rect.y + rect.height as i32 - 1, color);
        self.draw_vertical_line(
            rect.x + rect.width as i32 - 1,
            rect.y,
            rect.y + rect.height as i32 - 1,
            color,
        );
    }

    fn draw_circle(&mut self, center: Point, radius: u32, color: &RendererColor) {
        // https://stackoverflow.com/a/48291620
        let c = color.to_color().to_u32();
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
                let index =
                    ((center.y as i32 + cy) * self.width as i32 + center.x as i32 + cx) as usize;
                self.screen[index] = c;
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
        let (src_x, src_y, src_width, src_height) = match src {
            Some(src) => (src.x, src.y, src.width, src.height),
            None => (0, 0, texture.width, texture.height),
        };
        for y in 0..src_height as i32 {
            let sy = src_y + y;
            if sy < 0 || sy >= texture.height as i32 {
                continue;
            }
            let dy = dst.y + y;
            if dy < 0 || dy >= self.height as i32 {
                continue;
            }
            let si_start = (sy * texture.width as i32) as usize;
            let di_start = (dy * self.width as i32) as usize;
            for x in 0..src_width as i32 {
                let sx = src_x + x;
                if sx < 0 || sx >= texture.width as i32 {
                    continue;
                }
                let dx = dst.x + x;
                if dx < 0 || dx >= self.width as i32 {
                    continue;
                }

                let si = si_start + sx as usize;
                let di = di_start + dx as usize;

                let alpha = texture.pixels[si] & 0xff000000;
                if alpha == 0 {
                    continue;
                } else if alpha == 0xff000000 {
                    self.screen[di] = texture.pixels[si];
                } else {
                    // Alpha blending not implemented (or needed)
                }
            }
        }
    }

    fn fill_and_render_texture(
        &mut self,
        _color: RendererColor,
        texture: &Self::Texture,
        dst: Rect,
    ) {
        self.render_texture(texture, None, dst);
    }

    fn window_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
