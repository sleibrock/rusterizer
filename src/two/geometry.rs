// Geometry library for 2d space operations
extern crate bmp;

// Standard library imports
use std::mem::swap;
use std::ops::{Add, Sub, Mul, Neg};

// crate imports
use self::bmp::{Image, Pixel};

// A 2d vector consisting of only two types
pub struct V2 {
    pub x: f32,
    pub y: f32,
}

// A 2d line with two endpoints (A and B)
pub struct Line {
    pub a: V2,
    pub b: V2,
}

// A Rect represented as an origin with two width/height numbers
pub struct Rect {
    pub p: V2,
    pub w: f32,
    pub h: f32,
}

// A triangle with 3 verticies
pub struct Tri {
    pub v1: V2,
    pub v2: V2,
    pub v3: V2,
}

// 2d vector interface
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

    pub fn scale(&self, scalar: f32) -> V2 {
        // multiply the vector by a scalar (different from V2.mul)
        return V2{x: self.x*scalar, y: self.y*scalar};
    }

    pub fn divide(&self, scalar: f32) -> V2 {
        // divide the vector by a scalar (must be nonzero/non-NaN)
        if scalar.is_infinite() || scalar.is_nan() || scalar != 0.0 {
            panic!("Division by zero or non-finite number");
        }
        return V2{x: self.x/scalar, y: self.y/scalar};
    }

    pub fn mag(&self) -> f32 {
        // Return the magnitude of the vector
        return (self.x*self.x+self.y*self.y).sqrt();
    }

    pub fn normal(&self) -> V2 {
        // return the normal (unit) vector (vec/magnitude)
        return self.divide(self.mag());
    }
}

// Implement common unit traits for V2
impl Add for V2 {
    type Output = V2;
    fn add(self, other: V2) -> V2 {
        return V2{x: self.x+other.x, y: self.y+other.y};
    }
}

impl Sub for V2 {
    type Output = V2;
    fn sub(self, other: V2) -> V2 {
        return V2{x: self.x-other.x, y: self.y-other.y};
    }
}

impl Mul for V2 {
    type Output = f32;
    fn mul(self, other: V2) -> f32 {
        // Multiplication of vectors calculates the Dot product
        // use V2.scale to scale a vector by a scalar
        return (self.x*other.x) + (self.y*other.y);
    }
}

impl Neg for V2 {
    type Output = V2;
    fn neg(self) -> V2 {
        return V2{x: -self.x, y: -self.y};
    }
}

impl PartialEq for V2 {
    fn eq(&self, other: &V2) -> bool {
        return (self.x==other.x) && (self.y==other.y);
    }
}

// used for copy/cloning structs (Clone is a supertrait of Copy)
// needed when vars are moved from one reference to another
impl Copy for V2 {}
impl Clone for V2 {
    fn clone(&self) -> V2 {
        return *self;
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
        return Rect::new(
            [tx, ty],
            (self.a.x-self.b.x).abs(),
            (self.a.y-self.b.y).abs()
        );
    }

    pub fn draw(&self, color: Pixel, img: &mut Image) {
        // Draw a line from A to B compensating Y as we travel (Bresenham)
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
        // line drawing finished
    }
    // end line
}

impl Rect {
    pub fn new(p: [f32; 2], w: f32, h: f32) -> Rect {
        return Rect{p: V2::new(p), w: w, h: h};
    }

    pub fn draw(&self, color: Pixel, img: &mut Image){
        // Draw lines around the rectangle given
        let rx2 = self.p.x + self.w;
        let ry2 = self.p.y + self.h;
        self.p.line_to([rx2, self.p.y]).draw(color, img);
        self.p.line_to([self.p.x, ry2]).draw(color, img);
        V2::new([self.p.x, ry2]).line_to([rx2, ry2]).draw(color, img);
        V2::new([rx2, self.p.y]).line_to([rx2, ry2]).draw(color, img);
    }


    pub fn fill(&self, color: Pixel, img: &mut Image){
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
    // end rect
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
        let v0 = self.v3 - self.v1;
        let v1 = self.v2 - self.v1;
        let v2 = p - self.v1;
        let dot00 = v0 * v0;
        let dot01 = v0 * v1;
        let dot02 = v0 * v2;
        let dot11 = v1 * v1;
        let dot12 = v1 * v2;
        let i_d : f32 = 1.0 / (dot00*dot11-dot01*dot01) as f32;
        let u   : f32 = (dot11*dot02-dot01*dot12) as f32 * i_d;
        let v   : f32 = (dot00*dot12-dot01*dot02) as f32 * i_d;
        return (u >= 0.0) && (v >= 0.0) && (u + v < 1.0);
    }

    pub fn draw(&self, color: Pixel, img: &mut Image){
        // Draw the lines from each vert to create a stroke triangle
        self.v1.line_to([self.v2.x, self.v2.y]).draw(color, img);
        self.v2.line_to([self.v3.x, self.v3.y]).draw(color, img);
        self.v3.line_to([self.v1.x, self.v1.y]).draw(color, img);
    }
    
    pub fn fill(&self, color: Pixel, img: &mut Image){
        // Fill a triangle by checking each point in a rect
        let cx = img.get_width() as i32;
        let cy = img.get_height() as i32;
        let rect = self.rect();
        let rx = rect.p.x as i32; let rw = rect.w as i32;
        let ry = rect.p.y as i32; let rh = rect.h as i32;
        for y in ry..ry+rh {
            for x in rx..rx+rw {
                if x < 0 || x >= cx || y < 0 || y >= cy {
                    continue;
                }
                if !self.contains(V2::new([x as f32, y as f32])) {
                    continue;
                }
                img.set_pixel(x as u32, y as u32, color);
            }
        }
        // finish rendering
    }
    // end Tri
}

// end
