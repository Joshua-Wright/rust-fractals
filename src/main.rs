// main.rs
#![crate_name="rust_image_stuff"]
extern crate rust_image_stuff;
extern crate imagefmt;
extern crate palette;

extern crate clap;
use clap::{Arg, App};

/*
RUSTFLAGS="-C target-feature=+avx" cargo run --release

RUSTFLAGS="-C target-feature=+avx" cargo run --release -- -r=-0.743643887037151 -i 0.131825904205330 --zoom 100 --iter 2048
*/

use palette::{Rgb, Hsv, Lch, Hue};
use palette::pixel::Srgb;

use rust_image_stuff::*;
use rust_image_stuff::colors::*;

fn cmap_hsv(x: f64) -> (u8,u8,u8){
    let start_color = Srgb::new(1.0, 0.0, 0.0);
    let hsv_color: palette::Hsv = palette::Rgb::from(start_color).into();
    let c: palette::Rgb = hsv_color.shift_hue(((x*360.0) as f32).into()).into();
    c.to_pixel()
}

fn log2(x: f32) -> f32 {
    if x < 0.0 {
        -1f32
    } else {
        (x+1f32).log2()
    }
}

fn sin2(x: f32) -> f32 { 
    let pi = std::f32::consts::PI;
    if x < 0.0 {
        -1f32
    } else {
        0.5f32*(x * pi / 4f32).sin() + 0.5f32
    }
}



fn main() {
    println!("Hello, world!");
    let matches = App::new("mandelbrot")
        .arg(Arg::with_name("width")
             .help("width of image")
             .short("x")
             .default_value("800")
             )
        .arg(Arg::with_name("height")
             .help("height of image")
             .short("y")
             .default_value("800")
             )
        .arg(Arg::with_name("iter")
             .help("iteration count")
             .long("iter")
             .default_value("256")
             )
        .arg(Arg::with_name("r")
             .help("real value of center point")
             .default_value("0")
             .short("r")
             )
        .arg(Arg::with_name("i")
             .help("imaginary value of center point")
             .default_value("0")
             .short("i")
             )
        .arg(Arg::with_name("zoom")
             .help("zoom")
             .default_value("1")
             .long("zoom")
             )
        .get_matches();
    
    let z = 800;
    let cfg = FractalCfg::from_matches(&matches);
    let buf2 = mandelbrot(&cfg);
    println!("max {:?}", buf2.iter().cloned().fold(std::f32::NAN, f32::max));
    println!("min {:?}", buf2.iter().cloned().fold(std::f32::NAN, f32::min));

    // let buf = ColorMapHSV{}.colorize_buffer(buf2.into_iter().map(|x| x.log2()).collect());
    // let buf = ColorMapHot{}.colorize_buffer(buf2.into_iter().map(|x| x.log2()).collect());
    // let buf2 = buf2.into_iter().map(|x| x.sin()).collect();
    let buf2: Vec<f32> = buf2.into_iter()
        .map(log2)
        .map(sin2)
        .collect();
    println!("max {:?}", buf2.iter().cloned().fold(std::f32::NAN, f32::max));
    println!("min {:?}", buf2.iter().cloned().fold(std::f32::NAN, f32::min));
    let buf = ColorMapHot{}.colorize_buffer(buf2);


    println!("u8 max {:?}", buf.iter().cloned().max());
    println!("u8 min {:?}", buf.iter().cloned().min());
    imagefmt::write("test.png", z, z, imagefmt::ColFmt::RGB, &buf, imagefmt::ColType::Auto).unwrap();

}

