# rust-fractals
mandelbrot set in rust. 

Uses AVX intrinsics to speed up render. This means it requires nightly rust and a CPU that supports AVX instructions.

```
USAGE:
    fractals [FLAGS] [OPTIONS]

FLAGS:
    -b, --bin        also output bin of the image, for later recoloring
    -h, --help       Prints help information
    -j, --julia      render julia set instead of mandelbrot set
    -q, --quiet      supress info
    -V, --version    Prints version information

OPTIONS:
        --ci <ci>              [default: 0.0]
        --cr <cr>              [default: 0.0]
    -y <height>               height of image [default: 800]
    -i, --iter <i>            imaginary value of center point [default: 0]
        --iter <iter>         iteration count [default: 256]
    -m, --mul <multiplier>    multiplier for colormap [default: 1]
        --offset <offset>     offset of color gradient [default: 0.0]
    -o, --out <output>        output filename [default: output.png]
    -r <r>                    real value of center point [default: 0]
    -x <width>                width of image [default: 800]
        --zoom <zoom>         zoom [default: 1]
```
