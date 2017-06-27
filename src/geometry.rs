// Crate defs
extern crate bmp;

// standard imports
use std::mem::swap;

// external
use self::bmp::{Pixel, Image};

pub struct V2 {
    pub x: f32,
    pub y: f32,
}

pub struct Line {
    pub a: V2,
    pub b: V2,
}

pub struct Rect {
    pub p: V2,
    pub w: f32,
    pub h: f32,
}

pub struct Tri {
    pub v1: V2,
    pub v2: V2,
    pub v3: V2,
}

impl V2 {
    pub fn new(data: [f32; 2]) -> V2 {
        return V2{x: data[0], y: data[1]};
    }

    pub fn line_to(&self, data: [f32; 2]) -> Line {
        // Create a line to a set of points (non-vector)
        return Line{a:V2::new([self.x, self.y]), b:V2::new(data)};
    }

    pub fn segment(&self, data: &V2) -> Line {
        // Create a line to another vector (vector used)
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

    pub fn mul(&self, scalar: f32) -> V2 {
        // multiply the vector by a scalar
        return V2{x: self.x*scalar, y: self.y*scalar};
    }

    pub fn div(&self, scalar: f32) -> V2 {
        // divide the vector by a scalar (must be nonzero/non-NaN)
        if scalar.is_infinite() || scalar.is_nan() || scalar != 0.0 {
            panic!("Division by zero or non-finite number");
        }
        return V2{x: self.x/scalar, y: self.y/scalar};
    }

    pub fn dot(&self, other: &V2) -> f32 {
        // Get the dot product (sum of all multiplications)
        return (self.x*other.x) + (self.y*other.y);
    }
}

impl Line {
    pub fn new(data0: [f32; 2], data1: [f32; 2]) -> Line {
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
    pub fn new(p: [f32; 2], w: f32, h: f32) -> Rect {
        return Rect{p: V2::new(p), w: w, h: h};
    }
}

impl Tri {
    pub fn new(v1: [f32; 2], v2: [f32; 2], v3: [f32; 2]) -> Tri {
        return Tri{v1: V2::new(v1), v2: V2::new(v2), v3: V2::new(v3)};
    }

    pub fn rect(&self) -> Rect {
        // Find the bounding rect of the triangle
        let tx = self.v1.x.min(self.v2.x).min(self.v3.x);
        let ty = self.v1.y.min(self.v2.y).min(self.v3.y);
        let bx = self.v1.x.max(self.v2.x).max(self.v3.x);
        let by = self.v1.y.max(self.v2.y).max(self.v3.y);
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
        let mut xa = self.a.x as i32;
        let mut xb = self.b.x as i32;
        let mut ya = self.a.y as i32;
        let mut yb = self.b.y as i32;
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
            // Check if the line is steep, and use xy accordingly
            if steep {
                if x >= 0 && x < cy && y >= 0 && y < cx { 
                    img.set_pixel(y as u32, x as u32, color);
                }
            } else {
                if x >= 0 && x < cx && y >= 0 && y < cy {
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
        let cx = img.get_width()  as i32;
        let cy = img.get_height() as i32;
        let rx = self.p.x as i32; let rw = self.w as i32;
        let ry = self.p.y as i32; let rh = self.h as i32;
        for y in ry..ry+rh {
            for x in rx..rx+rw { 
                if x >= 0 && x < cx && y >= 0 && y < cy { 
                    img.set_pixel(x as u32, y as u32, color);
                }
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
        let cy = img.get_height() as i32;
        let rect = self.rect();
        let rx = rect.p.x as i32; let rw = rect.w as i32;
        let ry = rect.p.y as i32; let rh = rect.h as i32;
        for y in ry..ry+rh {
            for x in rx..rx+rw {
                if x >= 0 && x < cx && y >= 0 && y < cy { 
                    if self.contains(V2::new([x as f32, y as f32])) {
                        img.set_pixel(x as u32, y as u32, color);
                    }
                }
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
        let a = V2::new([10.0, 20.0]);
        let b = V2::new([30.0, 30.0]);
        let c = a.add(&b);
        assert_eq!(c.x, 40.0);
        assert_eq!(c.y, 50.0);
    }

    #[test]
    fn test_sub() {
        let a = V2::new([100.0, 50.0]);
        let b = V2::new([50.0, 30.0]);
        let c = a.sub(&b);
        let d = b.sub(&a);
        assert_eq!(c.x, 50.0);
        assert_eq!(c.y, 20.0);
        assert_eq!(d.x, -50.0);
        assert_eq!(d.y, -20.0);
    }
    
    #[test]
    fn test_dotprod() {
        let a = V2::new([1, 3]);
        let b = V2::new([4, -2]);
        let d1 = a.dot(&b);
        let d2 = b.dot(&a);
        assert_eq!(-2.0, d1);
        assert_eq!(-2.0, d2);
    }

    #[test]
    fn test_draw_every_pixel() {
        // test drawing on every pixel to check proper edge case handling
    }
}
// end
