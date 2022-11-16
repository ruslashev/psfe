use super::framebuffer::Framebuffer;
use super::rendering_backend::Event;

const GRID_OFFS_X: u32 = 3;
const GRID_OFFS_Y: u32 = 3;

const PSF1_MODE512: u8 = 0x01;

pub struct State {
    pub fb: Framebuffer,
    font: Font,
    hov_x: u32,
    hov_y: u32,
}

struct Font {
    _version: u8,
    width: u8,
    height: u8,
    glyphs: Vec<BitMatrix>,
}

struct BitMatrix {
    // This is a pretty wasteful structure, but it simplifies things
    width: u8,
    height: u8,
    data: Vec<bool>,
}

impl State {
    pub fn new(fb_width: u32, fb_height: u32, file: &[u8]) -> Self {
        Self {
            fb: Framebuffer::new(fb_width, fb_height),
            font: Font::from_file(file).expect("failed to parse font"),
            hov_x: 0,
            hov_y: 0,
        }
    }

    pub fn update(&mut self, _t: f64, _dt: f64) {}

    pub fn render(&mut self) {
        self.fb.clear();

        for (grid_y, row) in self.font.glyphs.chunks(16).enumerate() {
            for (grid_x, glyph) in row.iter().enumerate() {
                let fw = self.font.width as u32;
                let fh = self.font.height as u32;
                let gx = grid_x as u32;
                let gy = grid_y as u32;
                let offset_x = GRID_OFFS_X + gx * fw * 2;
                let offset_y = GRID_OFFS_Y + gy * fh * 2;

                let hovered = gx == self.hov_x && gy == self.hov_y;

                let border_color = if hovered { 0x990000 } else { 0x770000 };

                self.fb.draw_rect_hollow(offset_x, offset_y, fw + 2, fh + 2, border_color);

                for y in 0..fh {
                    for x in 0..fw {
                        let color = {
                            if glyph.get(x as usize, y as usize) {
                                0xffffff
                            } else if hovered {
                                0x606060
                            } else {
                                0x000000
                            }
                        };

                        self.fb.draw_pixel(offset_x + x + 1, offset_y + y + 1, color);
                    }
                }
            }
        }
    }

    pub fn events(&mut self, event: Event) {
        match event {
            Event::MouseMotion(x, y) => {
                self.detect_mouse_hover(x, y);
            }
            _ => (),
        }
    }

    fn detect_mouse_hover(&mut self, mut x: i32, mut y: i32) {
        x -= GRID_OFFS_X as i32;
        y -= GRID_OFFS_Y as i32;

        x /= 2;
        y /= 2;

        x /= self.font.width as i32;
        y /= self.font.height as i32;

        if x <= 16 && y <= 16 {
            self.hov_x = x as u32;
            self.hov_y = y as u32;
        }
    }
}

impl Font {
    fn from_file(file: &[u8]) -> Option<Self> {
        if file[0..2] == [0x36, 0x04] {
            return Self::parse_psf1(file);
        }

        None
    }

    fn parse_psf1(file: &[u8]) -> Option<Self> {
        let height = file[3];

        Some(Self {
            _version: 1,
            width: 8,
            height,
            glyphs: Self::parse_psf1_glyphs(height, file),
        })
    }

    fn parse_psf1_glyphs(height: u8, file: &[u8]) -> Vec<BitMatrix> {
        let mode = file[2];
        let num_glyphs = if mode & PSF1_MODE512 != 0 { 512 } else { 256 };
        let mut glyphs = vec![];
        let glyph_data = &file[4..];

        for i in 0..num_glyphs {
            let mut glyph = BitMatrix::new(8, height);
            let offset = i * height as usize;

            for h in 0..height {
                let row = glyph_data[offset + h as usize];

                for bit in 0..8 {
                    let mask = 1 << bit;

                    if row & mask != 0 {
                        glyph.set(8 - bit - 1, h.into());
                    }
                }
            }

            glyphs.push(glyph);
        }

        glyphs
    }
}

impl BitMatrix {
    fn new(width: u8, height: u8) -> Self {
        let mut data = Vec::new();
        let w: usize = width.into();
        let h: usize = height.into();

        data.resize(w * h, false);

        Self {
            width,
            height,
            data,
        }
    }

    fn set(&mut self, x: usize, y: usize) {
        assert!(x < self.width.into());
        assert!(y < self.height.into());

        let w: usize = self.width.into();

        self.data[y * w + x] = true;
    }

    fn get(&self, x: usize, y: usize) -> bool {
        assert!(x < self.width.into());
        assert!(y < self.height.into());

        let w: usize = self.width.into();

        self.data[y * w + x]
    }
}
