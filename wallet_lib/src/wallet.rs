
use std::fs::create_dir;

#[derive(Debug)]
pub struct Wallet {
    path: String,
}

impl Wallet {
    pub fn new(path: String) -> Self {
        println!("-> Wallet::new({})", path);

        let _w = Wallet {
            path: path,
        };
        _w.init();

        _w
    }

    pub fn init(&self) -> () {
        println!("-> Wallet::init()");
        self.create_dirs();
    }

    // TODO
    fn create_dirs(&self) {
        println!("-> Wallet::create_dirs() -> {}", self.path);

        // fs::create_dir_all("/some/dir")?;
        // Ok(())
    }

    // TODO
    // pub fn add(&self, entry: Entry) {
    //     println!("-> Wallet::add() {:?}", entry);
    // }
}
