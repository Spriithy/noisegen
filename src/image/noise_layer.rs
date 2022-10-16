use itertools::Itertools;

use image::{Luma, Rgb, Rgba};

use crate::{
    noise::Noise,
    utils::{
        maths::{Lerp, LerpInv},
        MinMax,
    },
};

use super::Layer;

pub struct NoiseLayer {
    width: usize,
    height: usize,
    noise: Vec<f64>,
    bounds: (f64, f64),
}

impl NoiseLayer {
    pub fn new(width: usize, height: usize, noise: &mut Box<dyn Noise>) -> Self {
        let noise = noise.chunk(width, height);
        let bounds = noise.min_max().unwrap();

        Self {
            width,
            height,
            noise,
            bounds,
        }
    }

    fn at(&self, x: u32, y: u32) -> f64 {
        self.noise[(x as usize) * self.width + (y as usize)]
    }

    fn at_norm(&self, x: u32, y: u32) -> f64 {
        self.at(x, y).lerp_inv(self.bounds.0, self.bounds.1)
    }
}

impl Layer<Luma<u8>> for NoiseLayer {
    fn shape(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn read(&self, x: u32, y: u32) -> Luma<u8> {
        let pixel = self.at_norm(x, y).lerp(u8::MIN as f64, u8::MAX as f64) as u8;
        Luma([pixel])
    }

    fn pixels(&self) -> Vec<Luma<u8>> {
        Itertools::cartesian_product(0..(self.width as u32), 0..(self.height as u32))
            .map(|(x, y)| self.read(x, y))
            .collect()
    }
}

impl Layer<Rgb<u8>> for NoiseLayer {
    fn shape(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn read(&self, x: u32, y: u32) -> Rgb<u8> {
        let pixel = self.at_norm(x, y).lerp(u8::MIN as f64, u8::MAX as f64) as u8;
        Rgb([pixel; 3])
    }

    fn pixels(&self) -> Vec<Rgb<u8>> {
        Itertools::cartesian_product(0..(self.width as u32), 0..(self.height as u32))
            .map(|(x, y)| self.read(x, y))
            .collect()
    }
}

impl Layer<Rgba<u8>> for NoiseLayer {
    fn shape(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn read(&self, x: u32, y: u32) -> Rgba<u8> {
        let pixel = self.at_norm(x, y).lerp(u8::MIN as f64, u8::MAX as f64) as u8;
        Rgba([pixel, pixel, pixel, 1])
    }

    fn pixels(&self) -> Vec<Rgba<u8>> {
        Itertools::cartesian_product(0..(self.width as u32), 0..(self.height as u32))
            .map(|(x, y)| self.read(x, y))
            .collect()
    }
}
