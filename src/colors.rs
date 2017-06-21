// Color code and library
extern crate bmp;
use self::bmp::Pixel;

// Blend a color based on the value of `t`
pub fn blend(c1: Pixel, c2: Pixel, t: u32) -> Pixel {
    return Pixel{r: 255, g: 255, b: 255};
}
