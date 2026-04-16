use raylib::RaylibHandle;

use crate::ui::{platform::RaylibPlatform, renderer::RaylibRenderer};

pub struct RaylibImguiSupport {
    pub context: imgui::Context,
    renderer: RaylibRenderer,
    platform: RaylibPlatform
}

impl RaylibImguiSupport {
    #[must_use]
    pub fn setup(
        rl: &mut raylib::RaylibHandle,
        thread: &raylib::RaylibThread
    ) -> RaylibImguiSupport {
        let mut context = imgui::Context::create();
        context.set_ini_filename(None);
        context.set_log_filename(None);
        context
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);
        let style = context.style_mut();
        let renderer = RaylibRenderer::init(rl, thread, &mut context);
        let platform = RaylibPlatform::init(rl, &mut context);

        RaylibImguiSupport {
            context,
            renderer,
            platform
        }
    }

    pub fn start_frame(&mut self, rl: &mut RaylibHandle) -> &mut imgui::Ui {
        self.platform.new_frame(rl, &mut self.context);
        self.platform.process_events(rl, &mut self.context);

        self.context.new_frame()
    }
    
    pub fn end_frame(&mut self, rl: &mut raylib::drawing::RaylibDrawHandle) {
        let [fb_x, fb_y] = self.context.io_mut().display_framebuffer_scale;
        let draw_data = self.context.render();

        self.renderer.render(rl, draw_data, [fb_x, fb_y]);
    }
}