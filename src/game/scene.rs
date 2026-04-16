use std::vec;

use raylib::prelude::{RaylibDrawHandle};

use crate::game::{context::Context, state::GameState};

pub enum SceneTransition {
    Push(Box<dyn Scene>),
    Pop,
    Replace(Box<dyn Scene>)
}

pub trait Scene {
    fn on_enter(&mut self, ctx: &mut Context) {}
    fn on_exit(&mut self, ctx: &mut Context) {}
    fn update(&mut self, ctx: &mut Context) -> Option<SceneTransition>;
    fn render_world(&mut self, d: &mut RaylibDrawHandle, state: &GameState);
    fn render_ui(&mut self, ui: &imgui::Ui, state: &GameState);
    fn handle_input(&mut self, ctx: &mut Context);
}

pub struct SceneManager {
    stack: Vec<Box<dyn Scene>>
}

impl SceneManager {
    pub fn init(initial: Box<dyn Scene>) -> SceneManager {
        let initial= initial;
        SceneManager { 
            stack: vec![initial]
        }
    }

    pub fn handle_input(&mut self, ctx: &mut Context) {
        if let Some(scene) = self.stack.last_mut() {
            scene.handle_input(ctx);
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let transition = {
            let Some(scene) = self.stack.last_mut() else { return };
            scene.update(ctx)
        };

        match transition {
            Some(SceneTransition::Replace(mut next)) => {
                if let Some(mut old) = self.stack.pop() {
                    old.on_exit(ctx);
                }
                next.on_enter(ctx);
                self.stack.push(next);
            }
            Some(SceneTransition::Push(mut next)) => {
                next.on_enter(ctx);
                self.stack.push(next);
            }
            Some(SceneTransition::Pop) => {
                if let Some(mut old) = self.stack.pop() {
                    old.on_exit(ctx);
                }

                if let Some(resumed) = self.stack.last_mut() {
                    resumed.on_enter(ctx);
                }
            }
            None => {}
        }
    }

    pub fn render(&mut self, d: &mut RaylibDrawHandle, ui: &imgui::Ui, state: &GameState) {
        if let Some(scene) = self.stack.last_mut() {
            scene.render_world(d, state);
            scene.render_ui(ui, state);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}