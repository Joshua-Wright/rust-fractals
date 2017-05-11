// fractal.rs
use FractalCfg;
use x86intrin::avx::*;


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


fn smooth_iter(iter: f32, mag: f32) -> f32 {
    let log_zn = mag.log2()/2f32;
    let nu = log_zn.log2();
    iter + 1f32 - nu
}


pub fn mandelbrot(
        cfg: &FractalCfg        
    ) -> Vec<f32> {
    let width          = cfg.width    as usize;
    let height         = cfg.height   as usize;
    let center_r       = cfg.center_r as f32;
    let center_i       = cfg.center_i as f32;
    let zoom           = cfg.zoom     as f32;
    let max_iterations = cfg.max_iterations;

    let mut buf = vec![0f32; width * height];
    
    let (xwidth, ywidth) = calc_width(width, height, zoom);
    let xscale = mm256_set1_ps(xwidth / (width as f32));
    let yscale = mm256_set1_ps(ywidth / (height as f32));
    let xmin   = mm256_set1_ps(center_r - xwidth / 2f32);
    let ymin   = mm256_set1_ps(center_i - ywidth / 2f32);

    let threshold = mm256_set1_ps((max_iterations as f32).powi(2));
    let one = mm256_set1_ps(1f32);

    if width % 8 != 0 {
        panic!("Bad image size! width must be a multiple of 8");
    }
    
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
            let my = mm256_set1_ps(y as f32);
            let cr = mm256_add_ps(mm256_mul_ps(mx, xscale), xmin);
            let ci = mm256_add_ps(mm256_mul_ps(my, yscale), ymin);
            let mut zr = cr;
            let mut zi = ci;

            let mut mk = mm256_set1_ps(1f32);
            let mut mag2final = mm256_set1_ps(0f32);
            for _ in 0..max_iterations {
                /* Compute z1 from z0 */
                let zr2 = mm256_mul_ps(zr, zr);
                let zi2 = mm256_mul_ps(zi, zi);
                let zrzi = mm256_mul_ps(zr, zi);

                /* zr1 = zr0 * zr0 - zi0 * zi0 + cr */
                /* zi1 = zr0 * zi0 + zr0 * zi0 + ci */
                zr = mm256_add_ps(mm256_sub_ps(zr2, zi2), cr);
                zi = mm256_add_ps(mm256_add_ps(zrzi, zrzi), ci);

                /* Increment k */
                let zr2 = mm256_mul_ps(zr, zr);
                let zi2 = mm256_mul_ps(zi, zi);
                let mag2 = mm256_add_ps(zr2, zi2);
                let mask = mm256_cmp_ps(mag2, threshold, CMP_LT_OS);
                mk = mm256_add_ps(mm256_and_ps(mask, one), mk);
                // save the magnitude at the maximum iteration
                mag2final = mm256_or_ps(
                    mm256_and_ps(mask, mag2), 
                    mm256_andnot_ps(mask, mag2final));
                // we can't just use the magnitude at the end because 
                // the cells are iterated even when they're too large
               
                if mm256_testz_ps(mask, mm256_set1_ps(-1f32)) == 1i32 {
                    break;
                }

            }

            let mk = mk.as_f32x8().as_array();
            let mag2final = mag2final.as_f32x8().as_array();
            for i in 0..8 {
                buf[y*height + x + i] = smooth_iter(mk[i], mag2final[i]);
            }
        }
    }
    buf
}

