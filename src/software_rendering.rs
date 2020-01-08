use crate::math::{clamp, Vec2};

static SCALE: f32 = 0.01;

pub struct RenderBuffer<'a> {
    pub pixels: &'a mut [u32],
    pub width: i32,
    pub height: i32,
}

impl<'a> RenderBuffer<'a> {
    pub fn clear(&mut self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.pixels[(y * self.width + x) as usize] = color;
            }
        }
    }

    pub fn draw_rect_in_pixels(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        let x0 = clamp(0, x0, self.width - 1);
        let x1 = clamp(0, x1, self.width - 1);
        let y0 = clamp(0, y0, self.height - 1);
        let y1 = clamp(0, y1, self.height - 1);

        for y in y0..=y1 {
            for x in x0..=x1 {
                self.pixels[(y * self.width + x) as usize] = color;
            }
        }
    }

    pub fn draw_rect(&mut self, mut p: Vec2, mut half_size: Vec2, color: u32) {
        let aspect_multiplier = self.calc_aspect_multiplier();

        half_size.x *= aspect_multiplier * SCALE;
        half_size.y *= aspect_multiplier * SCALE;

        p.x *= aspect_multiplier * SCALE;
        p.y *= aspect_multiplier * SCALE;

        p.x += self.width as f32 * 0.5;
        p.y += self.height as f32 * 0.5;

        let x0 = (p.x - half_size.x) as i32;
        let y0 = (p.y - half_size.y) as i32;
        let x1 = (p.x + half_size.x) as i32;
        let y1 = (p.y + half_size.y) as i32;

        self.draw_rect_in_pixels(x0, y0, x1, y1, color);
    }

    pub fn pixels_to_world(&self, pixels: Vec2) -> Vec2 {
        let aspect_multiplier = self.calc_aspect_multiplier();

        let mut result = Vec2::zero();
        result.x = pixels.x as f32 - self.width as f32 * 0.5;
        result.y = pixels.y as f32 - self.height as f32 * 0.5;

        result.x /= aspect_multiplier * SCALE;
        result.y /= aspect_multiplier * SCALE;

        result
    }

    fn calc_aspect_multiplier(&self) -> f32 {
        if (self.width as f32 / self.height as f32) < 1.77 {
            self.width as f32 / 1.77
        } else {
            self.height as f32
        }
    }
}
