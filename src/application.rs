use crate::arguments::Config;
use crate::image::Image;
use crate::range::Range;

use image::GenericImageView;
use pad::{Alignment, PadStr};

use std::process;

// Process the image with the supplied config
pub fn run(config: &Config) {
    if config.path.is_empty() {
        eprintln!("No path provided");
        process::exit(1);
    }

    let image = load_file(&config.path);

    // Get the chosen map from the config hashmap
    let string_map = if let Some(map) = config.maps.get(&config.mapping) {
        map
    } else {
        eprintln!("Could not get mapping from hashmap");
        process::exit(1);
    };

    // Pad the map with the number of spaces supplied in the config
    let pad_width = string_map.len() + config.spaces as usize;
    let padded_string_map = string_map.pad_to_width_with_alignment(pad_width, Alignment::Right);

    let mut chosen_map: Vec<char> = padded_string_map.chars().collect();

    // Inverse the contrast if arg is supplied
    if config.inverse {
        chosen_map.reverse();
    }

    let chosen_map_len = chosen_map.len() - 1;

    // Make the image smaller
    let resize_width = (image.width / config.scale) as u32;
    let resize_height = (image.height / config.scale) as u32;
    let resized_image = image.image.thumbnail(resize_width, resize_height);

    let mut ascii_art = String::new();

    let to_range = Range {
        start: 0.0,
        end: chosen_map_len as f32,
    };

    // Process each pixel in the image
    for (x, _y, pixel) in resized_image.pixels() {
        // Calcualte brightness based on a relative luminance function R 0 | G 1 | B 2
        let red = pixel.0[0] as f32;
        let green = pixel.0[1] as f32;
        let blue = pixel.0[2] as f32;

        ascii_art.push(
            chosen_map[Range::get_rbg_range()
                .map_to_this_range(&to_range, calc_luminance(red, green, blue))
                .floor() as usize],
        );

        // Check if is the last pixel in array
        if x == resize_width - 1 {
            ascii_art.push('\n');
        }
    }
    // Print the final image to standard output
    println!("{}", ascii_art);
}

fn calc_luminance(red: f32, green: f32, blue: f32) -> f32 {
    red * 0.2126 + 0.7152 * green + 0.0722 * blue
}

// Get image path
fn load_file(path: &str) -> Image {
    match Image::new(path) {
        Ok(img) => img,
        Err(_) => {
            eprintln!("Error in loading the image from file");
            process::exit(1);
        }
    }
}
