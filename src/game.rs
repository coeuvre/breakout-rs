use crate::math::Vec2;
use crate::software_rendering::RenderBuffer;

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
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub buttons: [ButtonState; ButtonType::COUNT as usize],
}

impl Input {
    pub fn new() -> Input {
        Input {
            mouse_x: 0,
            mouse_y: 0,
            buttons: [ButtonState {
                is_down: false,
                changed: false,
            }; ButtonType::COUNT as usize],
        }
    }
}

pub struct Game {
    player_p: Vec2,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player_p: Vec2::new(0.0, 0.0),
        }
    }
    pub fn simulate(&mut self, render_buffer: &mut RenderBuffer, input: &Input, dt: f32) {
        let speed = 100.0;

        if input.buttons[ButtonType::LEFT as usize].is_down() {
            self.player_p.x -= speed * dt;
        }
        if input.buttons[ButtonType::RIGHT as usize].is_down() {
            self.player_p.x += speed * dt;
        }
        if input.buttons[ButtonType::UP as usize].is_down() {
            self.player_p.y += speed * dt;
        }
        if input.buttons[ButtonType::DOWN as usize].is_down() {
            self.player_p.y -= speed * dt;
        }

        render_buffer.clear(0x551100);
        render_buffer.draw_rect(self.player_p, Vec2::new(1.0, 1.0), 0xffff00);
    }
}
