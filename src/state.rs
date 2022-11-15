use super::framebuffer::Framebuffer;

pub struct State {
    pub fb: Framebuffer,
}

impl State {
    pub fn new(fb_width: u32, fb_height: u32) -> Self {
        Self {
            fb: Framebuffer::new(fb_width, fb_height),
        }
    }

    pub fn update(&mut self, _dt: f64) {}

    pub fn render(&mut self) {
        self.fb.draw_pixel(0, 2, 0xff00ff);
    }
}
