use colorgrad;

pub struct HeatmapFilter {
    width: usize,
    height: usize,
    pixels: Vec<f64>,
    gradient: colorgrad::Gradient,
}
