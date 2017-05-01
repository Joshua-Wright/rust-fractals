#![feature(step_by)]

/*
RUSTFLAGS="-C target-feature=+ssse3" cargo run
RUSTFLAGS="-C target-feature=+avx" cargo run
*/
extern crate imagefmt;
extern crate palette;
extern crate num;
// extern crate simd;
// extern crate simdty;
extern crate x86intrin;
#[macro_use] extern crate itertools;

use std::fs::File;
use std::path::Path;
use num::complex::Complex;

use palette::{Rgb, Hsv, Lch, Hue};
use palette::pixel::Srgb;

use x86intrin::f32x8;
use x86intrin::m256;
use x86intrin::avx::*;
use x86intrin::avx;


fn mandelbrot(width: usize, height: usize) -> Vec<f32> {
    let max_iterations: usize = 512;
    let mut buf = vec![0f32;width * height];
    
    // __m256 xmin = _mm256_set1_ps(bounds[0]);
    let xmin = mm256_set1_ps(-2.0f32);
    // __m256 ymin = _mm256_set1_ps(bounds[2]);
    let ymin = mm256_set1_ps(-2.0f32);
    // __m256 xscale = _mm256_set1_ps((bounds[1] - bounds[0]) / width);
    let xscale = mm256_set1_ps(4f32/(width as f32));
    // __m256 yscale = _mm256_set1_ps((bounds[3] - bounds[2]) / height);
    let yscale = mm256_set1_ps(4f32/(height as f32));
    // __m256 threshold = _mm256_set1_ps(max_iterations * max_iterations);
    let threshold = mm256_set1_ps((max_iterations as f32).powi(2));
    // __m256 one = _mm256_set1_ps(1);
    let one = mm256_set1_ps(1f32);
    
    for (x, y) in iproduct!( (0..width).step_by(8) , 0..height ) {
        // buf[i*x + j] = 0f32;

        // __m256 mx = _mm256_set_ps(x + 7, x + 6, x + 5, x + 4, x + 3, x + 2, x + 1, x + 0);
        let mx = mm256_set_ps(
            (x + 7) as f32,
            (x + 6) as f32,
            (x + 5) as f32,
            (x + 4) as f32,
            (x + 3) as f32,
            (x + 2) as f32,
            (x + 1) as f32,
            (x + 0) as f32
            );
        // __m256 my = _mm256_set1_ps(y);
        let my = mm256_set1_ps(y as f32);
        // __m256 cr = _mm256_add_ps(_mm256_mul_ps(mx, xscale), xmin);
        let cr = mm256_add_ps(mm256_mul_ps(mx, xscale), xmin);
        // __m256 ci = _mm256_add_ps(_mm256_mul_ps(my, yscale), ymin);
        let ci = mm256_add_ps(mm256_mul_ps(my, yscale), ymin);
        // __m256 zr = cr;
        let mut zr = cr;
        // __m256 zi = ci;
        let mut zi = ci;

        // __m256 mk = _mm256_set1_ps(k);
        let mut mk = mm256_set1_ps(1f32);
        for _ in 0..max_iterations {
            /* Compute z1 from z0 */
            // __m256 zr2 = _mm256_mul_ps(zr, zr);
            let mut zr2 = mm256_mul_ps(zr, zr);
            // __m256 zi2 = _mm256_mul_ps(zi, zi);
            let mut zi2 = mm256_mul_ps(zi, zi);
            // __m256 zrzi = _mm256_mul_ps(zr, zi);
            let zrzi = mm256_mul_ps(zr, zi);

            /* zr1 = zr0 * zr0 - zi0 * zi0 + cr */
            /* zi1 = zr0 * zi0 + zr0 * zi0 + ci */
            // zr = _mm256_add_ps(_mm256_sub_ps(zr2, zi2), cr);
            zr = mm256_add_ps(mm256_sub_ps(zr2, zi2), cr);
            // zi = _mm256_add_ps(_mm256_add_ps(zrzi, zrzi), ci);
            zi = mm256_add_ps(mm256_add_ps(zrzi, zrzi), ci);

            /* Increment k */
            // zr2 = _mm256_mul_ps(zr, zr);
            zr2 = mm256_mul_ps(zr, zr);
            zi2 = mm256_mul_ps(zi, zi);
            // __m256 mag2 = _mm256_add_ps(zr2, zi2);
            let mag2 = mm256_add_ps(zr2, zi2);
            // __m256 mask = _mm256_cmp_ps(mag2, threshold, _CMP_LT_OS);
            let mask = mm256_cmp_ps(mag2, threshold, CMP_LT_OS);
            // mk = _mm256_add_ps(_mm256_and_ps(mask, one), mk);
            mk = mm256_add_ps(mm256_and_ps(mask, one), mk);
            if (mm256_testz_ps(mask, mm256_set1_ps(-1f32)) == 1i32) {
                break;
            }
            // if (_mm256_testz_ps(mask, _mm256_set1_ps(-1))) {
            //     break;
            // }
        }
        let mx_f32x8 = mk.as_f32x8().as_array();
        for i in 0..8 {
            buf[y*height + x + i] = mx_f32x8[i];
        }
    }
    buf
}

