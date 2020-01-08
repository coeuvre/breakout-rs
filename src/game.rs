use crate::math::*;
use crate::software_rendering::*;

#[derive(Copy, Clone)]
pub struct ButtonState {
    pub is_down: bool,
    pub changed: bool,
}

impl ButtonState {
    pub fn is_down(&self) -> bool {
        self.is_down
    }

    pub fn pressed(&self) -> bool {
        self.is_down && self.changed
    }

    pub fn released(&self) -> bool {
        !self.is_down && self.changed
    }
}

pub enum ButtonType {
    LEFT,
    RIGHT,
    UP,
    DOWN,

    COUNT,
}

pub struct Input {
    pub mouse: Vec2,
    pub buttons: [ButtonState; ButtonType::COUNT as usize],
}

impl Input {
    pub fn new() -> Input {
        Input {
            mouse: Vec2::new(0.0, 0.0),
            buttons: [ButtonState {
                is_down: false,
                changed: false,
            }; ButtonType::COUNT as usize],
        }
    }
}

pub struct Game {
    initialized: bool,
    player_p: Vec2,
    player_half_size: Vec2,
    player_dp: Vec2,
    ball_p: Vec2,
    ball_half_size: Vec2,
    ball_dp: Vec2,
}

impl Game {
    pub fn new() -> Game {
        Game {
            initialized: false,
            player_p: Vec2::zero(),
            player_half_size: Vec2::zero(),
            player_dp: Vec2::zero(),
            ball_p: Vec2::zero(),
            ball_half_size: Vec2::zero(),
            ball_dp: Vec2::zero(),
        }
    }

    pub fn simulate(&mut self, render_buffer: &mut RenderBuffer, input: &Input, dt: f32) {
        if !self.initialized {
            self.initialized = true;

            self.player_p.y = -40.0;
            self.player_half_size = Vec2::new(10.0, 2.0);

            self.ball_half_size = Vec2::new(0.75, 0.75);
            self.ball_p.y = 40.0;
            self.ball_dp.y = -40.0;
        }

        let player_new_x = render_buffer.pixels_to_world(input.mouse).x;
        self.player_dp.x = (player_new_x - self.player_p.x) / dt;
        self.player_p.x = player_new_x;

        self.ball_p = self.ball_p + self.ball_dp * dt;

        if self.ball_dp.y < 0.0
            && aabb_vs_aabb(
                self.player_p,
                self.player_half_size,
                self.ball_p,
                self.ball_half_size,
            )
        {
            self.ball_dp.y *= -1.0;
            self.ball_dp.x += self.player_dp.x;
        }

        render_buffer.clear(0x551100);
        render_buffer.draw_rect(self.ball_p, self.ball_half_size, 0x00ffff);
        render_buffer.draw_rect(self.player_p, self.player_half_size, 0x00ff00);
    }
}
