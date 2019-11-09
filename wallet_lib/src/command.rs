
use crate::wallet::Wallet;
use crate::entry::Entry;
use crate::types::Number;

// #[derive(Clone, Copy)]
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

pub trait Command {
    fn exec(&self);
}

pub struct InitCommand {
    pub options: CommandOptions,
}

pub struct AddCommand {
    pub options: CommandOptions,
}

pub struct ListCommand {
    pub options: CommandOptions,
}

pub struct HtmlCommand {
    pub options: CommandOptions,
}

impl Command for InitCommand {
    fn exec(&self) {
        println!("-> InitCommand::exec()");

        Wallet::new(self.options.get_wallet_path());
    }
}

impl Command for AddCommand {
    fn exec(&self) {
        println!("-> AddCommand::exec()");

        // TODO
        let mut entry = Entry::new(); // TODO: use from() here
        entry.set_revenue(self.options.revenue);
        entry.set_expense(self.options.expense);

        let wallet = Wallet::new(self.options.get_wallet_path());
        wallet.add(entry);
    }
}

impl Command for ListCommand {
    fn exec(&self) {
        println!("-> ListCommand::exec()");

        let wallet = Wallet::new(self.options.get_wallet_path());
        wallet.list(); // TODO
    }
}

impl Command for HtmlCommand {
    fn exec(&self) {
        println!("-> HtmlCommand::exec()");

        let wallet = Wallet::new(self.options.get_wallet_path());
        wallet.html(); // TODO
    }
}
