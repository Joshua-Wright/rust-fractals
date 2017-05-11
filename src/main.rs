// main.rs
extern crate rust_image_stuff;
extern crate imagefmt;
extern crate palette;

/*
RUSTFLAGS="-C target-feature=+ssse3" cargo run --release
RUSTFLAGS="-C target-feature=+avx" cargo run --release
*/

use palette::{Rgb, Hsv, Lch, Hue};
use palette::pixel::Srgb;

use rust_image_stuff::fractal::mandelbrot;
use rust_image_stuff::FractalCfg;


fn cmap_lch(x: f64) -> [u8; 3] {
    let start_color = Srgb::new(0.0, 1.0, 1.0);
    let lch_color: palette::Lch = palette::Rgb::from(start_color).into();
    let c: palette::Rgb = lch_color.shift_hue(((x*360.0) as f32).into()).into();
    c.to_pixel()
}

fn cmap_hsv(x: f64) -> [u8; 3] {
    let start_color = Srgb::new(0.0, 1.0, 1.0);
    let hsv_color: palette::Hsv = palette::Rgb::from(start_color).into();
    let c: palette::Rgb = hsv_color.shift_hue(((x*360.0) as f32).into()).into();
    c.to_pixel()
}

fn cmap_test(cmap: &Fn(f64) -> [u8; 3]) -> (usize, Vec<u8>) {
    let z = 512;
    let center = (z/2) as f64;

    let mut buf = vec![0; z*z*3];

    for i in 0usize..z {
        for j in 0usize..z {
            let x = i as f64;
            let y = j as f64;
            let dist = (( (x - center).powi(2) +  (y - center).powi(2) )*4.0).sqrt();
            let color = cmap((3.0*dist / (center)).sin());
            buf[(i*z + j)*3 + 0] = color[0];
            buf[(i*z + j)*3 + 1] = color[1];
            buf[(i*z + j)*3 + 2] = color[2];
        }
    }
    (z, buf)
}

fn main() {
    println!("Hello, world!");

    let (z, buf) = cmap_test(&cmap_hsv);
    imagefmt::write("test_hsv.png", z, z, imagefmt::ColFmt::RGB, &buf, imagefmt::ColType::Auto).unwrap();

    let (z, buf) = cmap_test(&cmap_lch);
    imagefmt::write("test_lch.png", z, z, imagefmt::ColFmt::RGB, &buf, imagefmt::ColType::Auto).unwrap();
    
    let z = 800;
    let mut buf = vec![0; z*z*3];
    // let buf2 = mandelbrot(z, z,512,0f32,0f32,1f32);
    let cfg = FractalCfg{
        max_iterations: 2048u32,
        center_r: -0.743643887037151, center_i: 0.131825904205330,
        zoom: 100.0, ..Default::default()
    };
    let buf2 = mandelbrot(&cfg);
    for idx in 0..(z*z) {
        buf[3*idx + 0] = (buf2[idx]) as u8;
        buf[3*idx + 1] = (buf2[idx]) as u8;
        buf[3*idx + 2] = (buf2[idx]) as u8;
    }
    println!("max {:?}", buf2.iter().cloned().fold(0f32/0f32, f32::max));
    println!("min {:?}", buf2.iter().cloned().fold(0f32/0f32, f32::min));
    imagefmt::write("test.png", z, z, imagefmt::ColFmt::RGB, &buf, imagefmt::ColType::Auto).unwrap();

}
