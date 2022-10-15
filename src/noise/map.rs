use std::ops;

pub struct NoiseMap {
    shape: [usize; 2],
    pixels: Vec<u8>,
}

impl NoiseMap {
    pub fn new(width: usize, height: usize) -> Self {
        NoiseMap {
            shape: [width, height],
            pixels: vec![0; width * height],
        }
    }

    pub fn from_pixels(width: usize, height: usize, pixels: Vec<f64>) -> Self {
        let (mut min, mut max) = (0., 0.);

        for &x in pixels.iter() {
            if x > max {
                max = x;
            }
            if x < min {
                min = x;
            }
        }

        let normalized_pixels = pixels.iter().map(|x| (x - min) / (max - min));
        let l8_pixels = normalized_pixels.map(|x| (x * 255.) as u8);

        NoiseMap {
            shape: [width, height],
            pixels: l8_pixels.collect(),
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.shape[0]
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.shape[1]
    }

    pub fn save(&self, path: &str) {
        image::save_buffer(
            path,
            &self.pixels,
            self.width().try_into().unwrap(),
            self.height().try_into().unwrap(),
            image::ColorType::L8,
        )
        .unwrap();
    }

    pub fn print(&self) {
        for x in 0..self.width() {
            for y in 0..self.height() {
                print!("{:>3} ", self[x][y])
            }
            println!()
        }
        println!("NoiseMap: {}x{}", self.width(), self.height())
    }
}

impl ops::Index<usize> for NoiseMap {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        let base = index * self.shape[1];
        &self.pixels[base..][..self.shape[1]]
    }
}

impl ops::IndexMut<usize> for NoiseMap {
    fn index_mut(&mut self, index: usize) -> &mut [u8] {
        let base = index * self.shape[1];
        &mut self.pixels[base..][..self.shape[1]]
    }
}
