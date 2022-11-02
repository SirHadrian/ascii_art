use ascii_art::{Config, Range};
use image::{io::Reader as ImageReader, GenericImageView, Pixel};
use std::{env, process};

fn main() {
    let mut config = Config::default();

    let args: Vec<String> = env::args().collect();

    let mut test = args.iter();

    // Skip first value
    test.next();
    loop {
        let arg = match test.next() {
            Some(val) => val,
            None => break,
        };
        match arg as &str {
            "-h" | "--help" => {
                println!("Hello");
            }

            "-s" | "--scale" => match test.next() {
                Some(value) => {
                    config.scale = value.parse::<f32>().expect("Incorrect scale value");
                }
                None => {
                    eprintln!("No scale value supplied");
                    process::exit(1);
                }
            },

            _ => {
                eprintln!("Wrong argument type");
                process::exit(1);
            }
        }
    }

    run(&config);
}

fn run(config: &Config) {
    //let mapping = " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";
    let mapping = "  _.,-=+:;cba!?0123456789$W#@Ñ";

    let mapping_array: Vec<char> = mapping.chars().collect();
    let mapping_array_len = mapping_array.len();

    let test_image = ImageReader::open("cat.jpg")
        .expect("Could not find the file")
        .decode()
        .expect("Could not decode the file contents");
    let (width, height) = test_image.dimensions();

    let resize_width = (width as f32 / config.scale) as u32;
    let resize_height = (height as f32 / config.scale) as u32;

    let resized_image = test_image.thumbnail(resize_width, resize_height);

    let mut ascii_art = String::new();

    let from_range = Range {
        start: 0.0,
        end: 255.0,
    };
    let to_range = Range {
        start: 0.0,
        end: mapping_array_len as f32,
    };

    for (x, _y, pixel) in resized_image.pixels() {
        let mut avg: f32 = 0.0;
        for val in pixel.to_rgb().channels() {
            avg += *val as f32;
        }
        avg /= 3.0;

        ascii_art
            .push(mapping_array[from_range.map_to_this_range(&to_range, avg).floor() as usize]);

        if x == resize_width - 1 {
            ascii_art.push('\n');
        }
    }
    println!("{}", ascii_art);
}
