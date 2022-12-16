pub mod naive;

use ndarray::Array2;

use crate::bmp::Image;
use crate::lut::TriFunc;
use crate::object::Object3d;
use crate::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct CghConfig {
    pub image_width: u32,
    pub image_height: u32,
    pub pixel_pitch: f64,
    pub wavelength: f64,
    pub offset: Point,
    pub scalar: f64,
}

pub trait CghProcessor {
    fn process(&self, tri_func: &dyn TriFunc, config: &CghConfig, object: &Object3d)
        -> Array2<f64>;
}

pub fn generate_cgh(
    processor: &dyn CghProcessor,
    tri_func: &dyn TriFunc,
    config: &CghConfig,
    object: &Object3d,
) -> Image {
    let array = processor.process(tri_func, config, object);

    let (min, max) = array.iter().fold((f64::MAX, f64::MIN), |(min, max), &v| {
        (min.min(v), max.max(v))
    });

    let mut image = Image::new(config.image_width as u32, config.image_height as u32);

    for ((y, x), &pixel) in array.indexed_iter() {
        let v = (255.0 * (pixel - min) / (max - min)) as u8;
        image.set_pixel(x as u32, y as u32, v);
    }

    image
}
