pub mod naive;

use crate::bmp::Image;
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
    fn process(&self, object: &Object3d) -> Image;
}
