#![allow(dead_code)]

mod noise;
use noise::perlin::Parameters;

use crate::noise::Noise2D;

fn main() {
    //Noise2D::Random.new(256, 256).save("random.png");

    let noise = Noise2D::Perlin {
        parameters: Parameters::new(4, 0.35, 1. / 256.),
    };

    if let Ok(map) = noise.new(2000, 2000) {
        map.save("perlin.png");
    }
}
