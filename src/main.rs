#![allow(dead_code)]

mod noise;
use noise::{perlin, rigid};

mod utils;

use crate::noise::Noise2D;

fn main() {
    let noise = Noise2D::Perlin {
        parameters: perlin::Parameters {
            seed: 123456789,
            layers: 4,
            persistence: 0.75,
            frequency: 1. / 512.,
            roughness: 1.5,
            strength: 1.5,
            floor: 0.,
            // weight: 1.,
        },
    };

    if let Ok(map) = noise.new(1024, 1024) {
        map.save("perlin.png");
    }
}
