// colors.rs
use palette::{Rgb, Hsv, Hue};
use palette::pixel::Srgb;
use std::boxed::Box;

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

pub fn color_map_from_str(s: &str) -> Box<ColorMap> {
    match s {
        "hot" => Box::new(ColorMapHot{}),
        "hsv" => Box::new(ColorMapHSV{}),
        "cosine" => Box::new(ColorMap3dCosine{
            a: [0.5, 0.5, 0.5],
            b: [0.5, 0.5, 0.5],
            c: [9.6, 9.6, 9.6],
            d: [3.0, 3.6, 4.0],
        }),
        _ => Box::new(ColorMapHot{}),
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
    pub c: [f32; 3],
    pub d: [f32; 3],
}
impl ColorMap for ColorMap3dCosine {
    fn colorize(&self, x: f32) -> (u8,u8,u8) {
        let mut pix: [f32; 3] = [0f32; 3];
        for i in 0..3 {
            let a = self.a[i];
            let b = self.b[i];
            let c = self.c[i];
            let d = self.d[i];
            pix[i] = 255f32 * (a + b * (c*x + d).cos());
        }
        (pix[0] as u8, pix[1] as u8, pix[2] as u8)
    }
}

