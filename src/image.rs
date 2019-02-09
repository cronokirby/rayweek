use std::io;


#[inline]
fn clip(f: f32) -> f32 {
    if f > 1.0 {
        1.0
    } else if f < 0.0 {
        0.0
    } else {
        f
    }
}

#[inline]
fn comp_byte(f: f32) -> u8 {
    (f * 255.0) as u8
}


/// Represents an RGBA color
#[derive(Clone, Copy, Debug)]
pub struct RGBA(pub f32, pub f32, pub f32, pub f32);

impl RGBA {
    pub fn clip(&self) -> Self {
        let RGBA(r, g, b, a) = *self;
        RGBA(clip(r), clip(g), clip(b), clip(a))
    }
}


/// Represents an RGBA image
pub struct Image {
    pixels: Vec<RGBA>,
    /// The width of the image in pixels
    pub width: usize,
    /// The height of the image in pixels
    pub height: usize
}

impl Image {
    /// Create an image of certain dimensions of a solid color
    pub fn solid_color(width: usize, height: usize, color: RGBA) -> Self {
        let pixels = vec![color; width * height];
        Image { pixels, width, height }
    }

    /// Get the color of a certain pixel
    pub fn get_pixel(&self, x: usize, y: usize) -> RGBA {
        self.pixels[self.width * y + x]
    }

    /// Write this image in bmp format to some sink
    pub fn write_bmp(&self, mut sink: impl io::Write) -> io::Result<()> {
        let w = self.width;
        let h = self.height;
        let byte_size = w * h * 4;

        let file_size = byte_size + 122;
        let file_header = [
            0x42, 0x4D,
            file_size as u8, (file_size >> 8) as u8,
            (file_size >> 16) as u8, (file_size >> 24) as u8,
            0,    0, 0, 0,
            0x7A, 0, 0, 0
        ];
        sink.write_all(&file_header)?;

        let dib_header = [
            0x6C, 0, 0, 0,
            w as u8, (w >> 8) as u8,
            (w >> 16) as u8, (w >> 24) as u8,
            h as u8, (h >> 8) as u8,
            (h >> 16) as u8, (h >> 24) as u8,
            1,    0,
            32,   0,
            3,    0,    0,    0,
            32,   0,    0,    0,
            0,    0,    0,    0,
            0,    0,    0,    0,
            0,    0,    0,    0,
            0,    0,    0,    0,
            0,    0,    0xFF, 0,
            0,    0xFF, 0,    0,
            0xFF, 0,    0,    0,
            0,    0,    0,    0xFF,
            0x20, 0x6E, 0x69, 0x57,
            0,    0,    0,    0,
            0,    0,    0,    0,
            0,    0,    0,    0,
            0,    0,    0,    0,
            0,    0,    0,    0,
            0,    0,    0,    0,
            0,    0,    0,    0,
            0,    0,    0,    0,
            0,    0,    0,    0,
        ];
        sink.write_all(&dib_header)?;

        for y in (0..h).rev() {
            for x in 0..w {
                let RGBA(r, g, b, a) = self.get_pixel(x, y).clip();
                sink.write_all(&[comp_byte(b), comp_byte(g), comp_byte(r), comp_byte(a)])?;
            }
        }
        Ok(())
    }
}
