use crate::state::State;

pub trait RenderingBackend {
    fn new(width: u32, height: u32, title: &'_ str) -> Self;
    fn render_state(&self, state: &State);
    fn main_loop(&mut self, state: State);
}

mod sdl2_backend;
pub type ChosenBackend = sdl2_backend::Sdl2Backend;
