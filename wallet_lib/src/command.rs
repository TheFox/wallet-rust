
use crate::wallet::Wallet;
use crate::entry::Entry;
use crate::types::Number;

/// Command options hold all available options for ALL commands.
/// Not all commands will us all options.
#[derive(Debug)]
pub struct CommandOptions {
    pub wallet_path: String,
    pub revenue: Number,
    pub expense: Number,
}

/// Common Options for commands.
impl CommandOptions {
    /// Create a new CommandOptions object.
    pub fn new() -> Self {
        CommandOptions {
            wallet_path: String::new(),
            revenue: 0.0,
            expense: 0.0,
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
            _ => println!("-> unknown kind: {:?}", self.kind),
            // _ => unreachable!(),
        }
    }

    fn exec_init(&self) {
        println!("-> Command::exec_init()");
        Wallet::new(self.options.get_wallet_path());
    }

    fn exec_add(&self) {
        println!("-> Command::exec_add()");

        // TODO
        let mut entry = Entry::new(); // TODO: use from() here
        entry.set_revenue(self.options.revenue);
        entry.set_expense(self.options.expense);

        let wallet = Wallet::new(self.options.get_wallet_path());
        wallet.add(entry);
    }

    fn exec_list(&self) {
        println!("-> Command::exec_list()");

        let wallet = Wallet::new(self.options.get_wallet_path());
        wallet.list(); // TODO
    }

    fn exec_html(&self) {
        println!("-> Command::exec_html()");

        let wallet = Wallet::new(self.options.get_wallet_path());
        wallet.html(); // TODO
    }
}
