// lib.rs
#![crate_name="rust_image_stuff"]
#![feature(test)]
extern crate test;

extern crate x86intrin;
extern crate palette;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate clap;
use clap::ArgMatches;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct FractalCfg {
    pub width: u32, pub height: u32,
    pub max_iterations: u32,
    pub center_r: f64, pub center_i: f64,
    pub zoom: f64,
}

impl Default for FractalCfg {
    fn default() -> Self {
        FractalCfg{
            width: 800u32, height: 800u32,
            max_iterations: 256u32,
            center_r: 0.0, center_i: 0.0,
            zoom: 1.0,
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
        }
    }
}

mod fractal;
pub use fractal::mandelbrot;

pub mod colors;

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
        // div by eps+1 to make sure it is in range [0,1), not [0,1]
        (0.5f32*(x * pi / 4f32).sin() + 0.5f32) / (1f32 + std::f32::EPSILON)
    }
}

pub fn normalize(xs: Vec<f32>, mul: f32) -> Vec<f32> {
    xs.into_iter()
        .map(log2)
        .map(|x| x*mul)
        .map(sin2)
        .collect()
}



#[cfg(test)]
mod tests {
    #[feature(test)]

    extern crate test;
    use test::Bencher;
    use test::black_box;
    use std::ops::Range;

    fn transform10(mag: f32, mx_f32x8: f32) -> f32 {
        let log_zn = mag.log10()/2f32;
        let nu = (log_zn / 2f32.log10()).log10() / 2f32.log10();
        mx_f32x8 + 1f32 - nu
    }

    fn transform2(mag: f32, mx_f32x8: f32) -> f32 {
        let log_zn = mag.log2()/2f32;
        let nu = log_zn.log2();
        mx_f32x8 + 1f32 - nu
    }

    #[bench]
    fn bench_log10(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(100);
            let mut sum = 0f32;
            for z in (40..(10*n)).map(|x| (x as f32)/10f32) {
                for c in (4..n).map(|x| x as f32) {
                    sum += transform10(z,c);
                }
            }
            sum
        })
    }

    #[bench]
    fn bench_log2(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(100);
            let mut sum = 0f32;
            for z in (40..(10*n)).map(|x| (x as f32)/10f32) {
                for c in (4..n).map(|x| x as f32) {
                    sum += transform2(z,c);
                }
            }
            sum
        })
    }

    #[test]
    fn test_log_same() {
        for z in (40..1000).map(|x| (x as f32)/10f32) {
            for c in (4..100).map(|x| x as f32) {
                let r1 = transform2(z,c);
                let r2 = transform10(z,c);
                println!("{} {}", r1, r2);
                assert!((r1 - r2).abs() < 0.00001);
            }
        }
    }

}
