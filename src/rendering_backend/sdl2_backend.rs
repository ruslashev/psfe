#![allow(non_upper_case_globals)] // rust-lang/rust #39371

use super::{Event, KeyButton, MouseButton, RenderingBackend};
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
    running: bool,
    window: *mut SDL_Window,
    renderer: *mut SDL_Renderer,
    texture: *mut SDL_Texture,
}

impl Sdl2Backend {
    fn current_time(&self) -> f64 {
        let ms = unsafe { SDL_GetTicks64() };
        (ms as f64) / 1000.0
    }

    fn get_events(&mut self, state: &mut State) {
        let mut event = MaybeUninit::uninit();

        unsafe {
            while SDL_PollEvent(event.as_mut_ptr()) != 0 {
                let ret_event = event.assume_init();

                match ret_event.type_ {
                    SDL_EventType_SDL_QUIT => self.running = false,
                    SDL_EventType_SDL_KEYDOWN => {
                        if let Some(key) = Self::key_button_to_enum(ret_event.key.keysym.sym) {
                            let event = Event::KeyPress(key);
                            State::events(state, event)
                        }
                    }
                    SDL_EventType_SDL_KEYUP => {
                        if let Some(key) = Self::key_button_to_enum(ret_event.key.keysym.sym) {
                            let event = Event::KeyRelease(key);
                            State::events(state, event)
                        }
                    }
                    SDL_EventType_SDL_MOUSEMOTION => {
                        let x = ret_event.motion.x;
                        let y = ret_event.motion.y;
                        let event = Event::MouseMotion(x, y);
                        State::events(state, event)
                    }
                    SDL_EventType_SDL_MOUSEBUTTONDOWN => {
                        let x = ret_event.button.x;
                        let y = ret_event.button.y;
                        let maybe_button = Self::mouse_button_to_enum(ret_event.button.button);
                        if let Some(button) = maybe_button {
                            let event = Event::MousePress(button, x, y);
                            State::events(state, event)
                        }
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
    }

    fn key_button_to_enum(keycode: i32) -> Option<KeyButton> {
        match keycode as u32 {
            SDL_KeyCode_SDLK_ESCAPE => Some(KeyButton::Escape),
            SDL_KeyCode_SDLK_a => Some(KeyButton::Character('a')),
            SDL_KeyCode_SDLK_b => Some(KeyButton::Character('b')),
            SDL_KeyCode_SDLK_c => Some(KeyButton::Character('c')),
            SDL_KeyCode_SDLK_d => Some(KeyButton::Character('d')),
            SDL_KeyCode_SDLK_e => Some(KeyButton::Character('e')),
            SDL_KeyCode_SDLK_f => Some(KeyButton::Character('f')),
            SDL_KeyCode_SDLK_g => Some(KeyButton::Character('g')),
            SDL_KeyCode_SDLK_h => Some(KeyButton::Character('h')),
            SDL_KeyCode_SDLK_i => Some(KeyButton::Character('i')),
            SDL_KeyCode_SDLK_j => Some(KeyButton::Character('j')),
            SDL_KeyCode_SDLK_k => Some(KeyButton::Character('k')),
            SDL_KeyCode_SDLK_l => Some(KeyButton::Character('l')),
            SDL_KeyCode_SDLK_m => Some(KeyButton::Character('m')),
            SDL_KeyCode_SDLK_n => Some(KeyButton::Character('n')),
            SDL_KeyCode_SDLK_o => Some(KeyButton::Character('o')),
            SDL_KeyCode_SDLK_p => Some(KeyButton::Character('p')),
            SDL_KeyCode_SDLK_q => Some(KeyButton::Character('q')),
            SDL_KeyCode_SDLK_r => Some(KeyButton::Character('r')),
            SDL_KeyCode_SDLK_s => Some(KeyButton::Character('s')),
            SDL_KeyCode_SDLK_t => Some(KeyButton::Character('t')),
            SDL_KeyCode_SDLK_u => Some(KeyButton::Character('u')),
            SDL_KeyCode_SDLK_v => Some(KeyButton::Character('v')),
            SDL_KeyCode_SDLK_w => Some(KeyButton::Character('w')),
            SDL_KeyCode_SDLK_x => Some(KeyButton::Character('x')),
            SDL_KeyCode_SDLK_y => Some(KeyButton::Character('y')),
            SDL_KeyCode_SDLK_z => Some(KeyButton::Character('z')),
            _ => None,
        }
    }

    fn mouse_button_to_enum(button_int: u8) -> Option<MouseButton> {
        match button_int.into() {
            SDL_BUTTON_LEFT => Some(MouseButton::Left),
            SDL_BUTTON_MIDDLE => Some(MouseButton::Middle),
            SDL_BUTTON_RIGHT => Some(MouseButton::Right),
            SDL_BUTTON_X1 => Some(MouseButton::X1),
            SDL_BUTTON_X2 => Some(MouseButton::X2),
            _ => None,
        }
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
                running: true,
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

        let mut curr_time = 0.0;
        let mut real_time;

        while self.running {
            real_time = self.current_time();

            while curr_time < real_time {
                curr_time += dt;

                self.get_events(&mut state);
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
