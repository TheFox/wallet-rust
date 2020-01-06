
use std::convert::From;
use crate::wallet::Wallet;
use crate::entry::Entry;
use crate::epic::Epic;
use crate::types::Number;
use crate::date::Date;
use crate::ext::BoolExt;

/// Command options hold all available options for ALL commands.
/// Not all commands will us all options.
#[derive(Debug, Clone)]
pub struct CommandOptions {
    pub wallet_path: String,
    pub id: Option<String>,
    pub title: Option<String>,
    pub date: Date,
    pub revenue: Option<Number>,
    pub filter_revenue: Option<bool>,
    pub expense: Option<Number>,
    pub filter_expense: Option<bool>,
    pub category: Option<String>,
    pub comment: Option<String>,
    pub force: bool,
    pub epic: Option<String>,
    pub handle: Option<String>,
    pub bgcolor: Option<String>,
}

/// Common Options for commands.
impl CommandOptions {
    /// Create a new CommandOptions object.
    pub fn new() -> Self {
        CommandOptions {
            wallet_path: String::new(),
            id: None,
            title: None,
            date: Date::new(),
            revenue: None,
            filter_revenue: None,
            expense: None,
            filter_expense: None,
            category: None,
            comment: None,
            force: false,
            epic: None,
            handle: None,
            bgcolor: None,
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
    fn exec_list(&self) {
        println!("-> Command::exec_list()");

        let wallet = Wallet::new(self.options.get_wallet_path());
        wallet.list(); // TODO
    }

    /// HTML
    fn exec_html(&self) {
        println!("-> Command::exec_html()");

        let wallet = Wallet::new(self.options.get_wallet_path());
        wallet.html(); // TODO
    }
}
