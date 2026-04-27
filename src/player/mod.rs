use std::collections::HashMap;

use raylib::color::Color;
use raylib::ffi::KeyboardKey;
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::{RaylibDraw, RaylibDrawHandle};

use crate::collision::AABB;
use crate::game::context::Context;
use crate::graphic::animation::Animation;
use crate::graphic::frame::Atlas;

const MAX_SPEED: f32 = 180.0;
const ACCELERATION: f32 = 1000.0;
const FRICTION: f32 = 600.0;

#[derive(PartialEq, Eq, Hash)]
pub enum PlayerState {
    IDLE, RUN,
}

pub struct PlayerConfig {
    pub start_position: Vector2,
    pub speed: f32
}

pub struct Player {
    animations: HashMap<PlayerState, Animation>,
    current_state: PlayerState,
    pub hitbox: AABB,
    pub input_dir: f32,
    pub position: Vector2,
    pub velocity: f32,
    pub facing: f32
}

impl Player {
    pub fn load(config: PlayerConfig, ctx: &mut Context) -> Self {
        let mut animations = HashMap::new();

        let idle_atlas = Atlas::from("assets/sprite-sheet/player/idle/sheet.json");
        let run_atlas = Atlas::from("assets/sprite-sheet/player/run/sheet.json");

        animations.insert(PlayerState::IDLE, Animation::load(
            ctx, "assets/sprite-sheet/player/idle/sheet.png",
            idle_atlas.get_frame_size().0,
            idle_atlas.get_frame_size().1,
            idle_atlas.count(), 12.0,
        ));
        animations.insert(PlayerState::RUN, Animation::load(
            ctx, "assets/sprite-sheet/player/run/sheet.png",
            run_atlas.get_frame_size().0,
            run_atlas.get_frame_size().1,
            run_atlas.count(), 12.0,
        ));
        let hitbox = AABB::new(9.0,4.0,40.0,54.0);
        Self {
            animations: animations,
            current_state: PlayerState::IDLE,
            hitbox: hitbox,
            input_dir: 0.0,
            position: config.start_position,
            velocity: 0.0,
            facing: 1.0
        }
    }

    pub fn handle_input(&mut self, ctx: &mut Context) {
        self.input_dir= 0.0;
        if ctx.rl.is_key_down(KeyboardKey::KEY_D) { self.input_dir += 1.0; }
        if ctx.rl.is_key_down(KeyboardKey::KEY_A) { self.input_dir -= 1.0; }
    }

    pub fn update(&mut self, dt: &f32) {
        if self.input_dir != 0.0 {
            self.velocity += self.input_dir * ACCELERATION * dt;
            self.velocity = self.velocity.clamp(-MAX_SPEED, MAX_SPEED);
            self.facing = self.input_dir.signum();
        } else {
            let friction = FRICTION * dt;
            if self.velocity.abs() <= friction {
                self.velocity = 0.0;
            } else {
                self.velocity -= self.velocity.signum() * friction;
            }
        }
        self.position.x += self.velocity * dt;
        
        if let Some(animation) = self.animations.get_mut(&self.current_state) {
            animation.update(dt);
        }

        self.current_state = if self.velocity.abs() > 0.0 {
            PlayerState::RUN
        } else {
            PlayerState::IDLE
        }
    }

    pub fn render(&mut self, d: &mut RaylibDrawHandle) {
        if let Some(animation) = self.animations.get(&self.current_state) {
            let mut source = animation.current_rect();

            if self.facing < 0.0 {
                source.width *= -1.0;
            }

            d.draw_texture_rec(
                &animation.sprite_sheet,
                source,
                self.position,
                Color::WHITE,
            );
        }

        #[cfg(feature = "debug")]
        {
            let hb = self.hitbox.world_rect(self.position);
            d.draw_rectangle_lines_ex(hb, 1.0, Color::RED);
            d.draw_circle(self.position.x as i32, self.position.y as i32, 3.0, Color::GREEN);
        }
    }

    pub fn resolve_collision(&mut self, collider: &AABB) {

    }
}