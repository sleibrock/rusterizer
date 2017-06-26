extern crate bmp;
extern crate rusterizer;
use bmp::{Image, Pixel};
use rusterizer::geometry::{Drawable, Tri};
use rusterizer::colors::blend;

/*
A program used to draw and fill in some triangles
 */

fn main(){
    let imgx = 300;
    let imgy = 300;
    let mut img = Image::new(imgx as u32, imgy as u32);

    let w = 40;
    let ww = 20;
    let h = 30;

    let black = get_pixel(Color::Black);
    let yellow = Pixel::new(220, 198, 0);

    for y in 0..12 {
        let ow = (y&1)*ww;
        let lerp = blend(yellow, black, (y as f32 / 11.0)); 
        for x in 0..10 {
            let t = Tri::new([x*w-ow, y*h], [x*w+ww-ow, y*h+h], [x*w+w-ow, y*h]);
            t.draw(lerp, &mut img);
        }
    }

    let _ = img.save("test.bmp");
}
// end
