use rand::Rng;

use super::Noise;

pub struct RandomNoise {
    rng: rand::rngs::ThreadRng,
}

impl RandomNoise {
    pub fn new() -> Self {
        RandomNoise {
            rng: rand::thread_rng(),
        }
    }
}

impl Noise for RandomNoise {
    fn eval(&mut self, _: f64, _: f64) -> f64 {
        self.rng.gen_range((0.)..1.)
    }
}
