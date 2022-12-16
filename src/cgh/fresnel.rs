use ndarray::Array2;
use ndarray::Zip;
use num::{Complex, Zero};

use super::{CghConfig, CghProcessor};

use crate::lut::TriFunc;
use crate::object::Object3d;
use crate::point::Point;

macro_rules! process {
    ($x:expr, $y:expr, $pixel:expr, $tri_func:expr, $config:expr, $object:expr) => {{
        let mut complex = Complex::zero();

        for &point in $object.points() {
            let point = (point * $config.scalar) + $config.offset;
            let pixel_point = Point::new($x as f64, $y as f64, 0.0);
            let distance = point.z
                + ((pixel_point.x - point.x).powi(2) + (pixel_point.y - point.y).powi(2))
                    / (2.0 * point.z);

            let theta = $config.pixel_pitch * distance / $config.wavelength;
            complex += Complex::new($tri_func.cos(theta), $tri_func.sin(theta));
        }

        *$pixel = f64::atan2(complex.im, complex.re);
    }};
}

pub struct CghProcessorFresnel;

impl CghProcessor for CghProcessorFresnel {
    fn process(
        &self,
        tri_func: &(dyn TriFunc + Sync),
        config: &CghConfig,
        object: &Object3d,
    ) -> Array2<f64> {
        let mut array =
            Array2::<f64>::zeros((config.image_height as usize, config.image_width as usize));

        array
            .indexed_iter_mut()
            .for_each(|((y, x), pixel)| process!(x, y, pixel, tri_func, config, object));

        array
    }
}

pub struct CghProcessorFresnelParallel;

impl CghProcessor for CghProcessorFresnelParallel {
    fn process(
        &self,
        tri_func: &(dyn TriFunc + Sync),
        config: &CghConfig,
        object: &Object3d,
    ) -> Array2<f64> {
        let mut array =
            Array2::<f64>::zeros((config.image_height as usize, config.image_width as usize));

        Zip::indexed(&mut array)
            .par_for_each(|(y, x), pixel| process!(x, y, pixel, tri_func, config, object));

        array
    }
}
