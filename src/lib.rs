use image::{io::Reader as ImageReader, DynamicImage, GenericImageView};
use std::collections::HashMap;
use std::error::Error;

pub struct Config {
    pub scale: f32,
    pub inverse: bool,
    pub mapping: u8,
    pub maps: HashMap<u8, String>,
    pub spaces: u8,
    pub rgb_values: [f32; 3],
}

impl Config {
    // Default configuration for processing the image
    pub fn default() -> Config {
        let mut greyscale_maps = HashMap::new();

        greyscale_maps.insert(1, String::from("_.,-=+:;cba!?0123456789$W#@Ñ"));
        greyscale_maps.insert(2, String::from("`.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@"));
        greyscale_maps.insert(
            3,
            String::from(".`'\"^,:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"),
        );
        greyscale_maps.insert(4, String::from(".:-=+*#%@"));

        Config {
            scale: 10.0,
            inverse: false,
            mapping: 1,
            maps: greyscale_maps,
            spaces: 1,
            rgb_values: [0.2126, 0.7152, 0.0722],
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
