mod framebuffer;
mod rendering_backend;

use framebuffer::Framebuffer;

fn update(_dt: f64) {
}

fn render(fb: &mut Framebuffer) {
    fb.draw_pixel(0, 2, 0xff00ff);
}

fn main() {
    let mut fb = Framebuffer::new(800, 600, "psfe");

    fb.main_loop(update, render);
}
