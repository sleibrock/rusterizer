extern crate bmp;
extern crate rusterizer;
use bmp::{Image};
use rusterizer::geometry::{Drawable, Fillable, Tri};
use rusterizer::colors::*;

/*
A program used to draw and fill in some triangles
 */

fn main(){
    let imgx = 300;
    let imgy = 300;
    let mut img = Image::new(imgx as u32, imgy as u32);
    let white = get_pixel(Color::White);
    let oj    = get_pixel(Color::Orange);
    let red   = get_pixel(Color::Red);
    
    for y in 0..8 {
        let c = pixel(100, 200, 80);
        for x in 0..8 {
            let xw = y * 10;
            let yh = y * 30;
            let xw2 = xw >> 1;
            let t = Tri::new([x*xw, y], [x*xw2, y+yh], [(x*xw)+xw, y]);
            t.fill(c, &mut img);
        }
    }

    let _ = img.save("test.bmp");
}
// end
