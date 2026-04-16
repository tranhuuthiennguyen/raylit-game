use imgui::internal::RawWrapper;
use raylib::{RaylibHandle, RaylibThread, color::Color, ffi, prelude::RaylibDrawHandle, text, texture::{Image, Texture2D}};

pub struct RaylibRenderer {
    _font_texture: Option<Texture2D>
}

impl RaylibRenderer {
    #[must_use]
    pub fn init(
        rl: &mut RaylibHandle,
        thread: &RaylibThread, 
        imgui: &mut imgui::Context
    ) -> RaylibRenderer {
        let mut res = RaylibRenderer {
            _font_texture: None,
        };
        res.reload_fonts(rl, thread, imgui);
        res
    }
    #[must_use]
    pub fn load_fonts(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        imgui: &mut imgui::Context
    ) -> Texture2D {
        let texture = imgui.fonts().build_rgba32_texture();
        let image =
            Image::gen_image_color(texture.width as i32, texture.height as i32, Color::BLANK);
        
        let pixel_len = texture.width * texture.height * 4;

        unsafe {
            (image.data as *mut u8)
                .copy_from_nonoverlapping(texture.data.as_ptr(), pixel_len as usize)
        };

        let texture = rl.load_texture_from_image(thread, &image).unwrap();
        imgui.fonts().tex_id = (texture.id as usize).into();
        return texture;
    }

    pub fn reload_fonts(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        imgui: &mut imgui::Context
    ) {
        self._font_texture = Some(RaylibRenderer::load_fonts(rl, thread, imgui));
    }

    pub fn render(
        &mut self,
        rl: &RaylibDrawHandle,
        draw_data: &imgui::DrawData,
        framebuffer_scale: [f32; 2]
    ) {
        unsafe {
            ffi::rlDrawRenderBatchActive();
            ffi::rlDisableBackfaceCulling();
        }

        for list in draw_data.draw_lists() {
            for command in list.commands() {
                match command {
                    imgui::DrawCmd::Elements { count, cmd_params } => {
                        let [x, y, z, w] = cmd_params.clip_rect;
                        self.enable_scissor(
                            rl, 
                            x - draw_data.display_pos[0], 
                            y - draw_data.display_pos[1], 
                            z - (x - draw_data.display_pos[0]), 
                            w - (y - draw_data.display_pos[1]), 
                            framebuffer_scale
                        );

                        self.render_triangles(
                            count, 
                            cmd_params.idx_offset, 
                            list.idx_buffer(), 
                            list.vtx_buffer(), 
                            cmd_params.texture_id
                        );
                        unsafe {
                            ffi::rlDrawRenderBatchActive();
                        }
                    }
                    imgui::DrawCmd::RawCallback { callback, raw_cmd } => {
                        let clip_rect = unsafe { *raw_cmd }.ClipRect;

                        self.enable_scissor(
                            rl,
                            clip_rect.x - draw_data.display_pos[0],
                            clip_rect.y - draw_data.display_pos[1],
                            clip_rect.z - (clip_rect.x - draw_data.display_pos[0]),
                            clip_rect.w - (clip_rect.y - draw_data.display_pos[1]),
                            framebuffer_scale,
                        );

                        unsafe { callback(list.raw(), raw_cmd) }
                    }
                    imgui::DrawCmd::ResetRenderState => (),
                }
            }
        }

        unsafe {
            ffi::rlSetTexture(0);
            ffi::rlDisableScissorTest();
            ffi::rlEnableBackfaceCulling();
        }
    }

    fn enable_scissor(
        &self,
        rl: &RaylibDrawHandle,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        framebuffer_scale: [f32; 2],
    ) {
        unsafe {
            ffi::rlEnableScissorTest();
        }
        let [mut scale_x, mut scale_y] = framebuffer_scale;
        // does not work on mac, see jeffm imgui bindings
        #[cfg(not(target_os = "macos"))]
        {
            if !rl.get_window_state().window_highdpi() {
                scale_x = 1.0;
                scale_y = 1.0;
            }
        }

        let scissor_x = (x * scale_x) as i32;
        let scissor_y = ((rl.get_screen_height() as f32 - (y + height)) * scale_y) as i32;
        let scissor_width = (width * scale_x) as i32;
        let scissor_height = (height * scale_y) as i32;
        unsafe {
            ffi::rlScissor(scissor_x, scissor_y, scissor_width, scissor_height);
        }
    }

    fn render_triangles(
        &self,
        count: usize,
        start: usize,
        index_buffer: &[imgui::DrawIdx],
        vertex_buffer: &[imgui::DrawVert],
        texture: imgui::TextureId,
    ) {
        if count < 3 {
            return;
        }

        let texture_id = texture.id() as u32;

        unsafe {
            ffi::rlBegin(ffi::RL_TRIANGLES as i32);
            ffi::rlSetTexture(texture_id);
        }

        for i in (0..count).step_by(3) {
            unsafe {
                if ffi::rlCheckRenderBatchLimit(3) {
                    ffi::rlBegin(ffi::RL_TRIANGLES as i32);
                    ffi::rlSetTexture(texture_id);
                }
            }

            let idx1 = index_buffer[start + i];
            let idx2 = index_buffer[start + i + 1];
            let idx3 = index_buffer[start + i + 2];

            let vert1 = &vertex_buffer[idx1 as usize];
            let vert2 = &vertex_buffer[idx2 as usize];
            let vert3 = &vertex_buffer[idx3 as usize];

            RaylibRenderer::triangle_vertex(vert1);
            RaylibRenderer::triangle_vertex(vert2);
            RaylibRenderer::triangle_vertex(vert3);
        }

        unsafe {
            ffi::rlEnd();
        }
    }

    fn triangle_vertex(vertex: &imgui::DrawVert) {
        let [r, g, b, a] = vertex.col;
        unsafe {
            ffi::rlColor4ub(r, g, b, a);
            ffi::rlTexCoord2f(vertex.uv[0], vertex.uv[1]);
            ffi::rlVertex2f(vertex.pos[0], vertex.pos[1]);
        }
    }
}