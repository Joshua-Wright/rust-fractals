# rust-fractals
mandelbrot set in rust. 

Uses AVX intrinsics to speed up render. This means it requires nightly rust and a CPU that supports AVX instructions.

```
USAGE:
    fractals [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -y <height>          height of image [default: 800]
    -i <i>               imaginary value of center point [default: 0]
        --iter <iter>    iteration count [default: 256]
    -m <mul>             multiplier for colormap [default: 1]
    -o <output>          output filename [default: output.png]
    -r <r>               real value of center point [default: 0]
    -x <width>           width of image [default: 800]
        --zoom <zoom>    zoom [default: 1]
```
