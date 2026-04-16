pub struct GameState {
    pub settings: Settings,
    pub save_slot: Option<SaveData>
}

impl GameState {
    pub fn new() -> Self {
        Self {
            settings: Settings::new(),
            save_slot: None
        }
    }
}

pub struct Settings {
    pub master_volume: f32,
    pub fullscreen: bool,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            master_volume: 0.7,
            fullscreen: false
        }
    }
}

pub struct SaveData {
    pub player_name: String,
    pub level: u32,
    pub health: f32,
    pub position: (f32, f32)
}