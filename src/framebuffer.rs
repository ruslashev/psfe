pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let length = (width * height).try_into().unwrap();
        let mut pixels = Vec::new();

        pixels.resize(length, 0);

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let idx = y * self.width + x;
        self.pixels[idx as usize] = (color << 8) | 0xff;
    }

    pub fn draw_rect_hollow(&mut self, x: u32, y: u32, w: u32, h: u32, color: u32) {
        for horz in 0..w {
            self.draw_pixel(x + horz, y, color);
            self.draw_pixel(x + horz, y + h - 1, color);
        }

        for vert in 0..h {
            self.draw_pixel(x, y + vert, color);
            self.draw_pixel(x + w - 1, y + vert, color);
        }
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: u32) {
        for dy in 0..h {
            for dx in 0..w {
                self.draw_pixel(x + dx, y + dy, color);
            }
        }
    }

    pub fn draw_square(&mut self, x: u32, y: u32, size: u32, color: u32) {
        self.draw_rect(x, y, size, size, color);
    }

    pub fn draw_square_hollow(&mut self, x: u32, y: u32, size: u32, color: u32) {
        self.draw_rect_hollow(x, y, size, size, color);
    }
}
