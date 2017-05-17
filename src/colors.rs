// colors.rs
extern crate std;
use palette::{Rgb, Hsv, Hue};
use palette::pixel::Srgb;
use palette::blend::{Equations, Parameter};
use palette::gradient::Gradient;
use palette::Blend;
use std::boxed::Box;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::f32::consts::PI;

pub fn color_map_from_str(s: &str) -> Box<ColorMap> {
    match s {
        "hot" => Box::new(ColorMapHot{}),
        "hsv" => Box::new(ColorMapHSV{}),
        "cosine" => Box::new(ColorMap3dCosine{
            a: [0.5, 0.5, 0.5],
            b: [0.5, 0.5, 0.5],
            d: [0.477, 0.573, 0.637],
        }),
        x if x.starts_with("mapfile:") => Box::new(ColorMapFromFile::new(&x[8..])),
        x if x.starts_with("gpf:") => Box::new(ColorMapFromGPF::new(&x[4..])),
        _ => panic!("unknown colormap"),
    }
}

pub trait ColorMap {
    // x on range [0,1)
    fn colorize(&self, x: f32) -> (u8,u8,u8);
    fn colorize_buffer(&self, buf: Vec<f32>) -> Vec<u8> {
        let mut outbuf = vec![0u8; buf.len() * 3];
        for i in 0..buf.len() {
            let (r,g,b) = if buf[i] < 0f32 {
                (0,0,0)
            } else {
                self.colorize(buf[i])
            };
            outbuf[3*i + 0] = r;
            outbuf[3*i + 1] = g;
            outbuf[3*i + 2] = b;
        }
        outbuf
    }

}

pub struct ColorMapHSV {}
impl ColorMap for ColorMapHSV {
    fn colorize(&self, x: f32) -> (u8,u8,u8) {
        let start_color = Srgb::new(1.0, 0.0, 0.0);
        let hsv_color: Hsv = Rgb::from(start_color).into();
        let c: Rgb = hsv_color.shift_hue(((x*360.0) as f32).into()).into();
        c.to_pixel()
    }
}

pub struct ColorMapHot{}
impl ColorMap for ColorMapHot {
    fn colorize(&self, x: f32) -> (u8,u8,u8) {
        let x = (x * 255.0) as f64;
        let (r,g,b) = match x as i32 {
            0...94   => (51.0*x/19.0, 0.0, 0.0),
            95...190 => (255.0, (85.0*x - 8075.0)/32.0, 0.0),
            _        => (255.0, 255.0, 255.0 * x / 64.0 - 48705.0 / 64.0),
        };
        (r as u8, g as u8, b as u8)
    }
}

pub struct ColorMap3dCosine {
    pub a: [f32; 3],
    pub b: [f32; 3],
    pub d: [f32; 3],
}
impl ColorMap for ColorMap3dCosine {
    fn colorize(&self, x: f32) -> (u8,u8,u8) {
        let mut pix: [f32; 3] = [0f32; 3];
        for i in 0..3 {
            let a = self.a[i];
            let b = self.b[i];
            let d = self.d[i];
            pix[i] = 255f32 * (a + b * ((x + d)*2.0*PI).cos());
        }
        (pix[0] as u8, pix[1] as u8, pix[2] as u8)
    }
}

pub struct ColorMapFromFile {
    pub colors: Vec<(u8,u8,u8)>,
}
impl ColorMapFromFile {
    fn new(filepath: &str) -> ColorMapFromFile {
        let f = File::open(filepath).expect("failed to open colormap file");
        let file = BufReader::new(&f);
        let colors: Vec<(u8,u8,u8)> = file.lines()
            .map(|line| {
                let line = line.expect("failed to read colormap file");
                let xs: Vec<_> = line.split_whitespace()
                    .map(|x| x.parse().expect("failed to parse integer"))
                    .collect();
                (xs[0], xs[1], xs[2])
            }).collect();
        ColorMapFromFile {
            colors: colors,
        }
    }
}
impl ColorMap for ColorMapFromFile {
    fn colorize(&self, x: f32) -> (u8,u8,u8) {
        let x = x * (self.colors.len() as f32 - 1.0);
        let left: Rgb<f32>  = Rgb::from_pixel(&self.colors[x.floor() as usize]);
        let right = Rgb::from_pixel(&self.colors[x.ceil()  as usize]);
        let blend_mode = Equations::from_parameters(
            Parameter::SourceAlpha,
            Parameter::OneMinusSourceAlpha
        );
        left.blend(right, blend_mode).to_pixel()
    }
}


pub struct ColorMapFromGPF {
    pub gradient: Gradient<Rgb>,
}
impl ColorMapFromGPF {
    fn new(filepath: &str) -> ColorMapFromGPF {
        println!("{}", filepath);
        let f = File::open(filepath).expect("failed to open colormap file");
        let file = BufReader::new(&f);
        let colors: Vec<(f32, Rgb<f32>)> = file.lines()
            .map(|x| x.unwrap())
            .filter(|line| !line.starts_with("#"))
            .map(|line| {
                let xs: Vec<f32> = line.split_whitespace()
                    .map(|x| x.parse().expect("failed to parse float"))
                    .collect();
                (xs[0], Rgb::new(xs[1], xs[2], xs[3]))
            }).collect();
        ColorMapFromGPF {
            gradient: Gradient::with_domain(colors),
        }
    }
}
impl ColorMap for ColorMapFromGPF {
    fn colorize(&self, x: f32) -> (u8,u8,u8) {
        let (xmin, xmax) = self.gradient.domain();
        let x = xmin + (xmax - xmin)*x;
        self.gradient.get(x).to_pixel()
    }
}
