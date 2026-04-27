use raylib::math::Rectangle;
use raylib::math::Vector2;

pub struct AABB {
    pub offset: Vector2,
    pub width: f32,
    pub height: f32
}

impl AABB {
    pub fn new(offset_x: f32, offset_y: f32, width: f32, height: f32) -> Self {
        Self {
            offset: Vector2 { x: offset_x, y: offset_y },
            width,
            height
        }
    }

    pub fn world_rect(&self, position: Vector2) -> Rectangle {
        Rectangle {
            x: position.x + self.offset.x,
            y: position.y + self.offset.y,
            width: self.width,
            height: self.height
        }
    }

    pub fn does_intersect(&self, self_pos: Vector2, other: &AABB, other_pos: Vector2) -> bool {
        let a = self.world_rect(self_pos);
        let b = other.world_rect(other_pos);

        a.x <= b.x + b.width &&
        a.x + a.width >= b.x &&
        a.y + a.height >= b.y &&
        a.y <= b.y + b.height
    }
}