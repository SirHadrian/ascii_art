pub mod commands {

    use ascii_art::{Config, Image, Range};
    use image::{GenericImageView, Pixel};
    use std::process;

    pub fn load_file(path: &str) -> Image {
        match Image::new(path) {
            Ok(img) => img,
            Err(_) => {
                eprintln!("Error in loading the image from file");
                process::exit(1);
            }
        }
    }

    pub fn help(config: &Config) {
        println!("\nASCII ART");
        println!("\nConvert image to ascii representation");
        println!("\nUsage:[EXE] [OPTIONS] -p, --path <FILE>");
        println!("\nMandatory: ");
        println!("-p, --path <FILE>     Path to the image file");
        println!("\nOptions: ");
        println!("-h, --help            Print help information");
        println!(
            "-r, --reduce          Reduce the image size, default: {}",
            config.scale
        );
        println!(
            "-m, --mapping         Select ASCII character to use in mapping the image, default: {}",
            config.mapping
        );
    }

    pub fn run(config: &Config, image: &Image) {
        let chosen_map: Vec<char> = if let Some(map) = config.maps.get(&config.mapping) {
            map.chars().collect()
        } else {
            eprintln!("Could not get mapping from hashmap");
            process::exit(1);
        };
        let chosen_map_len = chosen_map.len();

        let resize_width = (image.width / config.scale).floor() as u32;
        let resize_height = (image.height / config.scale).floor() as u32;

        let resized_image = image.image.thumbnail(resize_width, resize_height);

        let mut ascii_art = String::new();

        let to_range = Range {
            start: 0.0,
            end: chosen_map_len as f32,
        };

        for (x, _y, pixel) in resized_image.pixels() {
            // Calculate average brightness
            let mut avg: f32 = 0.0;
            for val in pixel.to_rgb().channels() {
                avg += *val as f32;
            }
            avg /= 3.0;

            ascii_art.push(
                chosen_map[Range::get_rbg_range()
                    .map_to_this_range(&to_range, avg)
                    .floor() as usize],
            );

            // Check if is the last pixel in array
            if x == resize_width - 1 {
                ascii_art.push('\n');
            }
        }
        println!("{}", ascii_art);
    }
}
