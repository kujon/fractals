use num::complex::Complex;

pub trait Fractal {
    fn iterated_function(&self, z: Complex<f32>) -> Complex<f32>;

    fn orbit_point_escapes(&self, c: Complex<f32>) -> bool;

    fn belongs_to_set(&self, num_iterations: u8, z: Complex<f32>) -> bool {
        let mut i: u8 = 0;
        let mut c = self.iterated_function(z);
        while i < num_iterations && !self.orbit_point_escapes(c) {
            c = self.iterated_function(c);
            i = i + 1;
        }

        i == num_iterations - 1
    }
}

pub struct Mandelbrot {
    c: f32,
}

impl Fractal for Mandelbrot {
    fn iterated_function(&self, z: Complex<f32>) -> Complex<f32> {
        z.powi(2) + self.c
    }
    fn orbit_point_escapes(&self, c: Complex<f32>) -> bool {
        c.norm() > 2.
    }
}
