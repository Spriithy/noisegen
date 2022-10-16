use super::{BaseNoise, Noise};

pub struct Parameters {
    pub seed: u64,
    pub layers: usize,
    pub persistence: f64,
    pub frequency: f64,
    pub roughness: f64,
    pub strength: f64,
    pub floor: f64,
    pub weight: f64,
}

pub struct RigidNoise {
    parameters: Parameters,
    noise: BaseNoise,
}

impl RigidNoise {
    pub fn new(parameters: Parameters) -> Self {
        let seed = parameters.seed;
        Self {
            parameters,
            noise: BaseNoise::new(seed),
        }
    }
}

impl Noise for RigidNoise {
    fn eval(&mut self, x: f64, y: f64) -> f64 {
        let mut value = 0.;
        let mut frequency = self.parameters.frequency;
        let mut amplitude = 1.;
        let mut weight = 1.;

        for _ in 0..self.parameters.layers {
            let mut noise = 1. - (self.noise.perlin(x * frequency, y * frequency, 0.)).abs();
            noise *= noise;
            noise *= weight;
            weight = (noise * self.parameters.weight).clamp(0., 1.);

            value += noise * amplitude;
            frequency *= self.parameters.roughness;
            amplitude *= self.parameters.persistence;
        }

        (value - self.parameters.floor) * self.parameters.strength
    }
}
