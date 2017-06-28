extern crate bmp;
extern crate rusterizer;
use bmp::Image;
use rusterizer::colors::pix;

/*
A program to do bilinear interpolation (bilerp)
*/

fn main() {
    let imgx = 1280;
    let imgy = 720;
    let mut img = Image::new(imgx as u32, imgy as u32);
    let tl = 0.0 as f32;
    let tr = 255.0 as f32;
    let bl = 80.0  as f32;
    let br = 0.0   as f32;
    let fcx = imgx as f32;
    let fcy = imgy as f32;
    for y in 0..imgy {
        let yf = y as f32;
        for x in 0..imgx {
            let xf = x as f32;
            let a = (fcx-xf)/fcx;
            let b = xf/fcx;
            let res = ((((fcy-yf)/fcy)*((a*tl)+(b*tr)))+((yf/fcy)*((a*bl)+(b*br)))) as u8;
            img.set_pixel(x, y, pix(res, res, res));
        }
    }
    let _ = img.save("bilerp.bmp"); 
}

// end
