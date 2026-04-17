use raylib::math::Rectangle;
use raylib::texture::Texture2D;

use crate::game::context::Context;

pub struct Animation {
    pub sprite_sheet: Texture2D,
    pub frames: Vec<Rectangle>,
    pub current: usize,
    elapsed: f32,
    pub frame_duration: f32
}

impl Animation {
    pub fn load(
        ctx: &mut Context,
        path: &str,
        frame_width: i32,
        frame_height: i32,
        count: usize,
        fps: f32
    ) -> Self {
        let sprite_sheet = ctx.rl
            .load_texture(&ctx.thread, path)
            .unwrap_or_else(|_|panic!("failed to load animation: {path}"));
        let frames = (0..count)
            .map(|i| Rectangle {
                x: (i as i32 * frame_width) as f32,
                y: 0.0,
                width: frame_width as f32,
                height: frame_height as f32
            })
            .collect();

        Self {
            sprite_sheet,
            frames,
            current: 0,
            elapsed: 0.0,
            frame_duration: 1.0 / fps
        }
    }

    pub fn update(&mut self, dt: &f32) {
        self.elapsed += dt;
        if self.elapsed >= self.frame_duration {
            self.elapsed -= self.frame_duration;
            self.current = (self.current + 1) % self.frames.len();
        }
    }

    pub fn current_rect(&self) -> Rectangle {
        self.frames[self.current]
    }
}