use std::collections::HashSet;

use crate::index_vec::{GIndex, IndexVec};
use crate::input::mouse::Button;
use crate::input::Input;
use crate::math::*;
use crate::software_rendering::*;

#[derive(Default, Clone)]
pub struct Entity {
    pub tags: HashSet<String>,

    pub position: Vec2,
    pub velocity: Vec2,
    pub half_size: Vec2,

    pub collide_with: HashSet<String>,

    pub life: i32,
    pub color: Option<u32>,
}

impl Entity {
    pub fn new() -> Entity {
        Entity::default()
    }
}

#[derive(Default)]
pub struct Game {
    initialized: bool,
    arena_half_size: Vec2,

    levels: Vec<Box<dyn Level>>,
    current_level: usize,

    entities: IndexVec<Entity>,

    player: Option<GIndex>,
}

impl Game {
    pub fn new() -> Game {
        Game::default()
    }

    fn load_level(&mut self, level_index: usize) {
        if let Some(level) = self.levels.get(level_index) {
            self.entities.clear();

            self.arena_half_size = Vec2::new(85.0, 45.0);

            // arena
            {
                let mut left = Entity::new();
                left.tags.insert("Wall".to_string());
                left.position = Vec2::new(-self.arena_half_size.x - 1.0, 0.0);
                left.half_size = Vec2::new(1.0, self.arena_half_size.y);
                self.entities.insert(left);

                let mut right = Entity::new();
                right.tags.insert("Wall".to_string());
                right.position = Vec2::new(self.arena_half_size.x + 1.0, 0.0);
                right.half_size = Vec2::new(1.0, self.arena_half_size.y);
                self.entities.insert(right);

                let mut top = Entity::new();
                top.tags.insert("Wall".to_string());
                top.position = Vec2::new(0.0, self.arena_half_size.y + 1.0);
                top.half_size = Vec2::new(self.arena_half_size.x, 1.0);
                self.entities.insert(top);

                let mut bottom = Entity::new();
                bottom.tags.insert("Wall".to_string());
                bottom.tags.insert("DeadWall".to_string());
                bottom.position = Vec2::new(0.0, -self.arena_half_size.y - 1.0);
                bottom.half_size = Vec2::new(self.arena_half_size.x, 1.0);
                self.entities.insert(bottom);
            }

            // player
            {
                let mut player = Entity::new();
                player.tags.insert("Player".to_string());
                player.position.y = -40.0;
                player.half_size = Vec2::new(10.0, 2.0);
                player.color = Some(0x00ff00);
                self.player = Some(self.entities.insert(player));
            }

            level.load(&mut self.entities);

            self.current_level = level_index;
        }
    }

