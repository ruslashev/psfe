use super::rendering_backend::{RenderingBackend, ChosenBackend};

pub type UpdateCallback = fn(f64);
pub type RenderCallback = fn(&mut Framebuffer);

pub struct Framebuffer {
    width: u32,
    height: u32,
    pixels: Vec<u32>,
    backend: ChosenBackend,
}

impl Framebuffer {
    pub fn new<'a>(
        width: u32,
        height: u32,
        title: &'a str,
    ) -> Self {
        let length = (width * height).try_into().unwrap();
        let mut pixels = Vec::new();

        pixels.resize(length, 0);

        Self {
            width,
            height,
            pixels,
            backend: ChosenBackend::new(width, height, title),
        }
    }

    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32) {
        let idx = y * self.width + x;
        self.pixels[idx as usize] = (color << 8) | 0xff;
    }

    pub fn draw_square(&mut self, x: u32, y: u32, size: u32, color: u32) {
        for dy in 0..size {
            for dx in 0..size {
                self.draw_pixel(x + dx, y + dy, color);
            }
        }
    }

    pub fn main_loop(&mut self, update: UpdateCallback, render: RenderCallback) {
        let updates_per_second = 60;
        let dt = 1.0 / f64::from(updates_per_second as i16);

        let mut running = true;
        let mut curr_time = 0.0;
        let mut real_time;

        while running {
            real_time = self.backend.current_time();

            while curr_time < real_time {
                curr_time += dt;

                running = self.backend.get_events();
                (update)(dt);
            }

            (render)(self);
            self.backend.draw_framebuffer(&self.pixels, self.width);
        }
    }
}
