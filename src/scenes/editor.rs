use raylib::{color::Color};
use raylib::prelude::*;

use crate::game::context::Context;
use crate::game::scene::{Scene, SceneTransition};
use crate::game::state::GameState;

pub struct EditorScene;

impl EditorScene {
    pub fn new() -> Self {
        Self
    }
}

impl Scene for EditorScene {
    fn on_enter(&mut self, ctx: &mut Context) {
        println!("[EDITOR]: on_enter()");
    }

    fn on_exit(&mut self, ctx: &mut Context) {
        println!("[EDITOR]: on_exit()");
    }

    fn update(&mut self, ctx: &mut Context) -> Option<SceneTransition> {
        None
    }

    fn render_world(&mut self, d: &mut RaylibDrawHandle, state: &GameState) {
        d.clear_background(Color::DARKGRAY);
    }

    fn render_ui(&mut self, ui: &imgui::Ui, state: &GameState) {
        ui.window("Editor")
            .position([0.0, 0.0], imgui::Condition::Always)
            .size([1280.0, 720.0], imgui::Condition::Always)
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .bg_alpha(0.0)
            .build(|| {
                ui.text("Editor scene");
            });
    }

    fn handle_input(&mut self, ctx: &mut Context) {
        
    }
}