#![feature(test)]
extern crate test;
use test::Bencher;
use test::black_box;
extern crate rand;
use rand::random;

pub fn work1(x: f64) -> f64 {
    x.sin().cos().log10()
}
pub fn work2(x: f64) -> f64 {
    x.cos().sin().log10().log2()
}

trait TestLooping {
    fn do_work(&self, x: f64) -> f64;
    fn full_loop(&self, iter: usize) -> f64 {
        (0..iter)
            .map(|x| self.do_work(x as f64))
            .sum()
    }
}

struct Impl1 {}
impl TestLooping for Impl1 {
    fn do_work(&self, x: f64) -> f64 {
        work1(x)
    }
}


struct Impl2 {}
impl TestLooping for Impl2 {
    fn do_work(&self, x: f64) -> f64 {
        work2(x)
    }
}

const ITERATIONS: usize = 10000;

#[bench]
fn bench_loop_trait(b: &mut Bencher) {
    b.iter(|| {
        let n = black_box(ITERATIONS);
        let looper: Box<TestLooping> = if random() {
            black_box(Box::new(Impl1{}))
        } else {
            black_box(Box::new(Impl2{}))
        };
        looper.full_loop(n)
    })
}

#[bench]
fn bench_loop_virtual(b: &mut Bencher) {
    b.iter(|| {
        let n = black_box(ITERATIONS);
        let looper: Box<TestLooping> = if random() {
            black_box(Box::new(Impl1{}))
        } else {
            black_box(Box::new(Impl2{}))
        };
        (0..n)
            .map(|x| looper.do_work(x as f64))
            .sum::<f64>()
    })
}

#[bench]
fn bench_loop_static(b: &mut Bencher) {
    b.iter(|| {
        let n = black_box(ITERATIONS);
        (0..n)
            .map(|x| work1(x as f64))
            .sum::<f64>()
    })
}
