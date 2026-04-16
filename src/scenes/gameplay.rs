use raylib::{color::Color, math::{Rectangle, Vector2}, prelude::RaylibDraw, texture::{RaylibTexture2D, Texture2D}};

use crate::{game::{context::Context, scene::{Scene, SceneTransition}, state::GameState}, player::Player};

const IDLE_FRAMES: i32 = 26;
const IDLE_ROWS: i32 = 1;

pub struct GameplayScene {
    player: Player,
    frame_counter: i32,
    frame_speed: i32,
    current_frame: i32,
    frame_rec: Rectangle,
    sprite_sheet: Option<Texture2D>,
    position: Vector2
}

impl GameplayScene {
    pub fn new() -> Self {
        Self {
            player: Player::new(100., 100.),
            frame_counter: 0,
            frame_speed: 8,
            current_frame: 0,
            frame_rec: Rectangle::default(),
            sprite_sheet: None,
            position: Vector2 { x: 100., y: 100. }
        }
    }
}

impl Scene for GameplayScene {
    fn on_enter(&mut self, ctx: &mut Context) {
        self.sprite_sheet = Some(
            ctx.rl.load_texture(&ctx.thread, "assets/sprite-sheet/player/idle/sheet.png")
                .expect("failed to load player idle sheet")
        );

        let sheet = self.sprite_sheet.as_ref().unwrap();
        println!("sheet size: {}x{}", sheet.width, sheet.height);

        let frame_width  = sheet.width  / IDLE_FRAMES;
        let frame_height = sheet.height / IDLE_ROWS;

        self.frame_rec     = Rectangle { x: 0.0, y: 0.0, width: frame_width as f32, height: frame_height as f32 };
        self.current_frame = 0;
        self.frame_counter = 0;
        self.frame_speed   = 8;
        self.position      = Vector2 { x: 100.0, y: 100.0 };
    }

    fn on_exit(&mut self, ctx: &mut Context) {}
    
    fn update(&mut self, ctx: &mut Context) -> Option<SceneTransition> {
        self.frame_counter += 1;

        if self.frame_counter >= 60 / self.frame_speed {
            self.frame_counter  = 0;
            self.current_frame += 1;

            if self.current_frame >= IDLE_FRAMES {
                self.current_frame = 0;
            }

            let frame_width = self.sprite_sheet.as_ref().unwrap().width / IDLE_FRAMES;
            self.frame_rec.x = (self.current_frame * frame_width) as f32;
        }

        None
    }

    fn render_world(&mut self, d: &mut raylib::prelude::RaylibDrawHandle, state: &GameState) {
        d.clear_background(Color::WHITE);

        d.draw_texture_rec(self.sprite_sheet.as_ref().unwrap(), self.frame_rec, self.position, Color::WHITE);
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
            });
    }

    fn handle_input(&mut self, ctx: &mut Context) {
        
    }
}