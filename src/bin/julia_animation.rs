// julia_animation.rs
extern crate fractals;
use fractals::*;
extern crate imagefmt;
extern crate serde_json;
extern crate bincode;
#[macro_use]
extern crate clap;
use clap::{Arg, App};
use std::io::prelude::*;
use std::path::*;
use std::fs::*;
extern crate rayon;
use rayon::prelude::*;
use std::io::prelude::*;
use std::io::BufReader;

pub fn main() {
    let matches = App::new("julia animation")
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
             .default_value("frames")
             .short("o")
             .long("output")
             .long("out")
             )
        .arg(Arg::with_name("cr")
             .default_value("0.285")
             .long("cr")
             )
        .arg(Arg::with_name("ci")
             .default_value("0.01")
             .long("ci")
             )
        .arg(Arg::with_name("offset")
             .help("offset of color gradient")
             .default_value("0.0")
             .long("offset")
             )
        .arg(Arg::with_name("radius")
             .help("radius of circle")
             .default_value("0.01")
             .long("radius")
             )
        .arg(Arg::with_name("n_frames")
             .help("number of frames to render")
             .default_value("300")
             .long("frames")
             .short("n")
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
        .arg(Arg::with_name("points")
             .help("file containing points per frame")
             .long("points")
             .takes_value(true)
             )
        .arg(Arg::with_name("downsample")
             .help("downsample result from 4x resolution")
             .long("downsample")
             .takes_value(false)
             )
        .get_matches();
    
    let cfg      = FractalCfg { julia: true, .. FractalCfg::from_matches(&matches) };
    let output   = matches.value_of("output").unwrap();

    let points: Vec<_> = 
        if matches.is_present("points") {
            let f = File::open(matches.value_of("points").unwrap()).expect("failed to open points file");
            let file = BufReader::new(&f);
            file.lines()
                .map(|x| x.unwrap())
                .filter(|line| !line.starts_with("#"))
                .map(|line| {
                    let xs: Vec<f64> = line.split_whitespace()
                        .map(|x| x.parse().expect("failed to parse float"))
                        .collect();
                    (xs[0], xs[1])
                }).collect()
        } else {
            let n_frames = value_t!(matches, "n_frames", i32).unwrap();
            let radius   = value_t!(matches, "radius", f64).unwrap();
            (0..n_frames).into_iter()
            .map(|i| {
                let t = (i as f64) / (n_frames as f64) * 2.0 * std::f64::consts::PI;
                (
                    cfg.cr + t.cos() * radius,
                    cfg.ci + t.sin() * radius
                )
            }).collect()
        };

    julia_animation(&cfg, &points, &output);
}

fn julia_animation(cfg: &FractalCfg, points: &Vec<(f64,f64)>, output: &str) {

    // create directory if it doesn't already exist
    create_dir(output).unwrap_or(());
    
    let configs: Vec<_> = points.iter().enumerate()
        .map(|(i, &(cr, ci))| {
            let new_cfg = FractalCfg {
                cr: cr,
                ci: ci,
                .. cfg.clone()
            };
            let filename = format!("frame_{}.png", i);
            (new_cfg, filename)
        })
        .collect();

    configs.into_par_iter()
        .for_each(|(new_cfg, filename)| {
            print!("rendering {}...", filename);
            std::io::stdout().flush().unwrap();
            write_fractal(&new_cfg, Path::new(output).join(filename).to_str().unwrap(), false, true).unwrap();
            println!("done");
        });

    println!("ffmpeg -framerate 60 -y -i {}/frame_%d.png {}.mp4", output, output);
    std::process::Command::new("ffmpeg")
        .args(&["-framerate", "60", "-y", "-i", &format!("{}/frame_%d.png", output), &format!("{}.mp4", output)])
        .spawn()
        .expect("failed to spawn ffmpeg")
        .wait()
        .expect("ffmpeg failed");
}
