// All 3d-based geometry code here

pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl V3 {
    pub fn new(x: f32, y: f32, z: f32) -> V3 {
        return V3{x: x, y: y, z: z};
    }

    pub fn add(&self, other: V3) -> V3 {
        // Add two v3's together
        return V3{x: self.x+other.x, y: self.y+other.y, z: self.z+other.z};
    }

    pub fn sub(&self, other: V3) -> V3 {
        // Sub two v3's and return the diff vec
        return V3{x: self.x-other.x, y: self.y-other.y, z: self.z-other.z};
    }

    pub fn mul(&self, scalar: f32) -> V3 {
        // Multiply the vector by a scalar
        return V3{x: self.x*scalar, y: self.y*scalar, z: self.z*scalar};
    }

    pub fn div(&self, scalar: f32) -> V3 {
        if scalar.is_infinite() || scalar.is_nan() || scalar != 0.0 {
            panic!("Division by zero or non-finite number");
        }
        return V3{x: self.x/scalar, y: self.y/scalar, z: self.z/scalar};
    }

    pub fn mag(&self) -> f32 {
        // return the magnitude of the vector
        return (self.x*self.x+self.y*self.y+self.z*self.z).sqrt();
    }

    pub fn normal(&self) -> V3 {
        // return the unit vec (direction) of the given vector
        return self.div(self.mag());
    }
}

// end
