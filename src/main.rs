// main.rs
extern crate imagefmt;
extern crate serde_json;
extern crate bincode;
extern crate clap;
use clap::{Arg, App};


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
        .arg(Arg::with_name("multiplier")
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
        .arg(Arg::with_name("offset")
             .help("offset of color gradient")
             .default_value("0.0")
             .long("offset")
             )
        .arg(Arg::with_name("julia")
             .help("render julia set instead of mandelbrot set")
             .short("j")
             .long("julia")
             .takes_value(false)
             )
        .arg(Arg::with_name("bin")
             .help("also output bin of the image, for later recoloring")
             .short("b")
             .long("bin")
             .takes_value(false)
             )
        .arg(Arg::with_name("quiet")
             .help("supress info")
             .short("q")
             .long("quiet")
             .takes_value(false)
             )
        .get_matches();
    
    let cfg = FractalCfg::from_matches(&matches);
    let output = matches.value_of("output").unwrap();

    write_fractal(&cfg, &output, matches.is_present("bin"), matches.is_present("quiet")).unwrap()
}

fn write_fractal(cfg: &FractalCfg, output: &str, write_bin: bool, quiet: bool) -> std::io::Result<()> {

    let metadata_file_path = format!("{}.json", output);
    
    if let Ok(mut metadata_file) = File::open(&metadata_file_path) {
        let mut contents = vec![];
        metadata_file.read_to_end(&mut contents)?;
        if contents == serde_json::to_vec_pretty(&cfg).unwrap() {
            if !quiet {
                println!("found existing file {}", output);
            }
            return Ok(());
        }
    }

    let buf = if cfg.julia {
        julia(&cfg)
    } else {
        mandelbrot(&cfg)
    };

    if !quiet {
        println!("f32 max {:?}", buf.iter().cloned().fold(std::f32::NAN, f32::max));
        println!("f32 min {:?}", buf.iter().cloned().fold(std::f32::NAN, f32::min));
    }
    if write_bin {
        let bin_file_path = format!("{}.bin", output);
        let mut binfile = File::create(bin_file_path)?;
        binfile.write(&bincode::serialize(&buf, bincode::Infinite).unwrap())?;
    }

    let buf = normalize(buf, cfg.multiplier as f32, cfg.offset as f32);
    let buf = ColorMapHot{}.colorize_buffer(buf);

    if !quiet {
        println!("u8 max {:?}", buf.iter().cloned().max());
        println!("u8 min {:?}", buf.iter().cloned().min());
    }
    
    imagefmt::write(output, cfg.width as usize, cfg.height as usize, imagefmt::ColFmt::RGB, &buf, imagefmt::ColType::Auto).unwrap();

    let mut outfile = File::create(metadata_file_path)?;
    outfile.write_all(&serde_json::to_vec_pretty(&cfg)?)
}

