use crate::{
    noise::NoiseMap,
    utils::{MinMax, MinMaxStore},
};

use super::{perlin, random, rigid, Noise};

pub enum Noise2D {
    Random,
    Perlin { parameters: perlin::Parameters },
    RigidPerlin { parameters: rigid::Parameters },
}

impl Noise2D {
    pub fn new(self, width: usize, height: usize) -> Result<NoiseMap, ()> {
        let mut noise: Box<dyn Noise> = match self {
            Self::Random => Box::new(random::RandomNoise::new()),
            Self::Perlin { parameters } => Box::new(perlin::PerlinNoise::new(parameters)),
            Self::RigidPerlin { parameters } => Box::new(rigid::RigidPerlinNoise::new(parameters)),
        };

        let pixels = noise.chunk(width, height);

        let minmax = MinMaxStore::from_vec(&pixels);
        match minmax.min_max() {
            (Some(min), Some(max)) => {
                println!("min={} max={}", min, max);

                let normalized_pixels = pixels.iter().map(|x| (x - min) / (max - min));
                let gradient = colorgrad::turbo().sharp(10, 0.25);
                let rgba_pixels = normalized_pixels
                    .map(|x| gradient.at(x).to_rgba8())
                    .flat_map(|x| x)
                    .collect();

                Ok(NoiseMap {
                    shape: [width, height],
                    pixels: rgba_pixels,
                })
            }
            _ => Err(()),
        }
    }
}
