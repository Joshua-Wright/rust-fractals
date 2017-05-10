// fractal.rs



use x86intrin::f32x8;
use x86intrin::m256;
use x86intrin::avx::*;
use x86intrin::avx;

// use std::num::complex::Complex;


fn calc_width(
        x:usize, y: usize,
        zoom: f32
    ) -> (f32,f32) {
    let x = x as f32;
    let y = y as f32;
    let dz = 4f32 / zoom;
    if x > y {
        (1f32 * x / y * dz, dz)
    } else {
        (dz, 1f32 * x / y * dz)
    }
}


pub fn mandelbrot(
        width: usize, height: usize,
        max_iterations: usize,
        center_r: f32, center_i: f32,
        zoom: f32,
    ) -> Vec<f32> {
    let mut buf = vec![0f32;width * height];
    
    let (xwidth, ywidth) = calc_width(width, height, zoom);
    let xscale = mm256_set1_ps(xwidth / (width as f32));
    let yscale = mm256_set1_ps(ywidth / (height as f32));
    let xmin = mm256_set1_ps(center_r - xwidth / 2f32);
    let ymin = mm256_set1_ps(center_i - ywidth / 2f32);

    let threshold = mm256_set1_ps((max_iterations as f32).powi(2));
    let one = mm256_set1_ps(1f32);
    
    for y in 0..height {
        for x in (0..(width/8)).map(|x| x*8) {

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
                let zr2 = mm256_mul_ps(zr, zr);
                // __m256 zi2 = _mm256_mul_ps(zi, zi);
                let zi2 = mm256_mul_ps(zi, zi);
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
                let zr2 = mm256_mul_ps(zr, zr);
                let zi2 = mm256_mul_ps(zi, zi);
                // __m256 mag2 = _mm256_add_ps(zr2, zi2);
                let mag2 = mm256_add_ps(zr2, zi2);
                // __m256 mask = _mm256_cmp_ps(mag2, threshold, _CMP_LT_OS);
                let mask = mm256_cmp_ps(mag2, threshold, CMP_LT_OS);
                // mk = _mm256_add_ps(_mm256_and_ps(mask, one), mk);
                mk = mm256_add_ps(mm256_and_ps(mask, one), mk);
                
                if (mm256_testz_ps(mask, mm256_set1_ps(-1f32)) == 1i32) {
                    break;
                }

                let mx_f32x8 = mk.as_f32x8().as_array();
                for i in 0..8 {
                    buf[y*height + x + i] = mx_f32x8[i];
                }
            }
        }
    }
    buf
}