use crate::point::Point;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Debug)]
pub struct Object3d {
    points: Vec<Point>,
}

impl Object3d {
    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }

    pub fn size(&self) -> usize {
        self.points.len()
    }

    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut buffer = [0; 4];

        reader.read_exact(&mut buffer)?;
        let size = u32::from_le_bytes(buffer);

        let mut points = Vec::with_capacity(size as usize);

        for _ in 0..size {
            reader.read_exact(&mut buffer)?;
            let x = i32::from_le_bytes(buffer);
            reader.read_exact(&mut buffer)?;
            let y = i32::from_le_bytes(buffer);
            reader.read_exact(&mut buffer)?;
            let z = i32::from_le_bytes(buffer);

            let point = Point::new(x as f64, y as f64, z as f64);
            points.push(point);
        }

        Ok(Object3d { points })
    }

    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(path)?;
        Self::from_reader(&mut file)
    }
}
