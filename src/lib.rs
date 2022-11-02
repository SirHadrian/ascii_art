use image::{io::Reader as ImageReader, DynamicImage, GenericImageView};
use std::collections::HashMap;
use std::error::Error;

pub struct Config {
    pub scale: f32,
    pub inverse: bool,
    pub mapping: u8,
    pub maps: HashMap<u8, String>,
    pub spaces: u8,
}

impl Config {
    // Default configuration for processing the image
    pub fn default() -> Config {
        let mut maps = HashMap::new();

        maps.insert(1, String::from("_.,-=+:;cba!?0123456789$W#@Ñ"));
        maps.insert(2, String::from("`.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@"));

        Config {
            scale: 10.0,
            inverse: false,
            mapping: 1,
            maps,
            spaces: 1,
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

    pub fn get_rbg_range() -> Range {
        Range {
            start: 0.0,
            end: 255.0,
        }
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

        Ok(Image {
            image,
            width: width as f32,
            height: height as f32,
        })
    }
}
