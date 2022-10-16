#![allow(dead_code)]

mod image;
mod noise;
mod utils;
use self::image::{Filter, GrayscaleStepFilter, NoiseLayer};
use ::image::RgbImage;
use noise::simple;

use crate::noise::Noise2D;

fn main() {
    let noise2d = Noise2D::Simple {
        parameters: simple::Parameters {
            seed: 123456789,
            layers: 4,
            persistence: 0.5,
            frequency: 1. / 128.,
            roughness: 1.75,
            strength: 1.,
            floor: 0.9,
            // weight: 1.,
        },
    };

    let noise = &mut noise2d.noise();

    let layer = NoiseLayer::new(1024, 1024, noise);

    let filter = GrayscaleStepFilter(10);
    let filtered = filter.apply(&layer);

    let image = RgbImage::from_fn(1024, 1024, |x, y| filtered.read(x, y));
    match image.save("perlin.png") {
        _ => {}
    }
}