fn cmap_lch(x: f64) -> [u8; 3] {
    let start_color = Srgb::new(0.0, 1.0, 1.0);
    let lch_color: palette::Lch = palette::Rgb::from(start_color).into();
    let c: palette::Rgb = lch_color.shift_hue(((x*360.0) as f32).into()).into();
    c.to_pixel()
}

fn cmap_hsv(x: f64) -> [u8; 3] {
    let start_color = Srgb::new(0.0, 1.0, 1.0);
    let hsv_color: palette::Hsv = palette::Rgb::from(start_color).into();
    let c: palette::Rgb = hsv_color.shift_hue(((x*360.0) as f32).into()).into();
    c.to_pixel()
}

fn cmap_test(cmap: &Fn(f64) -> [u8; 3]) -> (usize, Vec<u8>) {
    let z = 512;
    let center = (z/2) as f64;

    let mut buf = vec![0; z*z*3];

    for i in 0usize..z {
        for j in 0usize..z {
            let x = i as f64;
            let y = j as f64;
            let dist = (( (x - center).powi(2) +  (y - center).powi(2) )*4.0).sqrt();
            let color = cmap((3.0*dist / (center)).sin());
            buf[(i*z + j)*3 + 0] = color[0];
            buf[(i*z + j)*3 + 1] = color[1];
            buf[(i*z + j)*3 + 2] = color[2];
        }
    }
    (z, buf)
}

fn main() {
    println!("Hello, world!");

    let (z, buf) = cmap_test(&cmap_hsv);
    imagefmt::write("test_hsv.png", z, z, imagefmt::ColFmt::RGB, &buf, imagefmt::ColType::Auto).unwrap();

    let (z, buf) = cmap_test(&cmap_lch);
    imagefmt::write("test_lch.png", z, z, imagefmt::ColFmt::RGB, &buf, imagefmt::ColType::Auto).unwrap();
    
    let z = 800;
    let mut buf = vec![0; z*z*3];
    let buf2 = mandelbrot(z, z);
    for idx in 0..(z*z) {
        buf[3*idx + 0] = (buf2[idx]) as u8;
        buf[3*idx + 1] = (buf2[idx]) as u8;
        buf[3*idx + 2] = (buf2[idx]) as u8;
    }
    println!("max {:?}", buf2.iter().cloned().fold(0f32/0f32, f32::max));
    println!("min {:?}", buf2.iter().cloned().fold(0f32/0f32, f32::min));
    imagefmt::write("test.png", z, z, imagefmt::ColFmt::RGB, &buf, imagefmt::ColType::Auto).unwrap();

}
