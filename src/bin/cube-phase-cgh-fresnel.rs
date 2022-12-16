use holography::cgh::naive::CghProcessorNaive;
use holography::cgh::{generate_cgh, CghConfig};
use holography::lut::TriFuncNaive;
use holography::object::Object3d;
use holography::point::Point;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let object = Object3d::open("data/cube284.3d")?;

    let config = CghConfig {
        image_width: IMAGE_WIDTH,
        image_height: IMAGE_HEIGHT,
        pixel_pitch: PIXEL_PITCH,
        wavelength: WAVELENGTH,
        scalar: SCALAR,
        offset: OFFSET,
    };

    let tri_func = TriFuncNaive::default();
    let processor = CghProcessorNaive;

    generate_cgh(&processor, &tri_func, &config, &object).save("out/cube-phase-cgh-fresnel.bmp")?;

    Ok(())
}
