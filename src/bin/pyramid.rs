extern crate bmp;
extern crate rusterizer;
use bmp::Image;
use rusterizer::geometry::{Drawable, V2};
use rusterizer::colors::*;

/*
Draw a psuedo line style blockish building thing
(I don't know how else to describe it)
 */

fn main(){
    let imgx = 1280;
    let imgy = 720;

    let mut img = Image::new(imgx as u32, imgy as u32);
    let white = get_pixel(Color::White);
    let grey = blend(white, pix(128, 128, 128), 0.5);

    let pad: f32 = 50.0;
    let lines    = 30;
    let xrange   = (imgx as f32) - (2.0 * pad);
    let yrange   = (imgy as f32) - (2.0 * pad);
    let xdiff    = xrange / (lines as f32);
    let ydiff    = yrange / (lines as f32);

    for x in 0..lines {
        let xf = x as f32;
        let a = xf+pad;
        let b = xf*ydiff+pad;
        let c = a + (xf*xdiff);
        let d = (xf*3.0)+pad;
        V2::new([a+xf, b]).line_to([c, b]).draw(white, &mut img);

        V2::new([c, b]).line_to([c, d]).draw(grey, &mut img);
    }

    let _ = img.save("pyramid.bmp");
}
