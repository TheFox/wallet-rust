
use std::env;
use wallet_lib::command::CommandOptions;

const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

// #[macro_use]
// extern crate clap;
// use clap::App;

// // #[clap(version = "v1.0-beta")]
// #[derive(Clap)]
// struct Opts;

extern crate clap;
use clap::App;

fn main() {
    println!("-> start");

    let args: Vec<String> = env::args().collect();
    println!("-> cmd: '{:?}'", args);

    // let mut options = CommandOptions::new();
    // println!("-> options: '{:?}'", options);

    // Opts::parse();
    // let opts: Opts = Opts::parse();

    // println!("config: {}", opts.config);
    // println!("Using input file: {}", opts.input);
    // println!("file: {}", opts.file);

    App::new("fake").version(APP_VERSION).get_matches();

    // println!("APP_VERSION {}", APP_VERSION);

    println!("-> end");
}
