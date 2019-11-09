
extern crate clap;
use clap::{App, Arg};
use std::env;
use wallet_lib::command::CommandOptions;
use wallet_lib::command::Command;
use wallet_lib::command::{InitCommand, AddCommand, ListCommand, HtmlCommand};
use wallet_lib::ext::StringExt;
use wallet_lib::types::Number;

const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const APP_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");

/// Main
fn main() {
    println!("-> start");

    // let _s1 = String::from("1,23");
    // println!("-> _s1: '{:?}'", _s1);
    // println!("-> _s1 rc: '{:?}'", _s1.replace_comma());
    // println!("-> _s1 num: '{:?}'", _s1.replace_comma().to_num());
    // let _f1: f64 = _s1.replace_comma().to_num();
    // println!("-> _f1: '{:?}'", _f1);

    let args: Vec<String> = env::args().collect();
    println!("-> args: '{:?}'", args);

    // Vars Sub Command
    let mut vars_subcmd = App::new("vars")
        .about("Print variables.");

    // Init Sub Command
    let mut init_subcmd = App::new("init")
        .about("Initialize a new wallet.");

    // Add Sub Command
    let add_subcmd = App::new("add")
        .about("Add a new entry.")
        .arg(Arg::with_name("interactive")
            .short("i")
            .long("interactive")
            .help("Run the Add command in interactive mode.")
            .takes_value(false))
        .arg(Arg::with_name("revenue")
            .short("r")
            .long("revenue")
            .help("Set a Revenue.")
            .takes_value(true))
        .arg(Arg::with_name("expense")
            .short("e")
            .long("expense")
            .help("Set a Expense.")
            .takes_value(true));

    // List Sub Command
    let mut list_subcmd = App::new("list")
        .about("List entries.");

    // HTML Sub Command
    let html_subcmd = App::new("html")
        .about("Generate HTML output.");

    // Common Arguments
    let wallet_arg = Arg::with_name("wallet")
        .short("w")
        .long("wallet")
        .value_name("PATH")
        .help("Path to the wallet directory.")
        .takes_value(true);

    // Main App
    let app = App::new(APP_NAME)
        .version(APP_VERSION)
        .author(APP_AUTHORS)
        .about(APP_HOMEPAGE)
        .usage("wallet [OPTIONS] [SUBCOMMAND] [SUBCOMMAND_OPTIONS]")
        .subcommand(vars_subcmd)
        .subcommand(init_subcmd)
        .subcommand(add_subcmd)
        .subcommand(list_subcmd)
        .subcommand(html_subcmd)
        .arg(wallet_arg);

    // Command Options
    let mut options = CommandOptions::new();

    // Get Argument matches.
    let matches = app.get_matches();
    println!("-> matches '{:?}'", matches);

    if matches.is_present("wallet") {
        println!("-> wallet is present: {:?}", matches.value_of("wallet").unwrap());
        options.wallet_path = matches.value_of("wallet").unwrap().to_string();
    }

    match matches.subcommand() {
        ("vars", _) => {
            println!("-> cmd: vars");
            println!("APP_NAME '{}'", APP_NAME);
            println!("APP_VERSION '{}'", APP_VERSION);
            return;
        },
        ("init", Some(init_matches)) => {
            println!("-> cmd: init");
            if init_matches.is_present("interactive") {
                println!("-> interactive is present");
            }
            let cmd = InitCommand { options };
            cmd.exec();
        },
        ("add", Some(add_matches)) => {
            println!("-> cmd: add");

            // Interactive
            if add_matches.is_present("interactive") {
                println!("-> interactive is present");
            }

            // Revenue
            if add_matches.is_present("revenue") {
                // Convert from &str to String.
                let vs = add_matches.value_of("revenue").unwrap().to_string();
                println!("-> v {:?}", vs);

                // Convert from String to Number.
                let vn: Number = vs.replace_comma().to_num();

                println!("-> revenue is present: '{}'", vn);
                options.revenue = vn;
            }

            // Expense
            if add_matches.is_present("expense") {
                // Convert from &str to String.
                let vs = add_matches.value_of("expense").unwrap().to_string();
                println!("-> v {:?}", vs);

                // Convert from String to Number.
                let vn: Number = vs.replace_comma().to_num();

                println!("-> expense is present: '{}'", vn);
                options.expense = vn;
            }

            let cmd = AddCommand { options };
            cmd.exec();
        },
        ("html", Some(html_matches)) => {
            println!("-> cmd: html");
            if html_matches.is_present("path") {
                println!("-> path is present: {}", matches.value_of("wallet").unwrap());
            }
            let cmd = HtmlCommand { options };
            cmd.exec();
        },
        _ => unreachable!(),
    }

    println!("-> end");
}
