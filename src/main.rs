use std::io;
use std::fs::File;

mod image;
use image::*;


fn main() -> io::Result<()> {
    let file = File::create("image.bmp")?;
    let image = Image::solid_color(20, 20, RGBA(0.0, 0.5, 1.0, 1.0));
    image.write_bmp(file)?;
    Ok(())
}
