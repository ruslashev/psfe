use crate::state::State;

pub trait RenderingBackend {
    fn new(width: u32, height: u32, title: &'_ str) -> Self;
    fn render_state(&self, state: &State);
    fn main_loop(&mut self, state: State);
}

pub enum Event {
    KeyPress(KeyButton),
    KeyRelease(KeyButton),
    MouseMotion(i32, i32),
    MousePress(MouseButton, i32, i32),
    MouseRelease(i32, i32),
    MouseWheel(i32),
}

#[derive(PartialEq, Eq)]
pub enum KeyButton {
    Character(char),
    Escape,
    Minus,
}

#[derive(PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    X1,
    X2,
}

mod sdl2_backend;
pub type ChosenBackend = sdl2_backend::Sdl2Backend;
