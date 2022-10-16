use image::Pixel;

pub trait Layer<T>
where
    T: Pixel,
{
    fn shape(&self) -> (usize, usize);

    fn read(&self, x: u32, y: u32) -> T;

    fn pixels(&self) -> Vec<T>;
}

pub struct PixelLayer<T>
where
    T: Pixel,
{
    width: usize,
    height: usize,
    pixels: Vec<T>,
}

impl<T> Layer<T> for PixelLayer<T>
where
    T: Pixel,
{
    fn shape(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn read(&self, x: u32, y: u32) -> T {
        self.pixels[(x as usize) * self.width + (y as usize)]
    }

    fn pixels(&self) -> Vec<T> {
        self.pixels.clone()
    }
}

impl<T> PixelLayer<T>
where
    T: Pixel,
{
    pub fn new(width: usize, height: usize, pixels: &Vec<T>) -> Self {
        Self {
            width,
            height,
            pixels: pixels.clone(),
        }
    }
}
