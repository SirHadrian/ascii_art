use std::{collections::HashMap, process};

pub struct Config {
    pub path: String,
    pub scale: f32,
    pub inverse: bool,
    pub mapping: u8,
    pub maps: HashMap<u8, String>,
    pub spaces: u8,
}

impl Config {
    // Default configuration for processing the image
    fn default() -> Config {
        let mut greyscale_maps = HashMap::new();

        greyscale_maps.insert(1, String::from("_.,-=+:;cba!?0123456789$W#@Ñ"));
        greyscale_maps.insert(2, String::from("`.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@"));
        greyscale_maps.insert(
            3,
            String::from(".`'\"^,:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"),
        );
        greyscale_maps.insert(4, String::from(".:-=+*#%@"));

        Config {
            path: String::from(""),
            scale: 10.0,
            inverse: false,
            mapping: 1,
            maps: greyscale_maps,
            spaces: 1,
        }
    }

    pub fn parse(args: Vec<String>) -> Config {
        let mut arguments = args.iter();

        let mut config = Config::default();

        // Skip first value
        arguments.next();
        loop {
            // Get the next argument in iterator
            let arg = match arguments.next() {
                Some(val) => val,
                None => break,
            };
            match arg as &str {
                "-r" | "--reduce" => match arguments.next() {
                    Some(value) => {
                        let reduce = value.parse::<f32>().expect("Incorrect reduce value");

                        if reduce > 0.0 {
                            config.scale = reduce;
                        } else {
                            eprintln!("Incorect reduce value");
                            process::exit(1);
                        }
                    }
                    None => {
                        eprintln!("No scale value supplied");
                        process::exit(1);
                    }
                },

                "-p" | "--path" => match arguments.next() {
                    Some(path) => {
                        config.path = String::from(path);
                    }
                    None => {
                        eprintln!("No path supplied");
                        process::exit(1);
                    }
                },

                "-m" | "--mapping" => match arguments.next() {
                    Some(mode) => {
                        let mapping = mode.parse::<u8>().expect("Incorect mapping value");
                        if config.maps.contains_key(&mapping) {
                            config.mapping = mapping;
                        } else {
                            eprintln!("Incorect mapping option");
                            process::exit(1);
                        }
                    }
                    None => (),
                },

                "-s" | "--spaces" => match arguments.next() {
                    Some(value) => {
                        let spaces = value.parse::<u8>().expect("Incorect spaces value");
                        config.spaces = spaces;
                    }
                    None => (),
                },

                "-i" | "--inverse" => {
                    config.inverse = true;
                }

                "-h" | "--help" => {
                    // Help message
                    let help = || {
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
            "-m, --mapping         Select ASCII character to use in mapping the image 1-4, default: {}",
            config.mapping
        );
                        println!("-i, --inverse         Inverse the mapping of ascii chars, default darkest -> brightest");
                        println!("-s, --spaces          Pad extra spaces to the left of mapping strings to increase contrast, default: {}", config.spaces);
                    };

                    help();
                    process::exit(0);
                }

                _ => {
                    eprintln!("Wrong argument type");
                    process::exit(1);
                }
            }
        }
        config
    }
}
