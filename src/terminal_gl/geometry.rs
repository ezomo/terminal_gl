use crate::terminal_gl::{Canvas, ColoredCoord};

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            *self
        } else {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        }
    }

    pub fn dot(&self, other: &Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            *self
        } else {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        }
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Color {
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const YELLOW: Color = Color {
        r: 255,
        g: 255,
        b: 0,
    };
    pub const CYAN: Color = Color {
        r: 0,
        g: 255,
        b: 255,
    };
    pub const MAGENTA: Color = Color {
        r: 255,
        g: 0,
        b: 255,
    };

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn lerp(&self, other: &Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color {
            r: (self.r as f32 * (1.0 - t) + other.r as f32 * t) as u8,
            g: (self.g as f32 * (1.0 - t) + other.g as f32 * t) as u8,
            b: (self.b as f32 * (1.0 - t) + other.b as f32 * t) as u8,
        }
    }

    pub fn multiply(&self, factor: f32) -> Color {
        Color {
            r: ((self.r as f32 * factor).min(255.0).max(0.0)) as u8,
            g: ((self.g as f32 * factor).min(255.0).max(0.0)) as u8,
            b: ((self.b as f32 * factor).min(255.0).max(0.0)) as u8,
        }
    }
}

// Bresenhamのライン描画アルゴリズム (C++版を参考)
pub fn draw_line(
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    canvas: &Canvas,
    color: Color,
    pixels: &mut Vec<ColoredCoord>,
) {
    let mut x0 = x0;
    let mut y0 = y0;
    let mut x1 = x1;
    let mut y1 = y1;

    let steep = (x0 - x1).abs() < (y0 - y1).abs();

    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y0;

    for x in x0..=x1 {
        if steep {
            if y >= 0 && y < canvas.width as i32 && x >= 0 && x < canvas.height as i32 {
                pixels.push(ColoredCoord {
                    x: y,
                    y: x,
                    r: color.r,
                    g: color.g,
                    b: color.b,
                });
            }
        } else {
            if x >= 0 && x < canvas.width as i32 && y >= 0 && y < canvas.height as i32 {
                pixels.push(ColoredCoord {
                    x,
                    y,
                    r: color.r,
                    g: color.g,
                    b: color.b,
                });
            }
        }

        error2 += derror2;

        if error2 > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error2 -= dx * 2;
        }
    }
}

// 三角形描画（ワイヤーフレーム）
pub fn draw_triangle_wireframe(
    p0: Vec2,
    p1: Vec2,
    p2: Vec2,
    canvas: &Canvas,
    color: Color,
    pixels: &mut Vec<ColoredCoord>,
) {
    draw_line(
        p0.x as i32,
        p0.y as i32,
        p1.x as i32,
        p1.y as i32,
        canvas,
        color,
        pixels,
    );
    draw_line(
        p1.x as i32,
        p1.y as i32,
        p2.x as i32,
        p2.y as i32,
        canvas,
        color,
        pixels,
    );
    draw_line(
        p2.x as i32,
        p2.y as i32,
        p0.x as i32,
        p0.y as i32,
        canvas,
        color,
        pixels,
    );
}

// 塗りつぶし三角形（シンプルな実装）
pub fn draw_triangle_filled(
    mut p0: Vec2,
    mut p1: Vec2,
    mut p2: Vec2,
    canvas: &Canvas,
    color: Color,
    pixels: &mut Vec<ColoredCoord>,
) {
    // Y座標でソート
    if p0.y > p1.y {
        std::mem::swap(&mut p0, &mut p1);
    }
    if p1.y > p2.y {
        std::mem::swap(&mut p1, &mut p2);
    }
    if p0.y > p1.y {
        std::mem::swap(&mut p0, &mut p1);
    }

    let total_height = p2.y - p0.y;
    if total_height < 0.1 {
        return;
    }

    for y in (p0.y as i32)..=(p2.y as i32) {
        let second_half = y as f32 > p1.y || p1.y == p0.y;
        let segment_height = if second_half {
            p2.y - p1.y
        } else {
            p1.y - p0.y
        };

        if segment_height < 0.1 {
            continue;
        }

        let alpha = (y as f32 - p0.y) / total_height;
        let beta = (y as f32 - if second_half { p1.y } else { p0.y }) / segment_height;

        let mut a = p0.x + (p2.x - p0.x) * alpha;
        let mut b = if second_half {
            p1.x + (p2.x - p1.x) * beta
        } else {
            p0.x + (p1.x - p0.x) * beta
        };

        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        for x in (a as i32)..=(b as i32) {
            if x >= 0 && x < canvas.width as i32 && y >= 0 && y < canvas.height as i32 {
                pixels.push(ColoredCoord {
                    x,
                    y,
                    r: color.r,
                    g: color.g,
                    b: color.b,
                });
            }
        }
    }
}

// 円描画
pub fn draw_circle(
    center_x: i32,
    center_y: i32,
    radius: i32,
    canvas: &Canvas,
    color: Color,
    pixels: &mut Vec<ColoredCoord>,
) {
    let mut x = 0;
    let mut y = radius;
    let mut d = 3 - 2 * radius;

    let plot_circle_points =
        |x: i32, y: i32, cx: i32, cy: i32, pixels: &mut Vec<ColoredCoord>, color: Color| {
            let points = [
                (cx + x, cy + y),
                (cx - x, cy + y),
                (cx + x, cy - y),
                (cx - x, cy - y),
                (cx + y, cy + x),
                (cx - y, cy + x),
                (cx + y, cy - x),
                (cx - y, cy - x),
            ];

            for (px, py) in points.iter() {
                if *px >= 0 && *px < canvas.width as i32 && *py >= 0 && *py < canvas.height as i32 {
                    pixels.push(ColoredCoord {
                        x: *px,
                        y: *py,
                        r: color.r,
                        g: color.g,
                        b: color.b,
                    });
                }
            }
        };

    plot_circle_points(x, y, center_x, center_y, pixels, color);

    while y >= x {
        x += 1;
        if d > 0 {
            y -= 1;
            d = d + 4 * (x - y) + 10;
        } else {
            d = d + 4 * x + 6;
        }
        plot_circle_points(x, y, center_x, center_y, pixels, color);
    }
}
