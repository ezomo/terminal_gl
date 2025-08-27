use std::io::{self, Write};

#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct ColoredCoord {
    pub x: i32,
    pub y: i32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub r: Vec<u8>,
    pub g: Vec<u8>,
    pub b: Vec<u8>,
    pub changed_coords: Vec<Coord>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            r: vec![0; width * height],
            g: vec![0; width * height],
            b: vec![0; width * height],
            changed_coords: Vec::with_capacity(2000),
        }
    }

    pub fn init(&mut self) {
        self.r.fill(0);
        self.g.fill(0);
        self.b.fill(0);
        self.changed_coords.clear();
        self.set_black();
    }

    pub fn set_black(&mut self) {
        print!("\x1b[2J\x1b[H");

        let rows = (self.height + 1) / 2;
        for row in 0..rows {
            for col in 0..self.width {
                let y_upper = row * 2;
                let y_lower = row * 2 + 1;

                let (ru, gu, bu) = if y_upper < self.height {
                    let idx = y_upper * self.width + col;
                    (self.r[idx], self.g[idx], self.b[idx])
                } else {
                    (0, 0, 0)
                };

                let (rl, gl, bl) = if y_lower < self.height {
                    let idx = y_lower * self.width + col;
                    (self.r[idx], self.g[idx], self.b[idx])
                } else {
                    (0, 0, 0)
                };

                print!(
                    "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀",
                    ru, gu, bu, rl, gl, bl
                );
            }
            println!("\x1b[0m");
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return;
        }

        let idx = (y as usize) * self.width + (x as usize);
        self.r[idx] = r;
        self.g[idx] = g;
        self.b[idx] = b;

        self.changed_coords.push(Coord { x, y });
    }

    pub fn set_pixels(&mut self, pixels: &mut Vec<ColoredCoord>) {
        while let Some(coord) = pixels.pop() {
            self.set_pixel(coord.x, coord.y, coord.r, coord.g, coord.b);
        }
    }

    pub fn clear(&mut self) {
        for coord in &self.changed_coords {
            if coord.x < 0
                || coord.x >= self.width as i32
                || coord.y < 0
                || coord.y >= self.height as i32
            {
                continue;
            }
            let idx = (coord.y as usize) * self.width + (coord.x as usize);
            self.r[idx] = 0;
            self.g[idx] = 0;
            self.b[idx] = 0;
        }
    }

    pub fn present(&mut self) {
        for coord in &self.changed_coords {
            let x = coord.x;
            let y = coord.y;

            if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                let row = y / 2;
                let y_upper = row * 2;
                let y_lower = row * 2 + 1;

                let (ru, gu, bu) = if y_upper >= 0 && y_upper < self.height as i32 {
                    let idx = (y_upper as usize) * self.width + (x as usize);
                    (self.r[idx], self.g[idx], self.b[idx])
                } else {
                    (0, 0, 0)
                };

                let (rl, gl, bl) = if y_lower >= 0 && y_lower < self.height as i32 {
                    let idx = (y_lower as usize) * self.width + (x as usize);
                    (self.r[idx], self.g[idx], self.b[idx])
                } else {
                    (0, 0, 0)
                };

                print!("\x1b[{};{}H", row + 1, x + 1);
                print!(
                    "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀",
                    ru, gu, bu, rl, gl, bl
                );
            }
        }
        self.changed_coords.retain(|coord| {
            let idx = (coord.y as usize) * self.width + (coord.x as usize);
            !(self.r[idx] == 0 && self.g[idx] == 0 && self.b[idx] == 0)
        });

        print!("\x1b[0m");
        io::stdout().flush().unwrap();
    }
}
