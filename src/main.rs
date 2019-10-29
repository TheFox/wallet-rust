
extern crate clap;
use clap::{App, Arg};
use std::env;
// use wallet_lib::command::CommandOptions;

const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const APP_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");

fn main() {
    println!("-> start");

    let args: Vec<String> = env::args().collect();
    println!("-> cmd: '{:?}'", args);

    // Init Sub Command
    let mut init_subcmd = App::new("init")
        .about("Initialize a new wallet.");
    // init_subcmd.set_app_defaults();
    set_app_defaults(&mut init_subcmd);

    // Add Sub Command
    let add_subcmd = App::new("add")
        .about("Add a new entry.");

    // HTML Sub Command
    let html_subcmd = App::new("html")
        .about("Generate HTML output.");

    // Main App
    let app = App::new(APP_NAME)
        .version(APP_VERSION)
        .author(APP_AUTHORS)
        .about(APP_HOMEPAGE)
        .usage("wallet <command> [options]")
        .subcommand(init_subcmd)
        .subcommand(add_subcmd)
        .subcommand(html_subcmd)
        .arg(
            Arg::with_name("wallet")
                .short("w")
                .long("wallet")
                .value_name("PATH")
                .help("Path to the wallet directory.")
                .takes_value(true)
        );

    // Get Argument matches.
    let matches = app.get_matches();

    println!("-> matches '{:?}'", matches);

    println!("APP_NAME {}", APP_NAME);
    println!("APP_VERSION {}", APP_VERSION);

    println!("-> end");
}

// trait AppDefaults {
//     fn set_app_defaults(&mut self);
// }
// impl AppDefaults for App<'_, '_> {
//     fn set_app_defaults(&mut self) {
//         // use std::mem;

//         println!("-> set_app_defaults");

//         *self = self.version(APP_VERSION);

//         // mem::replace(&mut borrowed.knight, TheDarkKnight)
//         // mem::replace(&mut self, self.version(APP_VERSION));
//     }
// }

fn set_app_defaults(app: &mut App) {
    println!("-> set_app_defaults");

    // (*app).version(APP_VERSION);
    app.author(APP_AUTHORS);
    // app.about(APP_HOMEPAGE);
}
