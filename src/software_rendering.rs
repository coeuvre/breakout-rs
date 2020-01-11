use crate::line::line_iter;
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

    pub fn draw_line_in_pixels(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        let x0 = clamp(0, x0, self.width - 1);
        let x1 = clamp(0, x1, self.width - 1);
        let y0 = clamp(0, y0, self.height - 1);
        let y1 = clamp(0, y1, self.height - 1);

        for p in line_iter(x0, y0, x1, y1) {
            self.pixels[(p.y * self.width + p.x) as usize] = color;
        }
    }

    pub fn draw_rect_in_pixels(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        let x0 = clamp(0, x0, self.width);
        let x1 = clamp(0, x1, self.width);
        let y0 = clamp(0, y0, self.height);
        let y1 = clamp(0, y1, self.height);

        for y in y0..y1 {
            for x in x0..x1 {
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

    pub fn draw_line(&mut self, mut start: Vec2, mut end: Vec2, color: u32) {
        let aspect_multiplier = self.calc_aspect_multiplier();

        start.x *= aspect_multiplier * SCALE;
        start.y *= aspect_multiplier * SCALE;
        start.x += self.width as f32 * 0.5;
        start.y += self.height as f32 * 0.5;

        end.x *= aspect_multiplier * SCALE;
        end.y *= aspect_multiplier * SCALE;
        end.x += self.width as f32 * 0.5;
        end.y += self.height as f32 * 0.5;

        let x0 = start.x as i32;
        let y0 = start.y as i32;
        let x1 = end.x as i32;
        let y1 = end.y as i32;

        self.draw_line_in_pixels(x0, y0, x1, y1, color);
    }

    pub fn clear_and_draw_rect(
        &mut self,
        mut p: Vec2,
        mut half_size: Vec2,
        color: u32,
        clear_color: u32,
    ) {
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

        self.draw_rect_in_pixels(0, 0, x0, self.height, clear_color);
        self.draw_rect_in_pixels(x1, 0, self.width, self.height, clear_color);
        self.draw_rect_in_pixels(x0, 0, x1, y0, clear_color);
        self.draw_rect_in_pixels(x0, y1, x1, self.height, clear_color);
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
