use holography::bmp::Image;
use holography::point::Point;
use ndarray::Array2;

const IMAGE_WIDTH: u32 = 1920;
const IMAGE_HEIGHT: u32 = 1080;
const PIXEL_PITCH: f64 = 8.0e-6; // 8μm
const WAVELENGTH: f64 = 520.0e-9; // 520nm

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let k = 2.0 * std::f64::consts::PI / WAVELENGTH;

    let point_light = Point::new(
        IMAGE_WIDTH as f64 / 2.0,
        IMAGE_HEIGHT as f64 / 2.0,
        1.0 / PIXEL_PITCH,
    );

    let mut array = Array2::<f64>::zeros((IMAGE_HEIGHT as usize, IMAGE_WIDTH as usize));

    for ((y, x), pixel) in array.indexed_iter_mut() {
        let point = Point::new(x as f64, y as f64, 0.0);
        let distance = point_light.distance(&point);
        *pixel = 1.0 / distance * f64::cos(k * PIXEL_PITCH * distance);
    }

    let (min, max) = array.iter().fold((f64::MAX, f64::MIN), |(min, max), &v| {
        (min.min(v), max.max(v))
    });
    let mid = (max + min) / 2.0;

    let mut image = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for ((y, x), pixel) in array.indexed_iter() {
        let v = if *pixel > mid { 255 } else { 0 };
        image.set_pixel(x as u32, y as u32, v);
    }

    image.save("out/point-right-cgh.bmp")?;

    Ok(())
}
