#![allow(clippy::unreadable_literal)]

mod framebuffer;
mod rendering_backend;
mod state;

use rendering_backend::{ChosenBackend, RenderingBackend};
use state::State;

fn main() {
    let filename = "font.psf";
    let file = std::fs::read(filename).expect("failed to read file");

    let w = 1024;
    let h = 768;
    let mut backend = ChosenBackend::new(w, h, "psfe");
    let state = State::new(w, h, &file);

    backend.main_loop(state);
}
