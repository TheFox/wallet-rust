
use std::convert::From;
use crate::wallet::{Wallet, FilterOptions};
use crate::entry::{Entry, EntrySum};
use crate::epic::Epic;
use crate::types::Number;
use crate::date::Date;
use crate::ext::BoolExt;
use crate::string::{ShortString, ZeroString};

/// Command options hold all available options for ALL commands.
/// Not all commands will us all options.
#[derive(Debug, Clone)]
pub struct CommandOptions {
    pub wallet_path: String,
    pub id: Option<String>,
    pub title: Option<String>,
    pub date: Option<Date>,
    pub revenue: Option<Number>,
    pub expense: Option<Number>,
    pub filter_revenue: Option<bool>,
    pub filter_expense: Option<bool>,
    pub category: Option<String>,
    pub comment: Option<String>,
    pub force: bool,
    pub epic: Option<String>,
    pub handle: Option<String>,
    pub bgcolor: Option<String>,
    pub long: Option<bool>, // true = long, false = short
}

/// Common Options for commands.
impl CommandOptions {
    /// Create a new CommandOptions object.
    pub fn new() -> Self {
        CommandOptions {
            wallet_path: String::new(),
            id: None,
            title: None,
            date: None,
            revenue: None,
            expense: None,
            filter_revenue: None,
            filter_expense: None,
            category: None,
            comment: None,
            force: false,
            epic: None,
            handle: None,
            bgcolor: None,
            long: None,
        }
    }

    fn get_wallet_path(&self) -> String {
        println!("-> CommandOptions::get_wallet_path()");

        self.wallet_path.clone()
    }
}

/// The Kind of command to execute.
#[derive(Debug)]
pub enum CommandKind {
    None,
    InitCommand,
    AddCommand,
    EpicCommand,
    ListCommand,
    HtmlCommand,
}

#[derive(Debug)]
pub struct Command {
    kind: CommandKind,
    options: CommandOptions,
}

impl Command {
    pub fn new(kind: CommandKind, options: CommandOptions) -> Self {
        println!("-> Command::new()");

        Self {
            kind,
            options,
        }
    }

    pub fn exec(&self) {
        println!("-> Command::exec()");

        match self.kind {
            CommandKind::InitCommand => self.exec_init(),
            CommandKind::AddCommand => self.exec_add(),
            CommandKind::EpicCommand => self.exec_epic(),
            CommandKind::ListCommand => self.exec_list(),
            CommandKind::HtmlCommand => self.exec_html(),
            _ => unreachable!("Command not implemented"),
        }
    }

    /// Init
    fn exec_init(&self) {
        println!("-> Command::exec_init()");
        Wallet::new(self.options.get_wallet_path());
    }

    /// Add
    fn exec_add(&self) {
        println!("-> Command::exec_add()");

        let entry = Entry::from(self.options.clone());
        let wallet = Wallet::new(self.options.get_wallet_path());
        let added = wallet.add(entry, self.options.force);
        println!("Added: {}", added.to_string());
    }

    /// Epic
    fn exec_epic(&self) {
        println!("-> Command::exec_epic()");

        // TODO: --remove

        let mut epic = Epic::new();

        if let Some(handle) = &self.options.handle {
            epic.set_handle(handle.to_string());
        }
        if let Some(title) = &self.options.title {
            epic.set_title(title.to_string());
        }
        if let Some(bgcolor) = &self.options.bgcolor {
            epic.set_bgcolor(bgcolor.to_string());
        }

        let wallet = Wallet::new(self.options.get_wallet_path());
        let added = wallet.add_epic(epic);
        println!("Added: {}", added.yn());
    }

    /// List
    ///
    /// https://doc.rust-lang.org/std/fmt/#named-parameters
    /// https://stackoverflow.com/questions/32572486/how-can-i-use-a-dynamic-format-string-with-the-format-macro
    fn exec_list(&self) {
        println!("-> Command::exec_list()");

        let mut sum = EntrySum::new();

        // TODO
        let options = FilterOptions::from(self.options.clone());
        let wallet = Wallet::new(self.options.get_wallet_path());
        let entries = wallet.filter(options);

        // Format
        let mut long = false;
        let mut short = false;

        if let Some(long_opt) = self.options.long {
            if long_opt {
                long = true;
            } else {
                short = true;
            }
        }

        if entries.len() > 0 {
            if long {
                // Long
                println!("#### Date          Revenue    Expense    Balance             Category                 Epic   Title");
            }
            else if short {
                // Short
                println!("#### Date          Revenue    Expense    Balance  Title");
            }
            else {
                // Default
                println!("#### Date          Revenue    Expense    Balance   Category       Epic  Title");
            }

            for entry in entries {
                sum.inc();
                sum.inc_revenue(entry.revenue());
                sum.inc_expense(entry.expense());
                sum.inc_balance(entry.balance());

                if long {
                    let revenue = ZeroString::from(entry.revenue());

                    println!("{:<4} {} {} {:>10.2} {:>10.2} {:>20} {:>20}   {}",
                        sum.n,
                        entry.date().ymd(),
                        revenue.to_string(),
                        entry.expense(),
                        entry.balance(),
                        entry.category(),
                        entry.epic(),
                        entry.title(),
                    );
                }
                else if short {
                    let title = ShortString::from(entry.title(), 23);

                    println!("{:<4} {} {:>10.2} {:>10.2} {:>10.2}  {}",
                        sum.n,
                        entry.date().ymd(),
                        entry.revenue(),
                        entry.expense(),
                        entry.balance(),
                        title,
                    );
                }
                else {
                    let category = ShortString::from(entry.category(), 10);
                    let mut epic = ShortString::from(entry.epic(), 10);
                    let title = ShortString::from(entry.title(), 23);

                    if epic.to_string() == "default".to_string() {
                        epic = ShortString::new();
                    }

                    println!("{:<4} {} {:>10.2} {:>10.2} {:>10.2} {:>10} {:>10}  {}",
                        sum.n,
                        entry.date().ymd(),
                        entry.revenue(),
                        entry.expense(),
                        entry.balance(),
                        category.to_string(),
                        epic.to_string(),
                        title.to_string(),
                    );
                }
            }

            println!("TOTAL           {:>10.2} {:>10.2} {:>10.2}",
                sum.revenue,
                sum.expense,
                sum.balance);
        } else {
            println!("No entries found.");
        }
    }

    /// HTML
    fn exec_html(&self) {
        println!("-> Command::exec_html()");

        let wallet = Wallet::new(self.options.get_wallet_path());
        wallet.html(); // TODO
    }
}
