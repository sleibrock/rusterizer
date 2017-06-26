// Crate defs
extern crate bmp;

// standard imports
use std::mem::swap;
use std::cmp::{min, max};

// external
use self::bmp::{Pixel, Image};

pub struct V2 {
    pub x: i32,
    pub y: i32,
}

pub struct Line {
    pub a: V2,
    pub b: V2,
}

pub struct Rect {
    pub p: V2,
    pub w: i32,
    pub h: i32,
}

pub struct Tri {
    pub v1: V2,
    pub v2: V2,
    pub v3: V2,
}

impl V2 {
    pub fn new(data: [i32; 2]) -> V2 {
        return V2{x: data[0], y: data[1]};
    }

    pub fn line_to(&self, data: [i32; 2]) -> Line {
        return Line{a:V2::new([self.x, self.y]), b:V2::new(data)};
    }

    pub fn segment(&self, data: &V2) -> Line {
        return Line{a:V2::new([self.x, self.y]), b:V2::new([data.x, data.y])};
    }

    pub fn add(&self, other: &V2) -> V2 {
        // Add two vectors together (currently unsafe for OVF)
        return V2{x: self.x+other.x, y: self.y+other.y};
    }

    pub fn sub(&self, other: &V2) -> V2 {
        // Sub two vectors together (unsafe OVF)
        return V2{x: self.x-other.x, y: self.y-other.y};
    }

    pub fn dot(&self, other: &V2) -> i32 {
        // Get the dot product (sum of all multiplications)
        return (self.x*other.x) + (self.y*other.y);
    }
}

impl Line {
    pub fn new(data0: [i32; 2], data1: [i32; 2]) -> Line {
        return Line{a: V2::new(data0), b: V2::new(data1)};
    }

    pub fn rect(&self) -> Rect {
        let mut tx = self.a.x;
        let mut ty = self.a.y;
        if self.b.x < tx || self.b.y < ty {
            tx = self.b.x;
            ty = self.b.y;
        }
        return Rect::new([tx, ty], (self.a.x-self.b.x).abs(), (self.a.y-self.b.y).abs());
    }
}

impl Rect {
    pub fn new(p: [i32; 2], w: i32, h: i32) -> Rect {
        return Rect{p: V2::new(p), w: w, h: h};
    }
}

impl Tri {
    pub fn new(v1: [i32; 2], v2: [i32; 2], v3: [i32; 2]) -> Tri {
        return Tri{v1: V2::new(v1), v2: V2::new(v2), v3: V2::new(v3)};
    }

    pub fn rect(&self) -> Rect {
        // Find the bounding rect of the triangle
        let tx = min(self.v1.x, min(self.v2.x, self.v3.x));
        let ty = min(self.v1.y, min(self.v2.y, self.v3.y));
        let bx = max(self.v1.x, max(self.v2.x, self.v3.x));
        let by = max(self.v1.y, max(self.v2.y, self.v3.y));
        return V2::new([tx, ty]).line_to([bx, by]).rect();
    }

    pub fn contains(&self, p: V2) -> bool {
        // Check if point is inside the triangle using Barycentric equations
        let v0 = self.v3.sub(&self.v1);
        let v1 = self.v2.sub(&self.v1);
        let v2 = p.sub(&self.v1);
        let dot00 = v0.dot(&v0);
        let dot01 = v0.dot(&v1);
        let dot02 = v0.dot(&v2);
        let dot11 = v1.dot(&v1);
        let dot12 = v1.dot(&v2);
        let i_d : f32 = 1.0 / (dot00*dot11-dot01*dot01) as f32;
        let u   : f32 = (dot11*dot02-dot01*dot12) as f32 * i_d;
        let v   : f32 = (dot00*dot12-dot01*dot02) as f32 * i_d;
        return (u >= 0.0) && (v >= 0.0) && (u + v < 1.0);
    }
}

// Traits section (define the interface of inherited struts)
pub trait Drawable {
    // Drawable is used for stroking shapes, not filling
    fn draw(&self, color: Pixel, img: &mut Image);
}

pub trait Fillable {
    // Fillable is used to fill entire shapes
    fn fill(&self, color: Pixel, img: &mut Image);
}

