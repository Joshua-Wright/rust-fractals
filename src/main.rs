// main.rs
extern crate fractals;
use fractals::*;
extern crate imagefmt;
extern crate serde_json;
extern crate bincode;
extern crate clap;
use clap::{Arg, App};

/*
RUSTFLAGS="-C target-cpu=native" cargo build --bins --release && cp target/release/{fractals,julia_animation} .
*/

fn main() {
    let matches = App::new("mandelbrot and julia set")
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
        .arg(Arg::with_name("colormap")
             .help("colormap to use")
             .long("cmap")
             .default_value("hot")
             )
        .get_matches();
    
    let cfg = FractalCfg::from_matches(&matches);
    let output = matches.value_of("output").unwrap();

    write_fractal(&cfg, &output, matches.is_present("bin"), matches.is_present("quiet")).unwrap()
}

