use crate::noise;
use crate::noise::NoiseMap;

use super::perlin;

pub enum Noise2D {
    Random,
    Perlin { parameters: perlin::Parameters },
}

impl Noise2D {
    pub fn new(self, width: usize, height: usize) -> Result<NoiseMap, ()> {
        match self {
            Self::Random => Ok(noise::random::gen(width, height)),
            Self::Perlin { parameters } => {
                noise::perlin::PerlinNoiseGen::new(parameters).gen(width, height)
            }
        }
    }
}
