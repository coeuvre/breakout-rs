use crate::input::mouse::Button;
use crate::input::Input;
use crate::math::*;
use crate::software_rendering::*;

#[derive(Copy, Clone)]
struct Block {
    p: Vec2,
    block_size: Vec2,
    life: i32,
}

impl Block {
    pub fn new() -> Block {
        Block {
            p: Vec2::zero(),
            block_size: Vec2::zero(),
            life: 0,
        }
    }
}

pub struct Game {
    initialized: bool,
    blocks: Vec<Block>,
    arena_half_size: Vec2,
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
            blocks: vec![Block::new(); 64],
            arena_half_size: Vec2::zero(),
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

            self.arena_half_size = Vec2::new(85.0, 45.0);

            self.player_p.y = -40.0;
            self.player_half_size = Vec2::new(10.0, 2.0);

            self.ball_half_size = Vec2::new(0.75, 0.75);
            self.ball_p.y = 40.0;
            self.ball_dp.y = -40.0;
        }

        let mut mouse_p = render_buffer.pixels_to_world(input.mouse.position);

        let mut collided = false;

        //        let new_player_p = Vec2::new(mouse_p.x, self.player_p.y);
        let new_player_p = if input.mouse.button(Button::Left).is_down() {
            mouse_p
        } else {
            self.player_p
        };
        let player_movement = Line2::new(self.player_p, new_player_p);

        let mut ball_movement = Line2::new(self.ball_p, self.ball_p + (self.ball_dp) * dt);

        // ball vs player
        if let Some(collision) = swept_aabb2(
            &ball_movement,
            self.ball_half_size,
            self.player_p,
            self.player_half_size,
        ) {
            if (ball_movement.end - ball_movement.start) * collision.normal < 0.0 {
                ball_movement.truncate(collision.t);
                self.ball_dp = self.ball_dp.reflect(&collision.normal);
            }
        }

        self.player_p = player_movement.end;
        self.player_dp = (player_movement.end - player_movement.start) / dt;

        // ball vs arena
        if let Some(collision) = swept_aabb2(
            &ball_movement,
            -self.ball_half_size,
            Vec2::zero(),
            self.arena_half_size,
        ) {
            if (ball_movement.end - ball_movement.start) * collision.normal > 0.0 {
                ball_movement.truncate(collision.t);
                self.ball_dp = self.ball_dp.reflect(&collision.normal);
            }
        }

        self.ball_p = ball_movement.end;

        render_buffer.clear_and_draw_rect(Vec2::zero(), self.arena_half_size, 0x551100, 0x220500);
        render_buffer.draw_rect(self.ball_p, self.ball_half_size, 0x00ffff);
        if collided {
            render_buffer.draw_rect(self.player_p, self.player_half_size, 0xff0000);
        } else {
            render_buffer.draw_rect(self.player_p, self.player_half_size, 0x00ff00);
        }
    }
}
