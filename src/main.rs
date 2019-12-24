
extern crate clap;
use clap::{App, Arg};
use std::env;
use std::str::FromStr;
use chrono::{Local, DateTime, Datelike};
use wallet_lib::command::CommandOptions;
use wallet_lib::command::CommandKind;
use wallet_lib::command::Command;
use wallet_lib::ext::StringExt;
use wallet_lib::types::Number;
use wallet_lib::date::Date;

const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const APP_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");

/// Main
fn main() {
    println!("-> start");

    let args: Vec<String> = env::args().collect();
    println!("-> args: '{:?}'", args);

    // Vars Sub Command
    let vars_subcmd = App::new("vars")
        .about("Print variables.");

    // Init Sub Command
    let init_subcmd = App::new("init")
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
            .takes_value(true))
        .arg(Arg::with_name("date")
            .short("d")
            .long("date")
            .help("Date")
            .takes_value(true))
        .arg(Arg::with_name("id")
            .long("id")
            .help("ID")
            .takes_value(true))
        .arg(Arg::with_name("force")
            .short("f")
            .long("force")
            .help("Force add, even if ID already exists.")
            .takes_value(false));

    // List Sub Command
    let list_subcmd = App::new("list")
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
    let mut cmd_options = CommandOptions::new();
    let mut cmd_kind = CommandKind::None;

    // Get Argument matches.
    let matches = app.get_matches();
    println!("-> matches '{:?}'", matches);

    if matches.is_present("wallet") {
        println!("-> wallet is present: {:?}", matches.value_of("wallet").unwrap());
        cmd_options.wallet_path = matches.value_of("wallet").unwrap().to_string();
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

            // Cmd
            cmd_kind = CommandKind::InitCommand;
        },
        ("add", Some(add_matches)) => {
            println!("-> cmd: add");

            // Cmd
            cmd_kind = CommandKind::AddCommand;

            // Interactive
            if add_matches.is_present("interactive") {
                println!("-> interactive is present");
            }

            // Revenue
            if add_matches.is_present("revenue") {
                // Convert from &str to String.
                let vs = add_matches.value_of("revenue").unwrap().to_string();
                // println!("-> vs {:?}", vs);

                // Convert from String to Number.
                let vn: Number = vs.replace_comma().to_num();

                // println!("-> revenue is present: '{}'", vn);
                cmd_options.revenue = vn;
            }

            // Expense
            if add_matches.is_present("expense") {
                // Convert from &str to String.
                let vs = add_matches.value_of("expense").unwrap().to_string();
                // println!("-> vs {:?}", vs);

                // Convert from String to Number.
                let vn: Number = vs.replace_comma().to_num();

                // println!("-> expense is present: '{}'", vn);
                cmd_options.expense = vn;
            }

            // Date
            if add_matches.is_present("date") {
                // &str
                let vs = add_matches.value_of("date").unwrap();
                // println!("-> vs '{:?}'", vs);

                cmd_options.date = Date::from_str(vs).expect("Unable to parse given Date");
                // println!("-> date '{:?}'", cmd_options.date);
            }
        },
        ("list", Some(_list_matches)) => {
            println!("-> cmd: list");

            // Cmd
            cmd_kind = CommandKind::ListCommand;
        },
        ("html", Some(html_matches)) => {
            println!("-> cmd: html");
            if html_matches.is_present("path") {
                println!("-> path is present: {}", matches.value_of("wallet").unwrap());
            }

            // Cmd
            cmd_kind = CommandKind::HtmlCommand;
        },
        _ => unreachable!(),
    }

    // Now
    let now: DateTime<Local> = Local::now();

    // Correct date.
    if !cmd_options.date.has_year() {
        // println!("-> year missing");
        cmd_options.date.set_year(now.year());
    }

    if !cmd_options.date.has_month() {
        // println!("-> month missing");
        cmd_options.date.set_month(now.month());
    }

    if !cmd_options.date.has_day() {
        // println!("-> day missing");
        cmd_options.date.set_day(now.day());
    }

    println!("-> date: {:?}", cmd_options.date);

    println!("-> cmd: {:?}", cmd_kind);
    let cmd = Command::new(cmd_kind, cmd_options);
    println!("-> cmd: {:?}", cmd);

    println!("-> exec");
    cmd.exec();

    println!("-> end");
}
