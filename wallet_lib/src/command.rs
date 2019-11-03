
use crate::wallet::Wallet;

// #[derive(Clone, Copy)]
#[derive(Debug)]
pub struct CommandOptions {
    pub wallet_path: String,
}

/// Common Options for commands.
impl CommandOptions {
    /// Create a new CommandOptions object.
    pub fn new() -> Self {
        CommandOptions {
            wallet_path: String::new(),
        }
    }

    fn wallet_path(&self) -> String {
        println!("-> CommandOptions::wallet_path()");

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

        Wallet::new(self.options.wallet_path());
    }
}

impl Command for AddCommand {
    fn exec(&self) {
        println!("-> AddCommand::exec()");

        let wallet = Wallet::new(self.options.wallet_path());
        wallet.add();
    }
}

impl Command for ListCommand {
    fn exec(&self) {
        println!("-> ListCommand::exec()");

        let wallet = Wallet::new(self.options.wallet_path());
        wallet.list();
    }
}

impl Command for HtmlCommand {
    fn exec(&self) {
        println!("-> HtmlCommand::exec()");

        let wallet = Wallet::new(self.options.wallet_path());
        wallet.html();
    }
}
