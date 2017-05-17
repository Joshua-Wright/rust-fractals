#![feature(test)]
extern crate test;
use test::Bencher;
use test::black_box;

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
