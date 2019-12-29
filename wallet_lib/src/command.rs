
use crate::wallet::Wallet;
use crate::entry::Entry;
use crate::types::Number;
use crate::date::Date;
use crate::ext::BoolExt;

/// Command options hold all available options for ALL commands.
/// Not all commands will us all options.
#[derive(Debug)]
pub struct CommandOptions {
    pub wallet_path: String,
    pub id: Option<String>,
    pub date: Date,
    pub revenue: Option<Number>,
    pub expense: Option<Number>,
    pub category: Option<String>,
    pub comment: Option<String>,
    pub force: bool,
    pub epic: Option<String>,
}

/// Common Options for commands.
impl CommandOptions {
    /// Create a new CommandOptions object.
    pub fn new() -> Self {
        CommandOptions {
            wallet_path: String::new(),
            id: None,
            date: Date::new(),
            revenue: None,
            expense: None,
            category: None,
            comment: None,
            force: false,
            epic: None,
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

        println!("-> ID: '{:?}'", self.options.id);
        println!("-> revenue: '{:?}'", self.options.revenue);
        println!("-> expense: '{:?}'", self.options.expense);

        let mut entry = Entry::new(); // TODO: use from_command_options() here
        if let Some(ref id) = self.options.id {
            println!("-> take ID: {:?}", id);
            entry.set_id(id.clone());
        }
        entry.set_date(self.options.date);

        if let Some(revenue) = self.options.revenue {
            entry.set_revenue(revenue);
        }
        if let Some(expense) = self.options.expense {
            entry.set_expense(expense);
        }
        if let Some(category) = &self.options.category {
            entry.set_category(category.to_string());
        }
        if let Some(comment) = &self.options.comment {
            entry.set_comment(comment.to_string());
        }
        if let Some(epic) = &self.options.epic {
            entry.set_epic(epic.to_string());
        }

        let wallet = Wallet::new(self.options.get_wallet_path());
        let added = wallet.add(entry, self.options.force);
        // println!("Added: {}", added);
        println!("Added: {}", added.to_string());
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
