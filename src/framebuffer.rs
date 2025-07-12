use raylib::prelude::*;

pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    front_buffer: Image,
    back_buffer: Image,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, background_color: Color) -> Self {
        let front_buffer = Image::gen_image_color(width, height, background_color);
        let back_buffer = Image::gen_image_color(width, height, background_color);
        Framebuffer {
            width,
            height,
            front_buffer,
            back_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        // Limpia el buffer de colores con el color de fondo
        self.back_buffer = Image::gen_image_color(self.width, self.height, self.background_color);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        // Pone un pixel en la pantalla, asegurándose de no salir del buffer
        if x < self.width && y < self.height {
            Image::draw_pixel(
                &mut self.back_buffer,
                x as i32,
                y as i32,
                self.current_color,
            );
        }
    }

    pub fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.front_buffer, &mut self.back_buffer);
    }

    pub fn get_front_buffer_data(&self) -> ImageColors {
        self.front_buffer.get_image_data()
    }


    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_to_file(&self, file_path: &str) {
        // Guarda el framebuffer a un archivo usando export
        Image::export_image(&self.front_buffer, file_path);
    }
}
