extern crate bmp;
use tools::abs;
use self::bmp::{Pixel, Image};

pub struct V2 {
    pub x: i32,
    pub y: i32,
}

pub struct Line {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
}

pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl V2 {
    pub fn new(x: i32, y: i32) -> V2 {
        return V2{x: x, y: y};
    }
}

impl Line {
    pub fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Line {
        return Line{x0: x0, y0: y0, x1: x1, y1: y1};
    }
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        return Rect{x: x, y: y, w: w, h: h};
    }
}

// Traits section (define the interface of inherited struts)
pub trait Drawable {
    fn draw(&self, color: Pixel, img: &mut Image);
}

// Implementations of Structs (and Trait implements)
impl Drawable for Line {
    fn draw(&self, color: Pixel, img: &mut Image) {
        let mut steep = false;
        let mut xa = self.x0; let mut xb = self.x1;
        let mut ya = self.y0; let mut yb = self.y1;
        if abs(xa-xb) < abs(ya-yb) {
            xa = self.y0; xb = self.y1;
            ya = self.x0; yb = self.x1;
            steep = true;
        }
        if xa > xb {
            xa = self.x1; xb = self.x0;
            ya = self.y0; yb = self.y1;
        }
        let dx = xb-xa; let dy = yb-ya;
        let derror2 = abs(dy)*2;
        let mut error2 = 0;
        let mut y = ya;
        let cx = img.get_width() as i32;
        let cy = img.get_height() as i32;

        // draw all pixels correcting the Y as we travel
        for x in xa..xb {
            if x < 0 || x >= cx || y < 0 || y >= cy {
                continue;
            }
            if steep {
                img.set_pixel(y as u32, x as u32, color);
            } else {
                img.set_pixel(x as u32, y as u32, color);
            }
            error2 += derror2;
            if error2 > dx {
                if yb > ya { y += 1; } else { y -= 1 };
                error2 -= dx*2;
            }
        }
    }
}

impl Drawable for Rect {
    fn draw(&self, color: Pixel, img: &mut Image){
        let rx2 = self.x + self.w;
        let ry2 = self.y + self.h;
        Line{x0: self.x, y0: self.y, x1: rx2, y1: self.y}.draw(color, img);
        Line{x0: self.x, y0: self.y, x1: self.x, y1: ry2}.draw(color, img);
        Line{x0: rx2, y0: self.y, x1: rx2, y1: ry2}.draw(color, img);
        Line{x0: self.x, y0: ry2, x1: rx2, y1: ry2}.draw(color, img);
    }
}
