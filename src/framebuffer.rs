use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub background_color: Color,
    pub current_color: Color,
    pub color_buffer: Image,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);

        Framebuffer {
            width,
            height,
            background_color,
            current_color: Color::WHITE,
            color_buffer,
        }
    }
    //limpiar el framebuffer
    pub fn clear(&mut self) {
        self.color_buffer =
            Image::gen_image_color(self.width as i32, self.height as i32, self.background_color);
    }
    //poner píxel en la pantalla
    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.color_buffer
                .draw_pixel(x as i32, y as i32, self.current_color);
        }
    }
    //setear el color de fondo
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }
    //settear el color
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }
    //guardar el framebuffer en un archivo usando export
    pub fn render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }
    pub fn swap_buffer(& self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut render = window.begin_drawing(raylib_thread);
            render.draw_texture(&texture, 0, 0, Color::WHITE)
        }
    }
}
