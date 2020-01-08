use crate::math::{clamp, Vec2};

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
        let mut aspect_multiplier = self.height as f32;
        if (self.width as f32 / self.height as f32) < 1.77 {
            aspect_multiplier = self.width as f32 / 1.77;
        }

        let scale = 0.01;
        half_size.x *= aspect_multiplier * scale;
        half_size.y *= aspect_multiplier * scale;

        p.x *= aspect_multiplier * scale;
        p.y *= aspect_multiplier * scale;

        p.x += self.width as f32 * 0.5;
        p.y += self.height as f32 * 0.5;

        let x0 = (p.x - half_size.x) as i32;
        let y0 = (p.y - half_size.y) as i32;
        let x1 = (p.x + half_size.x) as i32;
        let y1 = (p.y + half_size.y) as i32;

        self.draw_rect_in_pixels(x0, y0, x1, y1, color);
    }
}
