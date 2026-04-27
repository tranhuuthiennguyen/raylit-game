use raylib::{math::Vector2, texture::Texture2D};

#[derive(Debug)]
pub struct Tile {
    id: String,
    position: Vector2,
    width: f32,
    height: f32
}

impl Tile {
    pub fn from(
        id: &str,
        position: Vector2,
        width: f32,
        height: f32
    ) -> Self {
        Self {
            id: id.to_string(),
            position: position,
            width: width,
            height: height
        }
    }
}