pub trait Lerp<T> {
    fn lerp(self, a: T, b: T) -> T;
}
pub trait LerpInv<T> {
    fn lerp_inv(self, a: T, b: T) -> f64;
}

pub trait Remap<T>: Lerp<T> + LerpInv<T> {
    fn remap(self, from_min: T, from_max: T, to_min: T, to_max: T) -> T;
}

impl Lerp<f64> for f64 {
    fn lerp(self, a: f64, b: f64) -> f64 {
        a + self * (b - a)
    }
}

impl LerpInv<f64> for f64 {
    fn lerp_inv(self, a: f64, b: f64) -> f64 {
        (self - a) / (b - a)
    }
}

impl Remap<f64> for f64 {
    fn remap(self, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64 {
        let t = self.lerp_inv(from_min, from_max);
        t.lerp(to_min, to_max)
    }
}

impl Lerp<u8> for u8 {
    fn lerp(self, a: u8, b: u8) -> u8 {
        a + self * (b - a)
    }
}

impl LerpInv<u8> for u8 {
    fn lerp_inv(self, a: u8, b: u8) -> f64 {
        (self as f64 - a as f64) / (b as f64 - a as f64)
    }
}

impl Remap<u8> for u8 {
    fn remap(self, from_min: u8, from_max: u8, to_min: u8, to_max: u8) -> u8 {
        let t = self.lerp_inv(from_min, from_max);
        to_min + (t * to_max as f64) as u8
    }
}
