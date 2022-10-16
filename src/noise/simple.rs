use super::{BaseNoise, Noise};

#[derive(Clone)]
pub struct Parameters {
    pub seed: u64,
    pub layers: usize,
    pub persistence: f64,
    pub frequency: f64,
    pub roughness: f64,
    pub strength: f64,
    pub floor: f64,
}

pub struct SimpleNoise {
    parameters: Parameters,
    noise: BaseNoise,
}

impl SimpleNoise {
    pub fn new(parameters: Parameters) -> Self {
        let seed = parameters.seed;
        Self {
            parameters,
            noise: BaseNoise::new(seed),
        }
    }
}

impl Noise for SimpleNoise {
    fn eval(&mut self, x: f64, y: f64) -> f64 {
        let mut value = 0.;
        let mut frequency = self.parameters.frequency;
        let mut amplitude = 1.;

        for _ in 0..self.parameters.layers {
            let noise = self.noise.perlin(x * frequency, y * frequency, 0.);
            value += (1. + noise) * 0.5 * amplitude;

            amplitude *= self.parameters.persistence;
            frequency *= self.parameters.roughness;
        }

        (value - self.parameters.floor) * self.parameters.strength
    }
}
