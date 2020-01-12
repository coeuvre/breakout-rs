use crate::input::Input;
use crate::math::*;
use crate::software_rendering::*;

#[derive(Copy, Clone)]
struct Block {
    center: Vec2,
    radius: Vec2,
    life: i32,
}

impl Block {
    pub fn new() -> Block {
        Block {
            center: Vec2::zero(),
            radius: Vec2::zero(),
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
            blocks: Vec::new(),
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
            self.ball_p.x = 40.0;
            self.ball_dp.y = -40.0;
            self.ball_dp.x = -10.0;

            for y in 0..8 {
                for x in 0..8 {
                    let mut block = Block::new();
                    block.center = Vec2::new(x as f32 * 12.0 - 40.0, y as f32 * 5.0);
                    block.radius = Vec2::new(5.0, 2.0);
                    block.life = 1;
                    self.blocks.push(block);
                }
            }
        }

        let mouse_p = render_buffer.pixels_to_world(input.mouse.position);

        let new_player_p = Vec2::new(mouse_p.x, self.player_p.y);
        let mut player_movement = Line2::new(self.player_p, new_player_p);
        let player_dp = (player_movement.end - player_movement.start) / dt;

        let mut ball_movement = Line2::new(self.ball_p, self.ball_p + (self.ball_dp) * dt);

        let mut ball_relative_player_movement = Line2::new(
            self.ball_p,
            self.ball_p + ball_movement.vec() - player_movement.vec(),
        );

        // ball vs player
        if self.ball_dp.y < 0.0 {
            if let Some(collision) = swept_aabb2(
                &ball_relative_player_movement,
                self.ball_half_size,
                self.player_p,
                self.player_half_size,
            ) {
                if ball_relative_player_movement.vec() * collision.normal <= 0.0 {
                    if collision.normal.x != 0.0 {
                        self.ball_dp.y *= -1.0;
                        if self.ball_dp * collision.normal <= 0.0 {
                            self.ball_dp.x *= -1.0;
                        }
                    } else {
                        self.ball_dp = self.ball_dp.reflect(&collision.normal);
                    }
                    self.ball_dp.x += player_dp.x * 0.1;
                    ball_movement.truncate(collision.t);
                    player_movement.truncate(collision.t);
                }
            }
        }

        self.player_p = player_movement.end;
        self.player_dp = player_dp;

        // ball vs blocks
        for block in self.blocks.iter_mut() {
            if let Some(collision) = swept_aabb2(
                &ball_movement,
                self.ball_half_size,
                block.center,
                block.radius,
            ) {
                if ball_movement.vec() * collision.normal <= 0.0 {
                    ball_movement.truncate(collision.t);
                    self.ball_dp = self.ball_dp.reflect(&collision.normal);
                    block.life -= 1;
                }
            }
        }

        self.blocks.retain(|block| block.life > 0);

        // ball vs arena
        if let Some(collision) = swept_aabb2(
            &ball_movement,
            -self.ball_half_size,
            Vec2::zero(),
            self.arena_half_size,
        ) {
            if (ball_movement.end - ball_movement.start) * collision.normal >= 0.0 {
                ball_movement.truncate(collision.t);
                self.ball_dp = self.ball_dp.reflect(&collision.normal);
            }
        }

        self.ball_p = ball_movement.end;

        render_buffer.clear_and_draw_rect(Vec2::zero(), self.arena_half_size, 0x551100, 0x220500);

        for block in self.blocks.iter() {
            render_buffer.draw_rect(block.center, block.radius, 0x000000);
        }
        render_buffer.draw_rect(self.player_p, self.player_half_size, 0x00ff00);
        render_buffer.draw_rect(self.ball_p, self.ball_half_size, 0x00ffff);
        render_buffer.draw_line(
            self.ball_p,
            self.ball_p + self.ball_dp.normalized() * 2.0,
            0x00ffff,
        );
    }
}
