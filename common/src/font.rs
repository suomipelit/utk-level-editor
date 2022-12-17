use crate::fn2::{Character, FN2};
use crate::render::{Color, Rect, Renderer};
use std::cmp::max;

const INDEX_OFFSET: usize = 33;
const SPACE_WIDTH: u32 = 5;
const TEXT_SHADOW_PIXELS: u32 = 1;

struct Glyph<T> {
    width: u32,
    height: u32,
    texture: T,
}

pub struct Font<T> {
    pub text_size_multiplier: u32,
    glyphs: Vec<Glyph<T>>,
    line_height: u32,
}

impl<T> Font<T> {
    pub fn new<R: Renderer<Texture = T>>(
        renderer: &mut R,
        fn2: &FN2,
        text_size_multiplier: u32,
    ) -> Self {
        let glyphs = fn2
            .characters
            .iter()
            .map(|character| Glyph {
                width: character.width + 1,
                height: character.height + 1,
                texture: Self::create_glyph_texture(renderer, character),
            })
            .collect::<Vec<_>>();
        let line_height = glyphs[0].height;
        Self {
            text_size_multiplier,
            glyphs,
            line_height,
        }
    }

    fn create_glyph_texture<R: Renderer<Texture = T>>(
        renderer: &mut R,
        character: &Character,
    ) -> T {
        let char_width = character.width;
        let char_height = character.height;
        let glyph_width = char_width + TEXT_SHADOW_PIXELS;
        let glyph_height = char_height + TEXT_SHADOW_PIXELS;

        let mut bitmap = vec![false; (char_width * char_height) as usize];
        let mut pixels = vec![Color::from((0, 0, 0, 0)); (glyph_width * glyph_height) as usize];

        for line in &character.lines {
            for x in 0..line.width {
                bitmap[(line.y * (character.width as u8) + line.x + x) as usize] = true;
            }
        }

        // Shadow
        for sy in 0..char_height {
            for sx in 0..char_width {
                let bit = bitmap[(sy * char_width + sx) as usize];
                if bit {
                    let ty = sy + TEXT_SHADOW_PIXELS;
                    let tx = sx + TEXT_SHADOW_PIXELS;
                    pixels[(ty * glyph_width + tx) as usize] = Color::from((0, 0, 0, 255));
                }
            }
        }

        // Actual glyph
        for y in 0..character.height {
            for x in 0..character.width {
                let bit = bitmap[(y * char_width + x) as usize];
                if bit {
                    pixels[(y * glyph_width + x) as usize] = Color::from((255, 0, 0, 255));
                }
            }
        }

        renderer.create_texture(glyph_width, glyph_height, &pixels)
    }

    pub fn line_height(&self) -> u32 {
        self.line_height * self.text_size_multiplier
    }

    pub fn render_text_relative<R: Renderer<Texture = T>>(
        &self,
        renderer: &mut R,
        text: &str,
        origo: (i32, i32),
        pos: (u32, u32),
    ) {
        let mut x = origo.0 + pos.0 as i32;
        let y = origo.1 + pos.1 as i32;
        for c in text.chars() {
            let c = c as usize;
            if c < INDEX_OFFSET {
                x += (SPACE_WIDTH * self.text_size_multiplier) as i32;
            } else {
                let glyph = &self.glyphs[c - INDEX_OFFSET];
                renderer.render_texture(
                    &glyph.texture,
                    None,
                    Rect::new(
                        x,
                        y,
                        glyph.width * self.text_size_multiplier,
                        glyph.height * self.text_size_multiplier,
                    ),
                );
                x += (glyph.width * self.text_size_multiplier) as i32;
            }
        }
    }

    pub fn render_text<R: Renderer<Texture = T>>(
        &self,
        renderer: &mut R,
        text: &str,
        pos: (u32, u32),
    ) {
        self.render_text_relative(renderer, text, (0, 0), pos);
    }

    pub fn text_size(&self, text: &str) -> (u32, u32) {
        let mut x = 0;
        let mut y = 0;
        for c in text.chars() {
            let c = c as usize;
            if c < INDEX_OFFSET {
                x += SPACE_WIDTH * self.text_size_multiplier;
            } else {
                let glyph = &self.glyphs[c - INDEX_OFFSET];
                x += glyph.width * self.text_size_multiplier;
                y = max(y, glyph.height * self.text_size_multiplier);
            }
        }
        (x, y)
    }
}
