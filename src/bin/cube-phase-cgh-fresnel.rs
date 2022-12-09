use holography::bmp::Image;
use holography::object::Object3d;
use holography::point::Point;
use ndarray::Array2;
use num::{Complex, Zero};

const IMAGE_WIDTH: u32 = 1920;
const IMAGE_HEIGHT: u32 = 1080;
const PIXEL_PITCH: f64 = 8.0e-6; // 8μm
const WAVELENGTH: f64 = 520.0e-9; // 520nm
const K: f64 = 2.0 * std::f64::consts::PI / WAVELENGTH;
const OFFSET: Point = Point::new(
    IMAGE_WIDTH as f64 / 2.0,
    IMAGE_HEIGHT as f64 / 2.0,
    1.0 / PIXEL_PITCH,
);
const SCALAR: f64 = 40.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let object = Object3d::open("data/cube284.3d")?;

    let mut array = Array2::<f64>::zeros((IMAGE_HEIGHT as usize, IMAGE_WIDTH as usize));

    for ((y, x), pixel) in array.indexed_iter_mut() {
        let mut complex = Complex::zero();

        for &point in object.points() {
            let point = (point * SCALAR) + OFFSET;
            let pixel_point = Point::new(x as f64, y as f64, 0.0);
            let distance = point.z
                + ((pixel_point.x - point.x).powi(2) + (pixel_point.y - point.y).powi(2))
                    / (2.0 * point.z);

            complex += Complex::from_polar(1.0, K * PIXEL_PITCH * distance);
        }

        *pixel = f64::atan2(complex.im, complex.re);
    }

    let (min, max) = array.iter().fold((f64::MAX, f64::MIN), |(min, max), &v| {
        (min.min(v), max.max(v))
    });

    let mut image = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for ((y, x), &pixel) in array.indexed_iter() {
        let v = (255.0 * (pixel - min) / (max - min)) as u8;
        image.set_pixel(x as u32, y as u32, v);
    }

    image.save("out/cube-phase-cgh-fresnel.bmp")?;

    Ok(())
}
