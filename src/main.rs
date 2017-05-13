// main.rs
extern crate imagefmt;

#[macro_use]
extern crate clap;
use clap::{Arg, App};

extern crate serde_json;

/*
RUSTFLAGS="-C target-feature=+avx" cargo run --release

RUSTFLAGS="-C target-feature=+avx" cargo run --release -- -r=-0.743643887037151 -i 0.131825904205330 --zoom 100 --iter 2048
*/

extern crate fractals;
use fractals::*;
use fractals::colors::*;

use std::fs::File;
use std::io::prelude::*;


fn main() {
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
             .default_value("256")
             .long("iter")
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
             .long("iter")
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
             .long("mul")
             )
        .arg(Arg::with_name("output")
             .help("output filename")
             .default_value("output.png")
             .short("o")
             .long("output")
             .long("out")
             )
        .arg(Arg::with_name("cr")
             .default_value("0.0")
             .long("cr")
             )
        .arg(Arg::with_name("ci")
             .default_value("0.0")
             .long("ci")
             )
        .arg(Arg::with_name("julia")
             .help("render julia set instead of mandelbrot set")
             .short("j")
             .long("julia")
             .takes_value(false)
             )
        .get_matches();
    
    let cfg = FractalCfg::from_matches(&matches);
    let mul = value_t!(matches, "mul", f32).unwrap();
    let output = matches.value_of("output").unwrap();

    let buf2 = if matches.is_present("julia") {
        julia(&cfg)
    } else {
        mandelbrot(&cfg)
    };

    // println!("f32 max {:?}", buf2.iter().cloned().fold(std::f32::NAN, f32::max));
    // println!("f32 min {:?}", buf2.iter().cloned().fold(std::f32::NAN, f32::min));

    let buf2 = normalize(buf2, mul);
    let buf = ColorMapHot{}.colorize_buffer(buf2);


    // println!("u8 max {:?}", buf.iter().cloned().max());
    // println!("u8 min {:?}", buf.iter().cloned().min());
    imagefmt::write(output, cfg.width as usize, cfg.height as usize, imagefmt::ColFmt::RGB, &buf, imagefmt::ColType::Auto).unwrap();

    let mut outfile = File::create(output.to_owned() + ".json").unwrap();
    outfile.write_all(&serde_json::to_vec_pretty(&cfg).unwrap()).unwrap();

}