    pub fn simulate(&mut self, render_buffer: &mut RenderBuffer, input: &Input, dt: f32) {
        if !self.initialized {
            self.initialized = true;
            self.levels.push(Box::new(Level0 {}));
            self.levels.push(Box::new(Level1 {}));

            self.load_level(0);
        }

        // Player Controller
        {
            if let Some(player) = self.player.and_then(|player| self.entities.get_mut(player)) {
                let mouse_p = render_buffer.pixels_to_world(input.mouse.position);
                let new_player_p = Vec2::new(mouse_p.x, player.position.y);
                player.velocity = (new_player_p - player.position) / dt;
            }
        }

        {
            for entity in self.entities.iter_mut() {
                if entity.tags.contains("Ball") {
                    if entity.velocity.y < 0.0 {
                        entity.collide_with.insert("Player".to_string());
                    } else {
                        entity.collide_with.remove("Player");
                    }
                }
            }
        }

        // Common collision and movement
        {
            let mut collisions = Vec::new();
            for (index_a, a) in self.entities.iter().with_index() {
                if a.velocity.len2() == 0.0 {
                    continue;
                }

                let mut t = 1.0f32;
                let mut c = None;

                for (index_b, b) in self.entities.iter().with_index() {
                    if index_a == index_b {
                        continue;
                    }

                    let mut collided_with = false;

                    for tag in a.collide_with.iter() {
                        if b.tags.contains(tag) {
                            collided_with = true;
                            break;
                        }
                    }

                    if collided_with {
                        let movement =
                            Line2::new(a.position, a.position + (a.velocity - b.velocity) * dt);
                        if let Some(collision) =
                            swept_aabb2(&movement, a.half_size, b.position, b.half_size)
                        {
                            if movement.vec() * collision.normal <= 0.0 {
                                if collision.t < t {
                                    t = collision.t;
                                    c = Some((index_b, collision));
                                }
                            }
                        }
                    }
                }

                collisions.push((index_a, c));
            }

            for (index_a, c) in collisions.iter() {
                if let Some((index_b, collision)) = c {
                    if let (Some(a), Some(b)) = self.entities.get_two_mut(index_a, index_b) {
                        a.position = a.position + a.velocity * dt * collision.t;

                        if a.tags.contains("Ball") {
                            if b.tags.contains("Block") {
                                a.velocity = a.velocity.reflect(&collision.normal);
                                b.life -= 1;
                            } else if b.tags.contains("Wall") {
                                a.velocity = a.velocity.reflect(&collision.normal);
                                if b.tags.contains("DeadWall") {
                                    a.life -= 1;
                                }
                            } else if b.tags.contains("Player") {
                                a.position = a.position + a.velocity * dt * collision.t;

                                if collision.normal.x != 0.0 {
                                    a.velocity.y *= -1.0;
                                    if a.velocity * collision.normal <= 0.0 {
                                        a.velocity.x *= -1.0;
                                    }
                                } else {
                                    a.velocity = a.velocity.reflect(&collision.normal);
                                }
                                a.velocity.x = (a.position.x - b.position.x) * 7.5;
                            }
                        }
                    }
                } else if let Some(a) = self.entities.get_mut(index_a) {
                    a.position = a.position + a.velocity * dt;
                }
            }
        }

        // Remove block, ball
        {
            let mut to_remove_entities = Vec::new();
            for (index, entity) in self.entities.iter().with_index() {
                if (entity.tags.contains("Block") || entity.tags.contains("Ball"))
                    && entity.life == 0
                {
                    to_remove_entities.push(index);
                }
            }
            for index in to_remove_entities.iter() {
                self.entities.remove(index);
            }
        }

        // Level manager
        {
            let mut ball_count = 0;
            for entity in self.entities.iter() {
                if entity.tags.contains("Ball") {
                    ball_count += 1;
                }
            }

            if ball_count == 0 {
                self.load_level(self.current_level);
            }

            if input.mouse.button(Button::Left).pressed() {
                self.load_level(((self.current_level - 1) + self.levels.len()) % self.levels.len());
            } else if input.mouse.button(Button::Right).pressed() {
                self.load_level(((self.current_level + 1) + self.levels.len()) % self.levels.len());
            }
        }

        render_buffer.clear_and_draw_rect(Vec2::zero(), self.arena_half_size, 0x551100, 0x220500);

        for entity in self.entities.iter() {
            if let Some(color) = entity.color {
                render_buffer.draw_rect(entity.position, entity.half_size, color);
            }

            if entity.tags.contains("Ball") && entity.velocity.len2() > 0.0 {
                render_buffer.draw_line(
                    entity.position,
                    entity.position + entity.velocity.normalized() * 2.0,
                    0xff0000,
                );
            }
        }
    }
}

trait Level {
    fn load(&self, entities: &mut IndexVec<Entity>);
}

struct Level0 {}

impl Level for Level0 {
    fn load(&self, entities: &mut IndexVec<Entity>) {
        // ball
        {
            let mut ball = Entity::new();
            ball.tags.insert("Ball".to_string());
            ball.collide_with.insert("Wall".to_string());
            ball.collide_with.insert("Block".to_string());
            ball.half_size = Vec2::new(0.75, 0.75);
            ball.position.x = 60.0;
            ball.velocity.y = -40.0;
            ball.velocity.x = -30.0;
            ball.color = Some(0x00ffff);
            ball.life = 1;
            entities.insert(ball);
        }

        for y in 0..8 {
            for x in 0..8 {
                let mut block = Entity::new();
                block.tags.insert("Block".to_string());
                block.position = Vec2::new(x as f32 * 12.0 - 40.0, y as f32 * 5.0);
                block.half_size = Vec2::new(5.0, 2.0);
                block.color = Some(0x000000);
                block.life = 1;
                entities.insert(block);
            }
        }
    }
}

struct Level1 {}

impl Level for Level1 {
    fn load(&self, entities: &mut IndexVec<Entity>) {
        // ball
        {
            let mut ball = Entity::new();
            ball.tags.insert("Ball".to_string());
            ball.collide_with.insert("Wall".to_string());
            ball.collide_with.insert("Block".to_string());
            ball.half_size = Vec2::new(0.75, 0.75);
            ball.position.x = 60.0;
            ball.velocity.y = -40.0;
            ball.velocity.x = -30.0;
            ball.color = Some(0x00ffff);
            ball.life = 1;
            entities.insert(ball);

            let mut ball = Entity::new();
            ball.tags.insert("Ball".to_string());
            ball.collide_with.insert("Wall".to_string());
            ball.collide_with.insert("Block".to_string());
            ball.half_size = Vec2::new(0.75, 0.75);
            ball.position.x = -60.0;
            ball.velocity.y = -40.0;
            ball.velocity.x = 30.0;
            ball.color = Some(0x00ffff);
            ball.life = 1;
            entities.insert(ball);
        }

        // blocks
        for y in 0..8 {
            for x in 0..8 {
                let mut block = Entity::new();
                block.tags.insert("Block".to_string());
                block.position = Vec2::new(x as f32 * 12.0 - 40.0, y as f32 * 5.0);
                block.half_size = Vec2::new(5.0, 2.0);
                block.color = Some(0x000000);
                block.life = 1;
                entities.insert(block);
            }
        }
    }
}
