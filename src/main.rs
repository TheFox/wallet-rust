
extern crate clap;
use clap::{App, Arg, ArgMatches};
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
    // println!("-> args: '{:?}'", args);

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
        .arg(Arg::with_name("title")
            .short("t")
            .long("title")
            .help("Title")
            .takes_value(true))
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
        .arg(Arg::with_name("category")
            .short("c")
            .long("category")
            .help("Category")
            .takes_value(true))
        .arg(Arg::with_name("comment")
            .short("o")
            .long("comment")
            .help("Comment")
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
            .takes_value(false))
        .arg(Arg::with_name("epic")
            .short("x")
            .long("epic")
            .help("Epic")
            .takes_value(true));

    // Epic Sub Command
    let epic_subcmd = App::new("epic")
        .about("Add a new epic.")
        .arg(Arg::with_name("title")
            .short("t")
            .long("title")
            .help("Title")
            .takes_value(true))
        .arg(Arg::with_name("handle")
            .long("handle")
            .help("Handle (For example 'myepic')")
            .takes_value(true))
        .arg(Arg::with_name("bgcolor")
            .long("bgcolor")
            .help("Background Color (HTML)")
            .takes_value(true));

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
        .subcommand(epic_subcmd)
        .subcommand(list_subcmd)
        .subcommand(html_subcmd)
        .arg(wallet_arg);

    // Command Options
    let mut cmd_options = CommandOptions::new();
    let mut cmd_kind = CommandKind::None;

    // Get Argument matches.
    let matches = app.get_matches();
    // println!("-> matches '{:?}'", matches);

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
            println!("-> cmd: add ({:?})", add_matches);

            // Cmd
            cmd_kind = CommandKind::AddCommand;

            // Interactive
            if add_matches.is_present("interactive") {
                println!("-> interactive is present");
            }

            // Title
            set_title(add_matches, &mut cmd_options);

            // Date
            set_date(add_matches, &mut cmd_options);

            // Revenue
            if add_matches.is_present("revenue") {
                // Convert from &str to String.
                let vs = add_matches.value_of("revenue").unwrap().to_string();
                // println!("-> vs {:?}", vs);

                // Convert from String to Number.
                let vn: Number = vs.replace_comma().to_num();

                // println!("-> revenue is present: '{}'", vn);
                cmd_options.revenue = Some(vn);
            }

            // Expense
            if add_matches.is_present("expense") {
                // Convert from &str to String.
                let vs = add_matches.value_of("expense").unwrap().to_string();
                // println!("-> vs {:?}", vs);

                // Convert from String to Number.
                let vn: Number = vs.replace_comma().to_num();

                // println!("-> expense is present: '{}'", vn);
                cmd_options.expense = Some(vn);
            }

            // Category
            set_category(add_matches, &mut cmd_options);

            // Comment
            set_comment(add_matches, &mut cmd_options);

            // Epic
            set_epic(add_matches, &mut cmd_options);

            // ID
            if add_matches.is_present("id") {
                // Convert from &str to String.
                let vs = add_matches.value_of("id").unwrap().to_string();
                println!("-> vs {:?}", vs);

                println!("-> id is present: '{:?}'", vs);

                cmd_options.id = Some(vs);
            }

            // Force
            if add_matches.is_present("force") {
                let v = add_matches.value_of("force");
                println!("-> force is present: '{:?}'", v);

                cmd_options.force = true;
            }
        },
        ("epic", Some(epic_matches)) => {
            println!("-> cmd: epic ({:?})", epic_matches);

            // Cmd
            cmd_kind = CommandKind::EpicCommand;

            set_handle(epic_matches, &mut cmd_options);
            set_title(epic_matches, &mut cmd_options);
            set_bgcolor(epic_matches, &mut cmd_options);
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

    println!("-> date: {:?}", cmd_options.date);

    println!("-> cmd: {:?}", cmd_kind);
    let cmd = Command::new(cmd_kind, cmd_options);
    println!("-> cmd: {:?}", cmd);

    println!("-> exec");
    cmd.exec();

    println!("-> end");
}

fn set_title(matches: &ArgMatches, cmd_options: &mut CommandOptions) {
    // println!("-> set_title()");

    if !matches.is_present("title") {
        return;
    }

    // &str
    let vs = matches.value_of("title").unwrap();
    // println!("-> vs '{:?}'", vs);

    cmd_options.title = Some(vs.to_string());
}

fn set_date(matches: &ArgMatches, cmd_options: &mut CommandOptions) {
    println!("-> set_date()");

    if !matches.is_present("date") {
        return;
    }

    // &str
    let vs = matches.value_of("date").unwrap();
    println!("-> vs '{:?}'", vs);

    cmd_options.date = Date::from_str(vs).expect("Unable to parse given Date");
    // println!("-> date '{:?}'", cmd_options.date);

    // Now
    let now: DateTime<Local> = Local::now();

    // Correct date.
    if !cmd_options.date.has_year() {
        println!("-> year missing");
        cmd_options.date.set_year(now.year());
    }

    if !cmd_options.date.has_month() {
        println!("-> month missing");
        cmd_options.date.set_month(now.month());
    }

    if !cmd_options.date.has_day() {
        println!("-> day missing");
        cmd_options.date.set_day(now.day());
    }
}

fn set_category(matches: &ArgMatches, cmd_options: &mut CommandOptions) {
    // println!("-> set_category()");

    if !matches.is_present("category") {
        return;
    }

    // &str
    let vs = matches.value_of("category").unwrap();
    // println!("-> vs '{:?}'", vs);

    cmd_options.category = Some(vs.to_string());
}

fn set_comment(matches: &ArgMatches, cmd_options: &mut CommandOptions) {
    // println!("-> set_comment()");

    if !matches.is_present("comment") {
        return;
    }

    // &str
    let vs = matches.value_of("comment").unwrap();
    // println!("-> vs '{:?}'", vs);

    cmd_options.comment = Some(vs.to_string());
}

fn set_epic(matches: &ArgMatches, cmd_options: &mut CommandOptions) {
    // println!("-> set_epic()");

    if !matches.is_present("epic") {
        return;
    }

    // &str
    let vs = matches.value_of("epic").unwrap();
    // println!("-> vs '{:?}'", vs);

    cmd_options.epic = Some(vs.to_string());
}

fn set_handle(matches: &ArgMatches, cmd_options: &mut CommandOptions) {
    // println!("-> set_handle()");

    if !matches.is_present("handle") {
        return;
    }

    // &str
    let vs = matches.value_of("handle").unwrap();
    // println!("-> vs '{:?}'", vs);

    cmd_options.handle = Some(vs.to_string());
}

fn set_bgcolor(matches: &ArgMatches, cmd_options: &mut CommandOptions) {
    // println!("-> set_bgcolor()");

    if !matches.is_present("bgcolor") {
        return;
    }

    // &str
    let vs = matches.value_of("bgcolor").unwrap();
    // println!("-> vs '{:?}'", vs);

    cmd_options.bgcolor = Some(vs.to_string());
}
