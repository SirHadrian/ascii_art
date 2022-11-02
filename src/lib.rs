use image::{io::Reader as ImageReader, GenericImageView};
use std::error::Error;

use image::{self, DynamicImage};

pub struct Config {
    pub scale: f32,
    pub reverse: bool,
}

impl Config {
    pub fn default() -> Config {
        Config {
            scale: 10.0,
            reverse: false,
        }
    }
}

pub struct Range {
    pub start: f32,
    pub end: f32,
}

impl Range {
    pub fn map_to_this_range(&self, other_range: &Range, value: f32) -> f32 {
        (value - self.start) / (self.end - self.start) * (other_range.end - other_range.start)
            + other_range.start
    }
}

pub struct Image {
    pub image: DynamicImage,
    pub width: f32,
    pub height: f32,
}

impl Image {
    pub fn new(path: &str) -> Result<Image, Box<dyn Error>> {
        let image = ImageReader::open(path)?.decode()?;
        let (width, height) = image.dimensions();

        Ok(
            Image{
                image,
                width: width as f32,
                height: height as f32,  
            }
        )
    }
}
