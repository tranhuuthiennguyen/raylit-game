use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Frame {
    frame: Rect,
}

#[derive(Deserialize, Debug)]
pub struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[derive(Deserialize, Debug)]
pub struct Atlas {
    frames: Vec<Frame>,
}

impl Atlas {
    pub fn new(filename: &str) -> Self {
        let json_str = std::fs::read_to_string(filename).expect("frames metadata");
        Self {
            frames: serde_json::from_str(&json_str).unwrap()
        }
    }

    pub fn count(&self) -> usize {
        self.frames.len()
    }

    pub fn get_frame(&self, index: usize) -> &Frame {
        &self.frames[index]
    }
}