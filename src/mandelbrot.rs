use num_complex::Complex;

pub fn mandelbrot(c: Complex<f64>, max_iter: u32) -> u32 {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let mut n = 0;

    while z.norm_sqr() <= 4.0 && n < max_iter {
        z = z * z + c;
        n += 1;
    }

    n
}
