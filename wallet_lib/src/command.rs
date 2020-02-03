
use std::convert::From;
use crate::wallet::{Wallet, FilterOptions};
use crate::entry::Entry;
// use crate::entry::{EntryDisplay, EntryDisplayKind};
use crate::epic::Epic;
use crate::number::NumberType;
use crate::date::Date;
use crate::ext::BoolExt;

/// Command options hold all available options for ALL commands.
/// Not all commands will us all options.
#[derive(Debug, Clone)]
pub struct CommandOptions {
    pub wallet_path: String,
    pub html_path: String,
    pub id: Option<String>,
    pub title: Option<String>,
    pub date: Option<Date>,
    pub revenue: Option<NumberType>,
    pub expense: Option<NumberType>,
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
            html_path: String::new(),
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
            CommandKind::None => (),
            CommandKind::InitCommand => self.exec_init(),
            CommandKind::AddCommand => self.exec_add(),
            CommandKind::EpicCommand => self.exec_epic(),
            CommandKind::ListCommand => self.exec_list(),
            CommandKind::HtmlCommand => self.exec_html(),
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

        let options = FilterOptions::from(self.options.clone());
        let wallet = Wallet::new(self.options.get_wallet_path());

        let result = wallet.filter(options);

        // TODO: dynamic DisplayKind. use terminal width to determine which EntryDisplayKind value to use when no option is provided. maybe calculate width.

        // Kind
        // let mut kind = EntryDisplayKind::Normal;

        // if let Some(long_opt) = self.options.long {
        //     if long_opt {
        //         kind = EntryDisplayKind::Long;
        //     } else {
        //         kind = EntryDisplayKind::Short;
        //     }
        // }

        // TODO: use filterresult here
        // let entry_display = EntryDisplay::new(entries, kind);
        // entry_display.show();

        // Debug
        println!("-> result: {:?}", result);
    }

    /// HTML
    fn exec_html(&self) {
        println!("-> Command::exec_html()");

        let options = FilterOptions::from(self.options.clone());

        let wallet = Wallet::new(self.options.get_wallet_path());
        wallet.html(options);
    }
}

#[cfg(test)]
mod tests_basic {
    use super::{Command, CommandKind, CommandOptions};

    #[test]
    fn test_command1() {
        let options = CommandOptions::new();
        Command::new(CommandKind::None, options);
    }
}
