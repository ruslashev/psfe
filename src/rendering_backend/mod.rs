pub trait RenderingBackend {
    fn new<'a>(width: u32, height: u32, title: &'a str) -> Self;
    fn draw_framebuffer(&self, pixels: &[u32], width: u32);
    fn current_time(&self) -> f64;
    fn get_events(&mut self) -> bool;
}

mod sdl2_backend;
pub type ChosenBackend = sdl2_backend::Sdl2Backend;
