mod base;
mod map;
mod noise2d;
pub mod perlin;
pub mod random;
pub mod rigid;

pub use base::*;
pub use map::*;
pub use noise2d::*;

trait Noise {
    fn eval(&mut self, x: f64, y: f64) -> f64;

    fn chunk(&mut self, width: usize, height: usize) -> Vec<f64> {
        self.chunk_offset(width, height, 0, 0)
    }

    fn chunk_offset(
        &mut self,
        width: usize,
        height: usize,
        x_offset: usize,
        y_offset: usize,
    ) -> Vec<f64> {
        let mut pixels = vec![0.; width * height];

        for x in 0..width {
            for y in 0..height {
                let xx = (x + x_offset) as f64;
                let yy = (y + y_offset) as f64;
                pixels[x * width + y] = self.eval(xx, yy)
            }
        }

        pixels
    }
}
