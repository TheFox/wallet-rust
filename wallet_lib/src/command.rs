
// #[derive(Clone, Copy)]
#[derive(Debug)]
pub struct CommandOptions {
    wallet_path: String,
}

impl CommandOptions {
    pub fn new() -> Self {
        CommandOptions {
            wallet_path: String::new(),
        }
    }

    fn wallet_path(&self) -> String {
        println!("-> CommandOptions::wallet_path()");

        // self.wallet_path.clone()
        String::new() // TODO
    }
}

trait Command {
    fn exec(&self);
}

struct AddCommand {
    options: CommandOptions,
}

impl Command for AddCommand {
    fn exec(&self) {
        println!("-> AddCommand::exec()");

        // let _wallet = Wallet::new_with_path(self.options.wallet_path());
        // let _entry = Entry::new();

        // _wallet.add(_entry);
    }
}
