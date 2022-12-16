use holography::bmp::Image;
use holography::object::Object3d;
use holography::point::Point;
use ndarray::Array2;
use num::{Complex, Zero};
use std::f64::consts::PI;
use std::io;

const IMAGE_WIDTH: u32 = 1920;
const IMAGE_HEIGHT: u32 = 1080;
const PIXEL_PITCH: f64 = 8.0e-6; // 8μm
const WAVELENGTH: f64 = 520.0e-9; // 520nm
const OFFSET: Point = Point::new(
    IMAGE_WIDTH as f64 / 2.0,
    IMAGE_HEIGHT as f64 / 2.0,
    1.0 / PIXEL_PITCH,
);
const SCALAR: f64 = 40.0;

fn main() -> io::Result<()> {
    let object = Object3d::open("data/cube284.3d")?;

    let mut array = Array2::<f64>::zeros((IMAGE_HEIGHT as usize, IMAGE_WIDTH as usize));

    for (y, mut row) in array.outer_iter_mut().enumerate() {
        let mut theta_prev = vec![0.0; object.points().len()];
        let mut delta_prev = vec![0.0; object.points().len()];
        let mut zeta = vec![0.0; object.points().len()];

        let mut complex0 = Complex::<f64>::zero();
        for (j, &point) in object.points().iter().enumerate() {
            let point = (point * SCALAR) + OFFSET;
            theta_prev[j] = (PIXEL_PITCH / WAVELENGTH)
                * (point.z + (point.x.powi(2) + (y as f64 - point.y).powi(2)) / (2.0 * point.z));
            delta_prev[j] = (PIXEL_PITCH / (2.0 * WAVELENGTH * point.z)) * (2.0 * -point.x + 1.0);
            zeta[j] = PIXEL_PITCH / (WAVELENGTH * point.z);
            complex0 += Complex::from_polar(1.0, 2.0 * PI * theta_prev[j]);
        }

        let mut iter = row.iter_mut();

        *iter.next().unwrap() = f64::atan2(complex0.im, complex0.re);

        for pixel in iter {
            let mut complex = Complex::<f64>::zero();

            for (j, _) in object.points().iter().enumerate() {
                let theta = theta_prev[j] + delta_prev[j];
                let delta = delta_prev[j] + zeta[j];

                complex += Complex::from_polar(1.0, 2.0 * PI * theta);

                theta_prev[j] = theta;
                delta_prev[j] = delta;
            }

            *pixel = f64::atan2(complex.im, complex.re);
        }
    }

    let (min, max) = array.iter().fold((f64::MAX, f64::MIN), |(min, max), &v| {
        (min.min(v), max.max(v))
    });

    let mut image = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for ((y, x), &pixel) in array.indexed_iter() {
        let v = (255.0 * (pixel - min) / (max - min)) as u8;
        image.set_pixel(x as u32, y as u32, v);
    }

    image.save("out/cube-phase-cgh-recurrence-formula.bmp")?;

    Ok(())
}
