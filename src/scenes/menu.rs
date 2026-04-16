use raylib::prelude::*;
use raylib::color::Color;

use crate::{game::{context::Context, scene::{Scene, SceneTransition}, state::GameState}, scenes::{editor::EditorScene, gameplay::GameplayScene}};

pub struct MenuScene {
    bg_color: Color,
    change_bg: bool,
    go_to_editor: bool,
    go_to_gameplay: bool
}

impl MenuScene {
    pub fn new() -> Self {
        Self {
            bg_color: Color::BLACK,
            change_bg: false,
            go_to_editor: false,
            go_to_gameplay: false
        }
    }
}

impl Scene for MenuScene {

    fn on_enter(&mut self, ctx: &mut Context) {
        println!("[MENU]: on_enter()");
        self.go_to_editor = false;
    }

    fn on_exit(&mut self, ctx: &mut Context) {
        println!("[MENU]: on_exit()");
    }

    fn update(&mut self, ctx: &mut Context) -> Option<SceneTransition> {
        if self.go_to_editor {
            return Some(SceneTransition::Replace(Box::new(EditorScene::new())));
        }
        if self.go_to_gameplay {
            return Some(SceneTransition::Replace(Box::new(GameplayScene::new())))
        }
        None
    }
    
    fn render_world(&mut self, d: &mut RaylibDrawHandle, state: &GameState) {
        if self.change_bg {
            self.bg_color = Color { 
                r: d.get_random_value::<i32>(0..255) as u8, 
                g: d.get_random_value::<i32>(0..255) as u8, 
                b: d.get_random_value::<i32>(0..255) as u8, 
                a: 255 
            };
            self.change_bg = false;
        }
        d.clear_background(&self.bg_color);
    }

    fn render_ui(&mut self, ui: &imgui::Ui, state: &GameState) {
        let display_size = ui.io().display_size;
        ui.window("Overlay")
            .position([0.0, 0.0], imgui::Condition::Always)
            .size([1280.0, 720.0], imgui::Condition::Always)
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .scroll_bar(false)
            .bg_alpha(0.0)
            .build(|| {
                ui.text(format!("{:?}", display_size));
                if ui.button("Change Background Color") {
                    self.change_bg = true;
                }
                if ui.button("New Game") {
                    self.go_to_gameplay = true;
                }
                if ui.button("Open Editor") {
                    self.go_to_editor = true;
                }
            });
    }

    fn handle_input(&mut self, ctx: &mut Context) {
        
    }
}