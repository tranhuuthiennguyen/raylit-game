use raylib::color::Color;
use raylib::prelude::RaylibDraw;

use crate::world::World;
use crate::game::{context::Context, scene::{Scene, SceneTransition}, state::GameState};

pub struct GameplayScene {
    world: Option<World>

}

impl GameplayScene {
    pub fn new() -> Self {
        Self {
            world: None
        }
    }
}

impl Scene for GameplayScene {
    fn on_enter(&mut self, ctx: &mut Context) {
        self.world = Some(World::new(ctx));
    }

    fn on_exit(&mut self, _ctx: &mut Context) {}
    
    fn update(&mut self, ctx: &mut Context) -> Option<SceneTransition> {
        let dt = ctx.rl.get_frame_time();
        
        if let Some(world) = self.world.as_mut() {
            world.update(dt);
        }

        None
    }

    fn render_world(&mut self, d: &mut raylib::prelude::RaylibDrawHandle, _state: &GameState) {
        d.clear_background(Color::BLACK);

        if let Some(world) = self.world.as_mut() {
            world.render(d);
        }
    }

    fn render_ui(&mut self, ui: &imgui::Ui, _state: &GameState) {
        ui.window("Overlay")
            .position([0.0, 0.0], imgui::Condition::Always)
            .size([1280.0, 960.0], imgui::Condition::Always)
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .scroll_bar(false)
            .bg_alpha(0.0)
            .build(|| {
                #[cfg(feature = "debug")]
                {
                    if let Some(world) = self.world.as_mut() {
                        ui.text(format!("x: {:.1}; y: {:.1}", world.player.position.x, world.player.position.y));
                        ui.text(format!("vel: {:.1}", world.player.velocity));
                        ui.text(format!("dir: {:.1}", world.player.input_dir));
                    }
                }
            });
    }

    fn handle_input(&mut self, ctx: &mut Context) {
        if let Some(world) = self.world.as_mut() {
            world.player.handle_input(ctx);
        }
    }
}