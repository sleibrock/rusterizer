extern crate bmp;
extern crate rusterizer;
use bmp::Image;
use rusterizer::geometry::{Drawable, Fillable, Tri};
use rusterizer::colors::*;

/*
A program used to draw and fill in some triangles
 */

fn main(){
    let imgx = 1280;
    let imgy = 720;
    let mut img = Image::new(imgx as u32, imgy as u32);

    let w = (imgx as f32) / 10.0;
    let ww = w / 2.0;
    let h = (imgy as f32) / 10.0;

    let black = get_pixel(Color::Black);
    let yellow  = pix(220, 198, 0);
    let yellow2 = blend(yellow, black, 0.9);

    for y in 0..12 {
        let yf = y as f32;
        let ow = ((y&1) as f32)*ww;
        let lerp  = blend(yellow,  black, (yf / 11.0)); 
        let lerp2 = blend(yellow2, black, (yf / 11.0)); 
        for x in 0..12 {
            let xf = x as f32;
            let a = xf*w-ow;
            let b = yf*h;
            let c = a+ww;
            let d = b+h;
            let e = a+w;
            let t = Tri::new([a, b], [c, d], [e, b]);
            t.fill(lerp2, &mut img);
            t.draw(lerp, &mut img);
        }
    }

    let _ = img.save("triangles.bmp");
}
// end
