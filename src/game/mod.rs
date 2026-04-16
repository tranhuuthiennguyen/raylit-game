pub mod scene;
pub mod state;
pub mod context;

use raylib::prelude::*;

use crate::{game::{context::Context, scene::SceneManager, state::GameState}, scenes::menu::MenuScene, ui::imgui::RaylibImguiSupport};


pub struct Game {
    rl: RaylibHandle,
    thread: RaylibThread,
    scenes: SceneManager,
    pub state: GameState
}

impl Game {
    #[must_use]
    pub fn init(width: i32, height: i32, title: &str) -> Game {
        let (rl, thread) = raylib::init()
            .width(width)
            .height(height)
            .title(&title)
            .resizable()
            .build();
        let scenes = SceneManager::init(Box::new(MenuScene::new()));
        let state = GameState::new();
        Game {
            rl,
            thread,
            scenes,
            state
        }
    }

    pub fn run(&mut self) {
        let mut imgui_rl = RaylibImguiSupport::setup(&mut self.rl, &self.thread);
        self.rl.set_target_fps(60);
        while !self.rl.window_should_close() {
            {
                let mut ctx = Context {
                    rl: &mut self.rl,
                    state: &mut self.state,
                    thread: &self.thread
                };
                self.scenes.handle_input(&mut ctx);
                self.scenes.update(&mut ctx);
            }

            let ui = imgui_rl.start_frame(&mut self.rl);
            let mut d = self.rl.begin_drawing(&self.thread);
            d.clear_background(Color::WHITE);
            self.scenes.render(&mut d, ui, &self.state);
            imgui_rl.end_frame(&mut d);
        }
    }
}