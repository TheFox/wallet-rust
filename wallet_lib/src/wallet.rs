
#[derive(Debug)]
struct Wallet {
    path: String,
}

impl Wallet {
    fn new() -> Self {
        println!("-> Wallet::new()");

        Wallet {
            path: String::new(), // TODO
        }
    }

    fn new_with_path(path: String) -> Self {
        println!("-> Wallet::new_with_path({})", path);

        let _w = Wallet {
            path: path,
        };
        _w.init();

        _w
    }

    fn init(&self) {
        println!("-> Wallet::init()");
        self.create_dirs();
    }

    fn create_dirs(&self) {
        println!("-> Wallet::create_dirs()");
    }

    // fn add(&self, entry: Entry) {
    //     println!("-> Wallet::add() {:?}", entry);
    // }
}
