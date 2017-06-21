// assign 'drawing.rs' as the drawing module
pub mod geometry;
pub mod colors;

pub mod tools {
    // Return the absolute value of a signed integer
    pub fn abs(x:i32) -> i32 {
        return if x < 0 { -x } else { x };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
