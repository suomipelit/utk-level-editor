use crate::level::TILE_SIZE;

pub struct Graphics {
    pub render_multiplier: u32,
    pub resolution_x: u32,
    pub resolution_y: u32,
}

impl Graphics {
    pub fn new(resolution: (u32, u32), render_multiplier: u32) -> Graphics {
        Graphics {
            render_multiplier,
            resolution_x: resolution.0,
            resolution_y: resolution.1,
        }
    }

    pub fn get_render_size(&self) -> u32 {
        TILE_SIZE * self.render_multiplier
    }

    pub fn get_x_tiles_per_screen(&self) -> u32 {
        (self.resolution_x + self.get_render_size() - 1) / self.get_render_size()
    }

    pub fn get_full_x_tiles_per_screen(&self) -> u32 {
        self.resolution_x / self.get_render_size()
    }

    pub fn get_y_tiles_per_screen(&self) -> u32 {
        (self.resolution_y + self.get_render_size() - 1) / self.get_render_size()
    }

    pub fn get_full_y_tiles_per_screen(&self) -> u32 {
        self.resolution_y / self.get_render_size()
    }
}
