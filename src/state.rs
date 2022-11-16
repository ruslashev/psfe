use super::framebuffer::Framebuffer;
use super::rendering_backend::Event;

const GRID_OFFS_X: u32 = 3;
const GRID_OFFS_Y: u32 = 3;

const EDITOR_CELL_SIZE: u32 = 16;

const PSF1_MODE512: u8 = 0x01;

pub struct State {
    pub fb: Framebuffer,
    font: Font,

    hov_x: u32,
    hov_y: u32,
    sel_x: u32,
    sel_y: u32,

    editor_offs_x: u32,
    editor_offs_y: u32,
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
        let font = Font::from_file(file).expect("failed to parse font");
        let fw = font.width as u32;
        let fh = font.height as u32;

        Self {
            fb: Framebuffer::new(fb_width, fb_height),
            font,
            hov_x: 0,
            hov_y: 0,
            sel_x: 0,
            sel_y: 0,
            editor_offs_x: fb_width / 2 - fw * EDITOR_CELL_SIZE / 2,
            editor_offs_y: fb_height / 2 - fh * EDITOR_CELL_SIZE / 2,
        }
    }

    pub fn update(&mut self, _t: f64, _dt: f64) {}

    pub fn render(&mut self) {
        self.fb.clear();

        self.render_glyphs_grid();
        self.render_glyph_editor();
    }

    fn render_glyphs_grid(&mut self) {
        for (grid_y, row) in self.font.glyphs.chunks(16).enumerate() {
            for (grid_x, glyph) in row.iter().enumerate() {
                let fw = self.font.width as u32;
                let fh = self.font.height as u32;
                let gx = grid_x as u32;
                let gy = grid_y as u32;
                let offset_x = GRID_OFFS_X + gx * fw * 2;
                let offset_y = GRID_OFFS_Y + gy * fh * 2;

                let hovered = gx == self.hov_x && gy == self.hov_y;
                let selected = gx == self.sel_x && gy == self.sel_y;

                let border_color = if selected {
                    0x00aa00
                } else if hovered {
                    0x990000
                } else {
                    0x770000
                };

                self.fb.draw_rect_hollow(offset_x, offset_y, fw + 2, fh + 2, border_color);

                for y in 0..fh {
                    for x in 0..fw {
                        let color = {
                            if glyph.get(x as usize, y as usize) {
                                0xffffff
                            } else if selected {
                                0x848484
                            } else if hovered {
                                0x585858
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

    fn render_glyph_editor(&mut self) {
        let sel_idx = self.sel_y * 16 + self.sel_x;
        let sel_glyph = &self.font.glyphs[sel_idx as usize];
        let fh = self.font.height as u32;
        let fw = self.font.width as u32;

        for cell_y in 0..fh {
            for cell_x in 0..fw {
                let filled = sel_glyph.get(cell_x as usize, cell_y as usize);
                let color = if filled { 0xffffff } else { 0x111111 };

                let x = self.editor_offs_x + cell_x * EDITOR_CELL_SIZE;
                let y = self.editor_offs_y + cell_y * EDITOR_CELL_SIZE;

                self.fb.draw_square(x, y, EDITOR_CELL_SIZE, color);
                self.fb.draw_square_hollow(x, y, EDITOR_CELL_SIZE + 1, 0x222222);
            }
        }
    }

    pub fn events(&mut self, event: Event) {
        match event {
            Event::MouseMotion(x, y) => {
                self.detect_mouse_hover(x, y);
            }
            Event::MousePress(x, y) => {
                self.detect_mouse_hover(x, y);
                if self.mouse_in_glyphs_grid_area(x, y) {
                    self.sel_x = self.hov_x;
                    self.sel_y = self.hov_y;
                }
            }
            _ => (),
        }
    }

    fn mouse_in_glyphs_grid_area(&self, x: i32, y: i32) -> bool {
        let inx = (x - GRID_OFFS_X as i32) / 2 / self.font.width as i32 <= 16;
        let iny = (y - GRID_OFFS_Y as i32) / 2 / self.font.height as i32 <= 16;

        inx && iny
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
