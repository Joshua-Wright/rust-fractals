// lib.rs
#![feature(test)]
extern crate test;
extern crate x86intrin;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct FractalCfg {
    pub width: u32, pub height: u32,
    pub max_iterations: u32,
    pub center_r: f64, pub center_i: f64,
    pub zoom: f64,
}

impl Default for FractalCfg {
    fn default() -> FractalCfg {
        FractalCfg{
            width: 800u32, height: 800u32,
            max_iterations: 256u32,
            center_r: 0.0, center_i: 0.0,
            zoom: 1.0,
        }
    }
}

pub mod fractal;

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
        let iter = mx_f32x8 + 1f32 - nu;
        return iter;
    }

    fn transform2(mag: f32, mx_f32x8: f32) -> f32 {
        let log_zn = mag.log2()/2f32;
        let nu = log_zn.log2();
        let iter = mx_f32x8 + 1f32 - nu;
        return iter;
    }

    #[bench]
    fn bench_log10(b: &mut Bencher) {
        b.iter(|| { black_box((0..1000).map(|x| x as f32).fold(0f32, |old, new| 
            transform10(old, new)));
        });
    }

    #[bench]
    fn bench_log2(b: &mut Bencher) {
        b.iter(|| { black_box((0..1000).map(|x| x as f32).fold(0f32, |old, new|
            transform2(old, new)));
        });
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
