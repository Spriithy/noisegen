use super::{rigid, simple, Noise};

pub enum Noise2D {
    Simple { parameters: simple::Parameters },
    Rigid { parameters: rigid::Parameters },
}

impl Noise2D {
    /*
    pub fn new(self, width: usize, height: usize) -> Result<NoiseMap, ()> {
        let mut noise = self.noise();
        let pixels = noise.chunk(width, height);

        let (min, max) = pixels.min_max().unwrap();
        println!("min={} max={}", min, max);

        let normalized_pixels = pixels.iter().map(|x| x.clamp(min, max));
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
    */

    pub fn noise(self) -> Box<dyn Noise> {
        use Noise2D::*;

        match self {
            Simple { parameters } => Box::new(simple::SimpleNoise::new(parameters)),
            Rigid { parameters } => Box::new(rigid::RigidNoise::new(parameters)),
        }
    }
}
