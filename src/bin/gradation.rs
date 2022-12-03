use holography::bmp;
use std::io;

fn main() -> io::Result<()> {
    let mut image = bmp::Image::new(256, 256);

    let distance_max = ((image.height().pow(2) + image.width().pow(2)) as f64).sqrt();

    for (x, y) in image.coordinates() {
        let distance = ((y.pow(2) + x.pow(2)) as f64).sqrt();
        let color = (distance / distance_max * 255.0) as u8;
        image.set_pixel(x, y, color);
    }

    image.save("out/gradation.bmp")?;

    Ok(())
}
