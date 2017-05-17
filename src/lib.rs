// lib.rs
extern crate x86intrin;
extern crate palette;
extern crate bincode;
extern crate imagefmt;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate clap;
use clap::ArgMatches;

use std::fs::File;
use std::io::prelude::*;

use std::f32::consts::PI;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FractalCfg {
    pub width: u32, pub height: u32,
    pub max_iterations: u32,
    pub center_r: f64, pub center_i: f64,
    pub zoom: f64,
    pub cr: f64, pub ci: f64,
    pub multiplier: f64,
    pub julia: bool,
    pub offset: f64,
    pub colormap: String,
}

impl Default for FractalCfg {
    fn default() -> Self {
        FractalCfg{
            width: 800u32, height: 800u32,
            max_iterations: 256u32,
            center_r: 0.0, center_i: 0.0,
            zoom: 1.0,
            cr: 0.0, ci: 0.0,
            multiplier: 1.0,
            julia: false,
            offset: 0f64,
            colormap: "hot".to_owned(),
        }
    }
}

pub trait FromMatches {
    fn from_matches(matches: &ArgMatches) -> Self;
}

impl FromMatches for FractalCfg {
    fn from_matches(matches: &ArgMatches) -> FractalCfg {
        let d = FractalCfg::default();
        FractalCfg {
            width: value_t!(matches, "width", u32).unwrap_or(d.width),
            height: value_t!(matches, "height", u32).unwrap_or(d.height),
            max_iterations: value_t!(matches, "iter", u32).unwrap_or(d.max_iterations),
            center_r: value_t!(matches, "r", f64).unwrap_or(d.center_r),
            center_i: value_t!(matches, "i", f64).unwrap_or(d.center_i),
            zoom: value_t!(matches, "zoom", f64).unwrap_or(d.zoom),
            cr: value_t!(matches, "cr", f64).unwrap_or(d.ci),
            ci: value_t!(matches, "ci", f64).unwrap_or(d.ci),
            multiplier: value_t!(matches, "multiplier", f64).unwrap_or(d.multiplier),
            julia: matches.is_present("julia"),
            offset: value_t!(matches, "offset", f64).unwrap_or(d.offset),
            colormap: value_t!(matches, "colormap", String).unwrap_or(d.colormap),
        }
    }
}

mod fractal;
pub use fractal::*;

pub mod colors;
use colors::*;

// offset on range [0,1)
pub fn normalize(xs: Vec<f32>, mul: f32, offset: f32) -> Vec<f32> {
    xs.into_iter()
        .map(|x| {
            if x < 0f32 {
                -1f32
            } else {
                let x = (x+1f32).log2();
                let x = x * mul;
                let x = x + offset;
                // // div by eps+1 to make sure it is in range [0,1), not [0,1]
                // let x = (0.5f32*(x * PI * 2f32).sin() + 0.5f32) / (1f32 + std::f32::EPSILON);
                // triangle waves look better than sine waves, because the colors aren't bunched
                // together
                // TODO: user-selectable waves just like colormaps
                let x = x % (1f32 + std::f32::EPSILON);
                let x = if x < 0.5 {
                    2.0*x
                } else {
                    2.0 - 2.0*x
                };
                x
            }
        })
        .collect()
}





pub fn write_fractal(cfg: &FractalCfg, output: &str, write_bin: bool, quiet: bool) -> std::io::Result<()> {

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
    let buf = color_map_from_str(&cfg.colormap).colorize_buffer(buf);

    if !quiet {
        println!("u8 max {:?}", buf.iter().cloned().max());
        println!("u8 min {:?}", buf.iter().cloned().min());
    }
    
    imagefmt::write(output, cfg.width as usize, cfg.height as usize, imagefmt::ColFmt::RGB, &buf, imagefmt::ColType::Auto).expect("error writing file");

    let mut outfile = File::create(metadata_file_path)?;
    outfile.write_all(&serde_json::to_vec_pretty(&cfg)?)
}

