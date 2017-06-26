extern crate bmp;
extern crate rusterizer;
use bmp::{Image};
use rusterizer::geometry::{Drawable, V2};
use rusterizer::colors::*;

/*
A bare bones program drawing some lines and rectangles
to a BMP context which then renders into a BMP file
*/

fn main() {
    let imgx = 300;
    let imgy = 300;
    let mut img = Image::new(imgx as u32, imgy as u32);
    let white = get_pixel(Color::White);

    // Draw lines from the top left corner to pixels around the edges
    let v = V2::new([0,0]);
    let b = get_pixel(Color::Black);
    for x in 0..50 {
        v.line_to([x*6, imgy]).draw(blend(b, white, (x as f32/50.0)), &mut img);
    }
    for x in 0..50 {
        v.line_to([imgx, x*6]).draw(blend(b, white, (x as f32/50.0)), &mut img);
    }

    let _ = img.save("test.bmp");
}

// end
