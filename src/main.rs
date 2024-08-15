use minifb::{Key, Window, WindowOptions};
use rayon::prelude::*;
use std::time::Instant;

mod mandelbrot;
mod color;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const MAX_ITER: u32 = 1000;
const ZOOM_FACTOR: f64 = 0.9;
const PAN_STEP: f64 = 0.1;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Mandelbrot Explorer",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Initial bounds
    let mut x_min = -2.0;
    let mut x_max = 1.0;
    let mut y_min = -1.5;
    let mut y_max = 1.5;
    let original_bounds = (x_min, x_max, y_min, y_max);

    let mut last_update = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if last_update.elapsed().as_millis() > 16 { // ~60 FPS
            last_update = Instant::now();

            // Compute Mandelbrot set
            buffer
                .par_chunks_mut(WIDTH)
                .enumerate()
                .for_each(|(y, row)| {
                    let y = y as f64;
                    for x in 0..WIDTH {
                        let x_f64 = x as f64;
                        let re = x_min + (x_f64 / WIDTH as f64) * (x_max - x_min);
                        let im = y_min + (y / HEIGHT as f64) * (y_max - y_min);
                        let c = num_complex::Complex::new(re, im);
                        let iter = mandelbrot::mandelbrot(c, MAX_ITER);
                        row[x] = color::color_map(iter, MAX_ITER);
                    }
                });

            // Update window
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

            // Handle zooming in (Up arrow)
            if window.is_key_down(Key::Up) {
                let x_center = (x_min + x_max) / 2.0;
                let y_center = (y_min + y_max) / 2.0;
                let x_range = (x_max - x_min) * ZOOM_FACTOR;
                let y_range = (y_max - y_min) * ZOOM_FACTOR;
                x_min = x_center - x_range / 2.0;
                x_max = x_center + x_range / 2.0;
                y_min = y_center - y_range / 2.0;
                y_max = y_center + y_range / 2.0;
            }

            // Handle zooming out (Down arrow)
            if window.is_key_down(Key::Down) {
                let x_center = (x_min + x_max) / 2.0;
                let y_center = (y_min + y_max) / 2.0;
                let x_range = (x_max - x_min) * (1.0 / ZOOM_FACTOR);
                let y_range = (y_max - y_min) * (1.0 / ZOOM_FACTOR);
                x_min = x_center - x_range / 2.0;
                x_max = x_center + x_range / 2.0;
                y_min = y_center - y_range / 2.0;
                y_max = y_center + y_range / 2.0;
            }

            // Handle panning
            if window.is_key_down(Key::W) {
                let y_range = y_max - y_min;
                y_min -= y_range * PAN_STEP;
                y_max -= y_range * PAN_STEP;
            }

            if window.is_key_down(Key::S) {
                let y_range = y_max - y_min;
                y_min += y_range * PAN_STEP;
                y_max += y_range * PAN_STEP;
            }

            if window.is_key_down(Key::A) {
                let x_range = x_max - x_min;
                x_min -= x_range * PAN_STEP;
                x_max -= x_range * PAN_STEP;
            }

            if window.is_key_down(Key::D) {
                let x_range = x_max - x_min;
                x_min += x_range * PAN_STEP;
                x_max += x_range * PAN_STEP;
            }

            // Reset view to original bounds (R key)
            if window.is_key_down(Key::R) {
                x_min = original_bounds.0;
                x_max = original_bounds.1;
                y_min = original_bounds.2;
                y_max = original_bounds.3;
            }
        }
    }
}
