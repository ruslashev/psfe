use super::framebuffer::Framebuffer;
use super::rendering_backend::Event;

const GRID_OFFS_X: u32 = 3;
const GRID_OFFS_Y: u32 = 3;

const PSF1_MODE512: u8 = 0x01;

pub struct State {
    pub fb: Framebuffer,
    font: Font,
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
        }
    }

    pub fn update(&mut self, _t: f64, _dt: f64) {}

    pub fn render(&mut self) {
        self.fb.clear();

        let mut grid_y: u32 = 0;
        let mut grid_x: u32 = 0;

        for glyph in &self.font.glyphs {
            let fw = self.font.width.into();
            let fh = self.font.height.into();
            let offset_x = GRID_OFFS_X + grid_x * fw * 2;
            let offset_y = GRID_OFFS_Y + grid_y * fh * 2;

            self.fb.draw_rect_hollow(offset_x, offset_y, fw + 2, fh + 2, 0xaa0000);

            for y in 0..fh {
                for x in 0..fw {
                    let mut color = 0;

                    if glyph.get(x as usize, y as usize) {
                        color = 0xffffff;
                    }

                    self.fb.draw_pixel(offset_x + x + 1, offset_y + y + 1, color);
                }
            }

            grid_x += 1;
            if grid_x >= 16 {
                grid_x = 0;
                grid_y += 1;
            }
        }
    }

    pub fn events(&mut self, _event: Event) {}
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
