use ascii_art::{Config, Image};
use std::{env, process};

mod commands;
use crate::commands::actions;

fn main() {
    let mut config = Config::default();

    let args: Vec<String> = env::args().collect();

    let mut image: Option<Image> = None;

    let mut test = args.iter();

    // Skip first value
    test.next();
    loop {
        let arg = match test.next() {
            Some(val) => val,
            None => break,
        };
        match arg as &str {
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
                    image = Some(actions::load_file(path));
                }
                None => {
                    eprintln!("No path supplied");
                    process::exit(1);
                }
            },

            "-m" | "--mapping" => match test.next() {
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

            "-s" | "--spaces" => match test.next() {
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
                actions::help(&config);
                process::exit(0);
            }

            _ => {
                eprintln!("Wrong argument type");
                process::exit(1);
            }
        }
    }

    if let Some(img) = image {
        actions::run(&config, &img);
    }
}
