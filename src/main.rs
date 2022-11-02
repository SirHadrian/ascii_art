use ascii_art::{Config, Image, Range};
use image::{GenericImageView, Pixel};
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
                help(&config);
                process::exit(0);
            }

            "-r" | "--reduce" => match test.next() {
                Some(value) => {
                    config.scale = value.parse::<f32>().expect("Incorrect scale value");
                }
                None => {
                    eprintln!("No scale value supplied");
                    process::exit(1);
                }
            },

            "-p" | "--path" => match test.next() {
                Some(path) => {
                    let image = load_file("cat.jpg");
                    run(&config, &image);
                }
                None => {
                    eprintln!("No path supplied");
                    process::exit(1);
                }
            },

            _ => {
                eprintln!("Wrong argument type");
                process::exit(1);
            }
        }
    }
}

fn load_file(path: &str) -> Image {
    match Image::new(path) {
        Ok(img) => img,
        Err(_) => {
            eprintln!("Error in loading the image from file");
            process::exit(1);
        }
    }
}

fn help(config: &Config) {
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
}

fn run(config: &Config, image: &Image) {
    //let mapping = " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";
    let mapping = "  _.,-=+:;cba!?0123456789$W#@Ñ";

    let mapping_array: Vec<char> = mapping.chars().collect();
    let mapping_array_len = mapping_array.len();

    let resize_width = (image.width / config.scale).floor() as u32;
    let resize_height = (image.height / config.scale).floor() as u32;

    let resized_image = image.image.thumbnail(resize_width, resize_height);

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
