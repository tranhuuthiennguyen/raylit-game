use raylib::{color::Color, math::Vector2};
use raylib::prelude::RaylibDraw;

use crate::{game::{context::Context, scene::{Scene, SceneTransition}, state::GameState}, graphic::animation::Animation, player::{Player, PlayerConfig}};

pub struct GameplayScene {
    player: Option<Player>,
}

impl GameplayScene {
    pub fn new() -> Self {
        Self {
            player: None,
        }
    }
}

impl Scene for GameplayScene {
    fn on_enter(&mut self, ctx: &mut Context) {
        self.player = Some(Player::load(
            PlayerConfig {
                start_position: Vector2::new(200., 200.),
                speed: 0.0,
            }, ctx))
    }

    fn on_exit(&mut self, ctx: &mut Context) {}
    
    fn update(&mut self, ctx: &mut Context) -> Option<SceneTransition> {
        let dt = ctx.rl.get_frame_time();
        self.player.as_mut().unwrap().update(&dt);
        None
    }

    fn render_world(&mut self, d: &mut raylib::prelude::RaylibDrawHandle, state: &GameState) {
        d.clear_background(Color::BLACK);

        self.player.as_mut().unwrap().render(d);
    }

    fn render_ui(&mut self, ui: &imgui::Ui, state: &GameState) {
        ui.window("Overlay")
            .position([0.0, 0.0], imgui::Condition::Always)
            .size([1280.0, 720.0], imgui::Condition::Always)
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .scroll_bar(false)
            .bg_alpha(0.0)
            .build(|| {
                #[cfg(feature = "debug")]
                {
                    let player = self.player.as_ref().unwrap();
                    ui.text(format!("x: {:.1}; y: {:.1}", player.position.x, player.position.y));
                    ui.text(format!("vel: {:.1}", player.velocity));
                    ui.text(format!("dir: {:.1}", player.input_dir));
                }
            });
    }

    fn handle_input(&mut self, ctx: &mut Context) {
        self.player.as_mut().unwrap().handle_input(ctx);
    }
}