use ndarray::Array2;
use num::{Complex, Zero};

use super::{CghConfig, CghProcessor};

use crate::lut::TriFunc;
use crate::object::Object3d;
use crate::point::Point;

pub struct CghProcessorNaive;

impl CghProcessor for CghProcessorNaive {
    fn process(
        &self,
        tri_func: &dyn TriFunc,
        config: &CghConfig,
        object: &Object3d,
    ) -> Array2<f64> {
        let mut array =
            Array2::<f64>::zeros((config.image_height as usize, config.image_width as usize));

        for ((y, x), pixel) in array.indexed_iter_mut() {
            let mut complex = Complex::zero();

            for &point in object.points() {
                let point = (point * config.scalar) + config.offset;
                let pixel_point = Point::new(x as f64, y as f64, 0.0);
                let distance = point.distance(&pixel_point);

                let theta = config.pixel_pitch * distance / config.wavelength;
                complex += Complex::new(
                    tri_func.cos(theta) / distance,
                    tri_func.sin(theta) / distance,
                );
            }

            *pixel = f64::atan2(complex.im, complex.re);
        }

        array
    }
}
