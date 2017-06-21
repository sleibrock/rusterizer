extern crate bmp;
extern crate rusterizer;
use bmp::{Image, Pixel};
use rusterizer::geometry::{Drawable, Line, Rect};

/*
A bare bones program drawing some lines and rectangles
to a BMP context which then renders into a BMP file
*/

fn main() {
    let imgx : u32 = 300;
    let imgy : u32 = 300;
    let mut img = Image::new(imgx, imgy);
    let white : Pixel = Pixel{r: 255, g: 255, b: 255};

    for x in 0..100 {
        Line::new(0, x*4,x*5, x*4).draw(white, &mut img);
    }
    
    for y in 0..10 {
        Rect{x: y*10, y: y*10, w: y*10, h: y*7}.draw(white, &mut img);
    }
    
    let _ = img.save("test.bmp");

}

// end
