use super::framebuffer::Framebuffer;
use super::rendering_backend::Event;

pub struct State {
    pub fb: Framebuffer,

    mouse_x: u32,
    mouse_y: u32,

    square_x: u32,
    square_y: u32,
}

impl State {
    pub fn new(fb_width: u32, fb_height: u32) -> Self {
        Self {
            fb: Framebuffer::new(fb_width, fb_height),

            mouse_x: 10,
            mouse_y: 10,

            square_x: 10,
            square_y: 10,
        }
    }

    pub fn update(&mut self, t: f64, _dt: f64) {
        self.square_x = (t.cos() * 50.0 + 100.0).round() as u32;
        self.square_y = (t.sin() * 50.0 + 100.0).round() as u32;
    }

    pub fn render(&mut self) {
        self.fb.clear();

        self.fb.draw_pixel(0, 2, 0xff00ff);

        self.fb.draw_square(self.square_x - 1, self.square_y - 1, 3, 0xff1010);

        for i in 0..5 {
            self.fb.draw_pixel(self.mouse_x.saturating_sub(i), self.mouse_y, 0xffffff);
            self.fb.draw_pixel(self.mouse_x.saturating_add(i), self.mouse_y, 0xffffff);
            self.fb.draw_pixel(self.mouse_x, self.mouse_y.saturating_sub(i), 0xffffff);
            self.fb.draw_pixel(self.mouse_x, self.mouse_y.saturating_add(i), 0xffffff);
        }
    }

    pub fn events(&mut self, event: Event) {
        match event {
            Event::MouseMotion(x, y) => {
                self.mouse_x = x as u32;
                self.mouse_y = y as u32;
            }
            _ => (),
        }
    }
}
