// Color code and library
extern crate bmp;
use self::bmp::Pixel;

pub enum Color {
    Black,
    White,
    Red,
    Blue,
    Green,
    Orange,
}

pub fn get_pixel(c: Color) -> Pixel {
    // Get basic colors from an enum
    match c {
        Color::Black  => Pixel::new(0,0,0),
        Color::White  => Pixel::new(255,255,255),
        Color::Red    => Pixel::new(255,0,0),
        Color::Green  => Pixel::new(0,255,0),
        Color::Blue   => Pixel::new(0,255,0),
        Color::Orange => Pixel::new(255,106,0),
    }
}

pub fn pix(r: u8, g: u8, b: u8) -> Pixel {
    // Shortcut for creating pixels
    return Pixel{r: r, g: g, b: b};
}

pub fn blend(c1: Pixel, c2: Pixel, t: f32) -> Pixel {
    // Blend a color based on the value of `t`
    let a = (1.0 - t) * (c1.r as f32) + (t * c2.r as f32);
    let b = (1.0 - t) * (c1.g as f32) + (t * c2.g as f32);
    let c = (1.0 - t) * (c1.b as f32) + (t * c2.b as f32);
    return Pixel{r: a as u8, g: b as u8, b: c as u8};
}

// end