// Trait implements section
impl Drawable for Line {
    fn draw(&self, color: Pixel, img: &mut Image) {
        // Draw a line from A to B compensating Y as we travel
        // Begin by making sure the beginning A point is the minimum point
        // We want our accumulator to only increment
        let mut steep = false;
        let mut xa = self.a.x;
        let mut xb = self.b.x;
        let mut ya = self.a.y;
        let mut yb = self.b.y;
        if (xa-xb).abs() < (ya-yb).abs() {
            swap(&mut xa, &mut ya);
            swap(&mut xb, &mut yb);
            steep = true;
        }
        if xa > xb {
            swap(&mut xa, &mut xb);
            swap(&mut ya, &mut yb);
        }
        let dx = xb-xa;
        let dy = yb-ya;
        let derror2 = dy.abs()*2;
        let mut error2 = 0;
        let mut y = ya;
        let cx = img.get_width() as i32;
        let cy = img.get_height() as i32;

        // draw all pixels correcting the Y as we travel
        for x in xa..xb {
            // if a pixel is not within the canvas region, skip
            if x > 0 && x < cx && y > 0 && y < cy {
                if steep {
                    img.set_pixel(y as u32, x as u32, color);
                } else {
                    img.set_pixel(x as u32, y as u32, color);
                }
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
        // Draw lines around the rectangle given
        let rx2 = self.p.x + self.w;
        let ry2 = self.p.y + self.h;
        self.p.line_to([rx2, self.p.y]).draw(color, img);
        self.p.line_to([self.p.x, ry2]).draw(color, img);
        V2::new([self.p.x, ry2]).line_to([rx2, ry2]).draw(color, img);
        V2::new([rx2, self.p.y]).line_to([rx2, ry2]).draw(color, img);
    }
}

impl Fillable for Rect {
    fn fill(&self, color: Pixel, img: &mut Image){
        // Fill the rectangle with pixels one at a time
        let cx = img.get_width() as i32;
        let cy = img.get_height() as i32;
        for y in self.p.y..(self.p.y+self.h) {
            for x in self.p.x..(self.p.x+self.w) {
                if x < 0 || x >= cx || y < 0 || y >= cy {
                    continue;
                }
                img.set_pixel(x as u32, y as u32, color);
            }
        }
    }
}

impl Drawable for Tri {
    fn draw(&self, color: Pixel, img: &mut Image){
        // Draw the lines from each vert to create a stroke triangle
        self.v1.line_to([self.v2.x, self.v2.y]).draw(color, img);
        self.v2.line_to([self.v3.x, self.v3.y]).draw(color, img);
        self.v3.line_to([self.v1.x, self.v1.y]).draw(color, img);
    }
}

impl Fillable for Tri {
    fn fill(&self, color: Pixel, img: &mut Image){
        // Fill a triangle by checking each point in a rect
        let cx = img.get_width() as i32;
        let cy = img.get_width() as i32;
        let rect = self.rect();
        for y in rect.p.y..(rect.p.y+rect.h) {
            for x in rect.p.x..(rect.p.x+rect.w) {
                if x < 0 || x >= cx || y < 0 || y >= cy {
                    continue;
                }
                if !self.contains(V2::new([x,y])) {
                    continue;
                }
                img.set_pixel(x as u32, y as u32, color);
            }
        }
    }
}

// Unit test section for mathy stuff
#[cfg(test)]
mod test {
    use geometry::V2;
    #[test]
    fn test_add(){
        let a = V2::new([10, 20]);
        let b = V2::new([30, 30]);
        let c = a.add(&b);
        assert_eq!(c.x, 40);
        assert_eq!(c.y, 50);
    }

    #[test]
    fn test_sub() {
        let a = V2::new([100, 50]);
        let b = V2::new([50, 30]);
        let c = a.sub(&b);
        let d = b.sub(&a);
        assert_eq!(c.x, 50);
        assert_eq!(c.y, 20);
        assert_eq!(d.x, -50);
        assert_eq!(d.y, -20);
    }
    
    #[test]
    fn test_dotprod() {
        let a = V2::new([1, 3]);
        let b = V2::new([4, -2]);
        let d1 = a.dot(&b);
        let d2 = b.dot(&a);
        assert_eq!(-2, d1);
        assert_eq!(-2, d2);
    }
}
// end
