use crate::utils::maths::Remap;
use image::{Luma, Pixel, Rgb};

use super::{Layer, PixelLayer};
pub trait Filter<V, R>
where
    V: Pixel,
    R: Pixel,
{
    fn apply(self, layer: &dyn Layer<V>) -> Box<dyn Layer<R>>;
}

pub struct GrayscaleStepFilter(pub u8);

impl Filter<Luma<u8>, Luma<u8>> for GrayscaleStepFilter {
    fn apply(self, layer: &dyn Layer<Luma<u8>>) -> Box<dyn Layer<Luma<u8>>> {
        let steps: Vec<u8> = (0..self.0).map(|x| x.remap(0, self.0, 0, 255)).collect();

        let func = |x| {
            for window in steps.windows(2) {
                if x < window[1] {
                    return Luma([window[0]]);
                }
            }

            Luma([255])
        };

        let pixels = layer
            .pixels()
            .iter()
            .map(|Luma([p])| *p)
            .map(func)
            .collect();

        let (width, height) = layer.shape();

        Box::new(PixelLayer::new(width, height, &pixels))
    }
}

impl Filter<Luma<u8>, Rgb<u8>> for GrayscaleStepFilter {
    fn apply(self, layer: &dyn Layer<Luma<u8>>) -> Box<dyn Layer<Rgb<u8>>> {
        let steps: Vec<u8> = (0..self.0).map(|x| x.remap(0, self.0, 0, 255)).collect();

        let func = |x| {
            for window in steps.windows(2) {
                if x < window[1] {
                    return Rgb([window[0]; 3]);
                }
            }

            Rgb([255; 3])
        };

        let pixels = layer
            .pixels()
            .iter()
            .map(|Luma([p])| *p)
            .map(func)
            .collect();

        let (width, height) = layer.shape();

        Box::new(PixelLayer::new(width, height, &pixels))
    }
}
