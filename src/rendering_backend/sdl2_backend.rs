use super::{Event, RenderingBackend};
use std::ffi::{c_int, c_void, CStr, CString};
use std::mem::{size_of, MaybeUninit};
use std::ptr;

use crate::state::State;

#[allow(clippy::approx_constant)]
#[allow(clippy::upper_case_acronyms)]
#[allow(clippy::unreadable_literal)]
#[allow(dead_code)]
#[allow(improper_ctypes)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/sdl_bindings.rs"));
}

use bindings::*;

pub struct Sdl2Backend {
    window: *mut SDL_Window,
    renderer: *mut SDL_Renderer,
    texture: *mut SDL_Texture,
}

impl Sdl2Backend {
    fn current_time(&self) -> f64 {
        let ms = unsafe { SDL_GetTicks64() };
        (ms as f64) / 1000.0
    }

    #[allow(non_upper_case_globals)] // rust-lang/rust #39371
    fn get_events(&mut self, state: &mut State) -> bool {
        let mut event = MaybeUninit::uninit();

        unsafe {
            while SDL_PollEvent(event.as_mut_ptr()) != 0 {
                let ret_event = event.assume_init();

                match ret_event.type_ {
                    SDL_EventType_SDL_QUIT => return false,
                    SDL_EventType_SDL_KEYDOWN => return false, // CBA
                    SDL_EventType_SDL_MOUSEMOTION => {
                        let x = ret_event.motion.x;
                        let y = ret_event.motion.y;
                        let event = Event::MouseMotion(x, y);
                        State::events(state, event)
                    }
                    SDL_EventType_SDL_MOUSEBUTTONDOWN => {
                        let x = ret_event.button.x;
                        let y = ret_event.button.y;
                        let event = Event::MousePress(x, y);
                        State::events(state, event)
                    }
                    SDL_EventType_SDL_MOUSEBUTTONUP => {
                        let x = ret_event.button.x;
                        let y = ret_event.button.y;
                        let event = Event::MouseRelease(x, y);
                        State::events(state, event)
                    }
                    SDL_EventType_SDL_MOUSEWHEEL => {
                        let y = ret_event.wheel.y;
                        let event = Event::MouseWheel(y);
                        State::events(state, event)
                    }
                    _ => continue,
                }
            }
        }

        true
    }
}

impl RenderingBackend for Sdl2Backend {
    fn new(width: u32, height: u32, title: &'_ str) -> Self {
        let cstring = CString::new(title).unwrap();
        let char_ptr = cstring.as_ptr();

        let any = SDL_WINDOWPOS_UNDEFINED_MASK.try_into().unwrap();
        let w = width.try_into().unwrap();
        let h = height.try_into().unwrap();
        let win_flags = SDL_WindowFlags_SDL_WINDOW_SHOWN;

        let rend_flags = SDL_RendererFlags_SDL_RENDERER_ACCELERATED;

        let tex_format = SDL_PixelFormatEnum_SDL_PIXELFORMAT_RGBA8888;
        let tex_flags = SDL_TextureAccess_SDL_TEXTUREACCESS_STREAMING.try_into().unwrap();

        unsafe {
            let window = SDL_CreateWindow(char_ptr, any, any, w, h, win_flags);
            if window.is_null() {
                panic_sdl("create window");
            }

            let renderer = SDL_CreateRenderer(window, -1, rend_flags);
            if renderer.is_null() {
                panic_sdl("create renderer");
            }

            let texture = SDL_CreateTexture(renderer, tex_format, tex_flags, w, h);
            if texture.is_null() {
                panic_sdl("create texture");
            }

            Self {
                window,
                renderer,
                texture,
            }
        }
    }

    fn render_state(&self, state: &State) {
        let pixels = &state.fb.pixels;
        let pix_ptr = pixels.as_ptr().cast::<c_void>();
        let width = state.fb.width;
        let px_bytes = size_of::<u32>() as u32;
        let pitch = (width * px_bytes) as c_int;

        unsafe {
            SDL_UpdateTexture(self.texture, ptr::null(), pix_ptr, pitch);
            SDL_RenderClear(self.renderer);
            SDL_RenderCopy(self.renderer, self.texture, ptr::null(), ptr::null());
            SDL_RenderPresent(self.renderer);
        }
    }

    fn main_loop(&mut self, mut state: State) {
        let updates_per_second = 60;
        let dt = 1.0 / f64::from(updates_per_second as i16);

        let mut running = true;
        let mut curr_time = 0.0;
        let mut real_time;

        while running {
            real_time = self.current_time();

            while curr_time < real_time {
                curr_time += dt;

                running = self.get_events(&mut state);
                State::update(&mut state, curr_time, dt);
            }

            State::render(&mut state);

            self.render_state(&state);
        }
    }
}

impl Drop for Sdl2Backend {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyTexture(self.texture);
            SDL_DestroyRenderer(self.renderer);
            SDL_DestroyWindow(self.window);
            SDL_Quit();
        }
    }
}

fn panic_sdl(desc: &'_ str) -> ! {
    panic!("failed to {}: {}", desc, get_sdl_error_str());
}

fn get_sdl_error_str<'a>() -> &'a str {
    unsafe { CStr::from_ptr(SDL_GetError()) }.to_str().unwrap()
}
