extern crate bmp;
extern crate rusterizer;
use bmp::Image;
use rusterizer::two::geometry::V2;
use rusterizer::colors::*;

/*
A bare bones program drawing some lines and rectangles
to a BMP context which then renders into a BMP file
*/

fn main() {
    let imgx = 1280;
    let imgy = 720;
    let mut img = Image::new(imgx as u32, imgy as u32);
    let purple = pix(149, 0, 163);
    let b = get_pixel(Color::Black);

    // Draw lines from the top left corner to pixels around the edges
    let linecount = 50.0;
    let spacing1 = (imgx as f32) / linecount; 
    let spacing2 = (imgy as f32) / linecount;
    let v = V2::new([0.0, 0.0]);

    for x in 0..50 {
        let xx = x as f32 * spacing1;
        let lerp = blend(b, purple, (x as f32/linecount));
        v.line_to([xx, imgy as f32]).draw(lerp, &mut img);
    }

    for y in 0..50 {
        let yy = y as f32 * spacing2;
        let lerp = blend(b, purple, (y as f32/linecount));
        v.line_to([imgx as f32, yy]).draw(lerp, &mut img);
    }
    let _ = img.save("lines.bmp");
}

// end
