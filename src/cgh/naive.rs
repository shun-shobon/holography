use ndarray::Array2;
use num::{Complex, Zero};

use super::{CghConfig, CghProcessor};

use crate::bmp::Image;
use crate::lut::TriFunc;
use crate::object::Object3d;
use crate::point::Point;

pub struct CghProcessorNaive<'a> {
    tri_func: &'a dyn TriFunc,
    config: &'a CghConfig,
}

impl<'a> CghProcessorNaive<'a> {
    pub fn new(tri_func: &'a dyn TriFunc, config: &'a CghConfig) -> Self {
        Self { tri_func, config }
    }
}

impl<'a> CghProcessor for CghProcessorNaive<'a> {
    fn process(&self, object: &Object3d) -> Image {
        let mut array = Array2::<f64>::zeros((
            self.config.image_height as usize,
            self.config.image_width as usize,
        ));

        for ((y, x), pixel) in array.indexed_iter_mut() {
            let mut complex = Complex::zero();

            for &point in object.points() {
                let point = (point * self.config.scalar) + self.config.offset;
                let pixel_point = Point::new(x as f64, y as f64, 0.0);
                let distance = point.distance(&pixel_point);

                let theta = self.config.pixel_pitch * distance / self.config.wavelength;
                complex += Complex::new(
                    self.tri_func.cos(theta) / distance,
                    self.tri_func.sin(theta) / distance,
                );
            }

            *pixel = f64::atan2(complex.im, complex.re);
        }

        let (min, max) = array.iter().fold((f64::MAX, f64::MIN), |(min, max), &v| {
            (min.min(v), max.max(v))
        });

        let mut image = Image::new(self.config.image_width, self.config.image_height);

        for ((y, x), &pixel) in array.indexed_iter() {
            let v = (255.0 * (pixel - min) / (max - min)) as u8;
            image.set_pixel(x as u32, y as u32, v);
        }

        image
    }
}
