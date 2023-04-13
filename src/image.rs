use image::{io::Reader as ImageReader, DynamicImage, GenericImageView};
use std::error::Error;

pub struct Image {
    pub image: DynamicImage,
    pub width: f32,
    pub height: f32,
}

impl Image {
    pub fn new(path: &str) -> Result<Image, Box<dyn Error>> {
        let image = ImageReader::open(path)?.decode()?;
        let (width, height) = image.dimensions();

        Ok(Image {
            image,
            width: width as f32,
            height: height as f32,
        })
    }
}
