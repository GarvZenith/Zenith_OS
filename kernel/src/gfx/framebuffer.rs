use alloc::vec;
use alloc::vec::Vec;
use spin::Mutex;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 10, g: 10, b: 18 };
    pub const DARK_SPACE: Color = Color { r: 15, g: 20, b: 35 };
    pub const CYAN: Color = Color { r: 0, g: 220, b: 240 };
    pub const NEURAL_PURPLE: Color = Color { r: 180, g: 70, b: 255 };
    pub const ELECTRIC_BLUE: Color = Color { r: 50, g: 130, b: 255 };
    pub const WHITE: Color = Color { r: 240, g: 245, b: 255 };
    pub const ACCENT_GREEN: Color = Color { r: 40, g: 220, b: 120 };

    pub fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![Color::DARK_SPACE.to_u32(); width * height],
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color.to_u32();
        }
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: Color) {
        for dy in 0..h {
            for dx in 0..w {
                self.draw_pixel(x + dx, y + dy, color);
            }
        }
    }

    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: Color) {
        let dx = (x1 as isize - x0 as isize).abs();
        let dy = (y1 as isize - y0 as isize).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut curr_x = x0 as isize;
        let mut curr_y = y0 as isize;

        loop {
            if curr_x >= 0 && curr_y >= 0 {
                self.draw_pixel(curr_x as usize, curr_y as usize, color);
            }

            if curr_x == x1 as isize && curr_y == y1 as isize {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                curr_x += sx;
            }
            if e2 < dx {
                err += dx;
                curr_y += sy;
            }
        }
    }

    pub fn clear(&mut self, color: Color) {
        let val = color.to_u32();
        for pixel in &mut self.buffer {
            *pixel = val;
        }
    }
}

lazy_static! {
    pub static ref FRAMEBUFFER: Mutex<Framebuffer> = Mutex::new(Framebuffer::new(320, 240));
}
