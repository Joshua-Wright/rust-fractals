// colormap_test.rs
extern crate fractals;
extern crate imagefmt;

use fractals::colors::color_map_from_str;

pub fn main() {
    // skip program name
    let args: Vec<_> = std::env::args().skip(1).collect();
    let mut cmaps = vec![
        "hot",
        "hsv",
        "cosine",
    ];
    for c in args.iter() {
        cmaps.push(c);
    }
    let wid = 80;
    let z = 800;
    let n = cmaps.len();
    let buf: Vec<_> = cmaps.iter()
        .flat_map(|s| {
            let cmap = color_map_from_str(s);
            let buf = cmap.colorize_buffer((0..z).map(|i| (i as f32)/(z as f32)).collect());
            buf.into_iter().cycle().take(z*wid*3)
        })
        .collect();
    println!("colors:");
    for s in cmaps.iter() {
        println!("    {}", s);
    }
    imagefmt::write("colormaps.png", z, n*wid, imagefmt::ColFmt::RGB, &buf, imagefmt::ColType::Auto).expect("error writing file");
}
