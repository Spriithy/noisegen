use rand::{seq::SliceRandom, SeedableRng};

use super::NoiseMap;

#[derive(Clone)]
pub struct Parameters {
    seed: u64,
    octaves: usize,
    persistence: f64,
    frequency: f64,
}

impl Parameters {
    pub fn new(octaves: usize, persistence: f64, frequency: f64) -> Self {
        Self::new_with_seed(rand::random::<u64>(), octaves, persistence, frequency)
    }

    pub fn new_with_seed(seed: u64, octaves: usize, persistence: f64, frequency: f64) -> Self {
        Parameters {
            seed,
            octaves,
            persistence,
            frequency,
        }
    }
}

pub struct PerlinNoiseGen {
    parameters: Parameters,
    permutations: Vec<usize>,
}

impl PerlinNoiseGen {
    pub fn new(parameters: Parameters) -> Self {
        let permutations = permutations_from_seed(parameters.seed);
        PerlinNoiseGen {
            parameters,
            permutations,
        }
    }

    fn hash(&self, x: usize, y: usize, z: usize) -> usize {
        self.permutations[self.permutations[self.permutations[x] + y] + z]
    }

    fn perlin(&self, x: f64, y: f64, z: f64) -> f64 {
        let (xi0, yi0, zi0) = (x as usize & 255, y as usize & 255, z as usize & 255);
        let (xi1, yi1, zi1) = ((xi0 + 1) & 255, (yi0 + 1) & 255, (zi0 + 1) & 255);
        let (tx, ty, tz) = (x - x.floor(), y - y.floor(), z - z.floor());
        let (u, v, w) = (fade(tx), fade(ty), fade(tz));
        let (x0, y0, z0) = (tx, ty, tz);
        let (x1, y1, z1) = (tx - 1., ty - 1., tz - 1.);

        let a = grad(self.hash(xi0, yi0, zi0), x0, y0, z0);
        let b = grad(self.hash(xi1, yi0, zi0), x1, y0, z0);
        let c = grad(self.hash(xi0, yi1, zi0), x0, y1, z0);
        let d = grad(self.hash(xi1, yi1, zi0), x1, y1, z0);
        let e = grad(self.hash(xi0, yi0, zi1), x0, y0, z1);
        let f = grad(self.hash(xi1, yi0, zi1), x1, y0, z1);
        let g = grad(self.hash(xi0, yi1, zi1), x0, y1, z1);
        let h = grad(self.hash(xi1, yi1, zi1), x1, y1, z1);

        let k0 = a;
        let k1 = b - a;
        let k2 = c - a;
        let k3 = e - a;
        let k4 = a + d - b - c;
        let k5 = a + f - b - e;
        let k6 = a + g - c - e;
        let k7 = b + c + e + h - a - d - f - g;

        k0 + k1 * u + k2 * v + k3 * w + k4 * u * v + k5 * u * w + k6 * v * w + k7 * u * v * w
    }

    fn octave_perlin(&self, x: f64, y: f64, z: f64) -> f64 {
        let mut total = 0.;
        let mut frequency = self.parameters.frequency;
        let mut persistence = 1.;

        for _ in 0..self.parameters.octaves {
            total += self.perlin(x * frequency, y * frequency, z * frequency) * persistence;
            persistence *= self.parameters.persistence;
            frequency *= 2.;
        }

        total
    }

    pub fn gen(&self, width: usize, height: usize) -> Result<NoiseMap, ()> {
        let mut pixels = vec![0.; width * height];

        for x in 0..width {
            for y in 0..height {
                pixels[x * width + y] = self.octave_perlin(x as f64, y as f64, 0.)
            }
        }

        Ok(NoiseMap::from_pixels(width, height, pixels))
    }
}

#[inline]
fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6. - 15.) + 10.)
}

#[inline]
fn fade_deriv(t: f64) -> f64 {
    30. * t * t * (t * (t - 2.) + 1.)
}

#[inline]
fn lerp(a: f64, b: f64, x: f64) -> f64 {
    a + x * (b - a)
}

#[inline]
fn grad(hash: usize, x: f64, y: f64, z: f64) -> f64 {
    match hash & 0xF {
        0x0 => x + y,
        0x1 => -x + y,
        0x2 => x - y,
        0x3 => -x - y,
        0x4 => x + z,
        0x5 => -x + z,
        0x6 => x - z,
        0x7 => -x - z,
        0x8 => y + z,
        0x9 => -y + z,
        0xA => y - z,
        0xB => -y - z,
        0xC => y + x,
        0xD => -y + z,
        0xE => y - x,
        0xF => -y - z,
        _ => 0., // never happens
    }
}

fn permutations_from_seed(seed: u64) -> Vec<usize> {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    let mut perms: Vec<usize> = (0..512).map(|x| x & 255).collect();
    perms.shuffle(&mut rng);
    perms
}
