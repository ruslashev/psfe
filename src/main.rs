mod framebuffer;
mod rendering_backend;
mod state;

use rendering_backend::{ChosenBackend, RenderingBackend};
use state::State;

fn main() {
    let w = 800;
    let h = 600;
    let mut backend = ChosenBackend::new(w, h, "psfe");
    let state = State::new(w, h);

    backend.main_loop(state);
}
