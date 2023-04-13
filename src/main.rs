mod application;
mod arguments;
mod image;
mod range;

use std::env;

fn main() {
    // Get arguments from cli
    let retrive_args: Vec<String> = env::args().collect();

    // Configs setup
    let config = arguments::Config::parse(retrive_args);

    application::run(&config);
}
