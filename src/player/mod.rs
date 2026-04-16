use crate::player::position::Position;

pub mod position;

pub struct Player {
    pos: Position
}

impl Player {
    pub fn new(pos_x: f32, pos_y: f32) -> Self {
        Self {
            pos: Position::new(pos_x, pos_y)
        }
    }

    pub fn move_left(&mut self, dis: f32) {
        self.pos.x -= dis;
    }

    pub fn move_right(&mut self, dis: f32) {
        self.pos.x += dis;
    }
}