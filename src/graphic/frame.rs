use serde::Deserialize;



#[derive(Deserialize, Debug)]
pub struct Frame {
    pub filename: String,
    pub frame: Rect,
    pub rotated: bool,
    pub trimmed: bool,
    #[serde(rename = "spriteSourceSize")]
    pub sprite_source_size: Rect,
    #[serde(rename = "sourceSize")]
    pub source_size: Size
}

#[derive(Deserialize, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[derive(Deserialize, Debug)]
pub struct Size {
    w: i32,
    h: i32
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    pub version: String,
    pub image: String,
    pub format: String,
    size: Size
}

#[derive(Deserialize, Debug)]
pub struct Atlas {
    frames: Vec<Frame>,
    meta: Meta
}

impl Atlas {
    pub fn from(filename: &str) -> Self {
        let json_str = std::fs::read_to_string(filename).expect("frames metadata");
        serde_json::from_str(&json_str).unwrap()
    }

    pub fn count(&self) -> usize {
        self.frames.len()
    }

    pub fn get_frame_size(&self) -> (i32, i32) {
        (self.meta.size.w/self.count() as i32, self.meta.size.h)
    }
}