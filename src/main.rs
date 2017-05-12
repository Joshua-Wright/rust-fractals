// main.rs
#![crate_name="rust_image_stuff"]
extern crate rust_image_stuff;
extern crate imagefmt;

#[macro_use]
extern crate clap;
use clap::{Arg, App};

/*
RUSTFLAGS="-C target-feature=+avx" cargo run --release

RUSTFLAGS="-C target-feature=+avx" cargo run --release -- -r=-0.743643887037151 -i 0.131825904205330 --zoom 100 --iter 2048
*/

use rust_image_stuff::*;
use rust_image_stuff::colors::*;


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
        .arg(Arg::with_name("mul")
             .help("multiplier for colormap")
             .default_value("1")
             .short("m")
             )
        .get_matches();
    
    let cfg = FractalCfg::from_matches(&matches);
    let mul = value_t!(matches, "mul", f32).unwrap();

    let buf2 = mandelbrot(&cfg);
    println!("f32 max {:?}", buf2.iter().cloned().fold(std::f32::NAN, f32::max));
    println!("f32 min {:?}", buf2.iter().cloned().fold(std::f32::NAN, f32::min));

    let buf2 = normalize(buf2, mul);
    let buf = ColorMapHot{}.colorize_buffer(buf2);


    println!("u8 max {:?}", buf.iter().cloned().max());
    println!("u8 min {:?}", buf.iter().cloned().min());
    imagefmt::write("test.png", cfg.width as usize, cfg.height as usize, imagefmt::ColFmt::RGB, &buf, imagefmt::ColType::Auto).unwrap();

}

