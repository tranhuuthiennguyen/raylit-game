use raylib::{ffi::Rectangle, math::Vector2};

use crate::collision::AABB;

pub struct Terrain {
    pub position: Vector2,
    pub size: Vector2,
    pub texture_rect: Rectangle,
    pub collider: Option<AABB>,
}

impl Terrain {
    pub fn new(
        position: Vector2,
        size: Vector2,
        texture_rect: Rectangle,
        collidable: bool
    ) -> Self {
        let collider = if collidable {
            Some(AABB::new(0.0, 0.0, size.x, size.y))
        } else {
            None
        };

        Self {
            position,
            size,
            texture_rect,
            collider
        }
    }
}