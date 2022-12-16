use ndarray::Array2;
use num::{Complex, Zero};

use super::{CghConfig, CghProcessor};

use crate::lut::TriFunc;
use crate::object::Object3d;

pub struct CghProcessorRecurrence;

impl CghProcessor for CghProcessorRecurrence {
    fn process(
        &self,
        tri_func: &dyn TriFunc,
        config: &CghConfig,
        object: &Object3d,
    ) -> Array2<f64> {
        let mut array =
            Array2::<f64>::zeros((config.image_height as usize, config.image_width as usize));

        for (y, mut row) in array.outer_iter_mut().enumerate() {
            let mut theta_prev = vec![0.0; object.points().len()];
            let mut delta_prev = vec![0.0; object.points().len()];
            let mut zeta = vec![0.0; object.points().len()];

            let mut complex0 = Complex::<f64>::zero();
            for (j, &point) in object.points().iter().enumerate() {
                let point = (point * config.scalar) + config.offset;
                theta_prev[j] = (config.pixel_pitch / config.wavelength)
                    * (point.z
                        + (point.x.powi(2) + (y as f64 - point.y).powi(2)) / (2.0 * point.z));
                delta_prev[j] = (config.pixel_pitch / (2.0 * config.wavelength * point.z))
                    * (2.0 * -point.x + 1.0);
                zeta[j] = config.pixel_pitch / (config.wavelength * point.z);
                complex0 += Complex::new(tri_func.cos(theta_prev[j]), tri_func.sin(theta_prev[j]));
            }

            let mut iter = row.iter_mut();

            *iter.next().unwrap() = f64::atan2(complex0.im, complex0.re);

            for pixel in iter {
                let mut complex = Complex::<f64>::zero();

                for (j, _) in object.points().iter().enumerate() {
                    let theta = theta_prev[j] + delta_prev[j];
                    let delta = delta_prev[j] + zeta[j];

                    complex += Complex::new(tri_func.cos(theta), tri_func.sin(theta));

                    theta_prev[j] = theta;
                    delta_prev[j] = delta;
                }

                *pixel = f64::atan2(complex.im, complex.re);
            }
        }

        array
    }
}
