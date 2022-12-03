use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug)]
struct BmpFileHeader {
    file_type: [u8; 2],
    file_size: u32,
    reserved1: u16,
    reserved2: u16,
    data_offset: u32,
}

impl BmpFileHeader {
    const HEADER_SIZE: u32 = 14;

    fn new(header_size: u32, data_size: u32) -> Self {
        BmpFileHeader {
            file_type: *b"BM",
            file_size: header_size + data_size,
            reserved1: 0, // unused
            reserved2: 0, // unused
            data_offset: header_size,
        }
    }

    fn to_writer<W: Write>(&self, destination: &mut W) -> io::Result<()> {
        destination.write_all(&self.file_type)?;
        destination.write_all(&self.file_size.to_le_bytes())?;
        destination.write_all(&self.reserved1.to_le_bytes())?;
        destination.write_all(&self.reserved2.to_le_bytes())?;
        destination.write_all(&self.data_offset.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug)]
struct BmpInfoHeader {
    header_size: u32,
    width: i32,
    height: i32,
    planes: u16,
    bits_per_pixel: u16,
    compression: u32,
    image_size: u32,
    x_pixels_per_meter: i32,
    y_pixels_per_meter: i32,
    colors_used: u32,
    colors_important: u32,
}

impl BmpInfoHeader {
    const HEADER_SIZE: u32 = 40;

    fn new(width: i32, height: i32) -> Self {
        BmpInfoHeader {
            header_size: Self::HEADER_SIZE, // fixed
            width,
            height,
            planes: 1,             // fixed
            bits_per_pixel: 8,     // fixed
            compression: 0,        // fixed
            image_size: 0,         // fixed
            x_pixels_per_meter: 0, // fixed
            y_pixels_per_meter: 0, // fixed
            colors_used: 0,        // fixed
            colors_important: 0,   // fixed
        }
    }

    fn to_writer<W: Write>(&self, distination: &mut W) -> io::Result<()> {
        distination.write_all(&self.header_size.to_le_bytes())?;
        distination.write_all(&self.width.to_le_bytes())?;
        distination.write_all(&self.height.to_le_bytes())?;
        distination.write_all(&self.planes.to_le_bytes())?;
        distination.write_all(&self.bits_per_pixel.to_le_bytes())?;
        distination.write_all(&self.compression.to_le_bytes())?;
        distination.write_all(&self.image_size.to_le_bytes())?;
        distination.write_all(&self.x_pixels_per_meter.to_le_bytes())?;
        distination.write_all(&self.y_pixels_per_meter.to_le_bytes())?;
        distination.write_all(&self.colors_used.to_le_bytes())?;
        distination.write_all(&self.colors_important.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug)]
struct BmpColorPallet {
    r: u8,
    g: u8,
    b: u8,
    reserved: u8, // unused
}

impl BmpColorPallet {
    fn new(r: u8, g: u8, b: u8) -> Self {
        BmpColorPallet {
            r,
            g,
            b,
            reserved: 0,
        }
    }

    fn to_writer<W: Write>(&self, distination: &mut W) -> io::Result<()> {
        distination.write_all(&self.r.to_le_bytes())?;
        distination.write_all(&self.g.to_le_bytes())?;
        distination.write_all(&self.b.to_le_bytes())?;
        distination.write_all(&self.reserved.to_le_bytes())?;

        Ok(())
    }
}

const PALLATE_SIZE: usize = 256;

#[derive(Debug)]
pub struct Image {
    header: BmpFileHeader,
    info: BmpInfoHeader,
    pallet: [BmpColorPallet; PALLATE_SIZE],
    data: Vec<u8>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        let pallet = (0u8..=255)
            .map(|i| BmpColorPallet::new(i, i, i))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let data = vec![0; (width * height) as usize];
        let header = BmpFileHeader::new(
            BmpFileHeader::HEADER_SIZE + BmpInfoHeader::HEADER_SIZE + PALLATE_SIZE as u32 * 4,
            data.len() as u32,
        );
        let info = BmpInfoHeader::new(width as i32, height as i32);

        Image {
            header,
            info,
            pallet,
            data,
        }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.info.width as u32
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.info.height as u32
    }

    #[inline]
    pub fn set_pixel(&mut self, x: u32, y: u32, color: u8) {
        let index = (y * self.width() + x) as usize;
        self.data[index] = color;
    }

    #[inline]
    pub fn get_pixel(&self, x: u32, y: u32) -> u8 {
        let index = (y * self.width() + x) as usize;
        self.data[index]
    }

    #[inline]
    pub fn coordinates(&self) -> impl Iterator<Item = (u32, u32)> {
        ImageIndex::new(self.width(), self.height())
    }

    pub fn to_writer<W: Write>(&self, destination: &mut W) -> io::Result<()> {
        self.header.to_writer(destination)?;
        self.info.to_writer(destination)?;
        self.pallet
            .iter()
            .try_for_each(|pallet| pallet.to_writer(destination))?;
        destination.write_all(&self.data)?;

        Ok(())
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = File::create(path)?;
        self.to_writer(&mut file)
    }
}

#[derive(Debug, Clone, Copy)]
struct ImageIndex {
    width: u32,
    height: u32,
    x: u32,
    y: u32,
}

impl ImageIndex {
    fn new(width: u32, height: u32) -> Self {
        ImageIndex {
            width,
            height,
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for ImageIndex {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.height {
            return None;
        }

        let x = self.x;
        let y = self.y;

        self.x += 1;
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }

        Some((x, y))
    }
}
