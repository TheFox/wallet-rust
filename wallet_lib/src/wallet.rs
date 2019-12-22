
use std::fs::create_dir_all;
use std::path::PathBuf;
use crate::entry::Entry;

#[derive(Debug)]
pub struct Wallet {
    path: PathBuf,
    data_dir: PathBuf,
    html_dir: PathBuf,
    tmp_dir: PathBuf,
}

impl Wallet {
    pub fn new(path: String) -> Self {
        println!("-> Wallet::new({})", path);

        // let basedir = PathBuf::new(path.clone());

        let mut basedir = PathBuf::new();
        basedir.push(path);

        let mut data_dir = basedir.clone();
        data_dir.push("data");

        let mut html_dir = basedir.clone();
        html_dir.push("html");

        let mut tmp_dir = basedir.clone();
        tmp_dir.push("tmp");

        println!("-> basedir  {:?}", basedir);
        println!("-> data_dir {:?}", data_dir);
        println!("-> html_dir {:?}", html_dir);
        println!("-> tmp_dir  {:?}", tmp_dir);

        let _w = Wallet {
            path: basedir,
            data_dir: data_dir,
            html_dir: html_dir,
            tmp_dir: tmp_dir,
        };
        _w.init();

        _w
    }

    pub fn init(&self) {
        println!("-> Wallet::init()");
        self.create_dirs();
    }

    fn create_dirs(&self) {
        println!("-> Wallet::create_dirs()");

        create_dir_all(&self.path).expect("Cannot create base path.");
        create_dir_all(&self.data_dir).expect("Cannot create data directory.");
        create_dir_all(&self.html_dir).expect("Cannot create html directory.");
        create_dir_all(&self.tmp_dir).expect("Cannot create tmp directory.");
    }

    // TODO
    pub fn add(&self, entry: Entry) {
        println!("-> Wallet::add()");
        println!("-> entry {:?}", entry);
    }

    // TODO
    pub fn list(&self) {
        println!("-> Wallet::list()");
    }

    // TODO
    pub fn html(&self) {
        println!("-> Wallet::html()");
    }
}

#[cfg(test)]
mod tests {
    use super::Wallet;
    // use std::fs;
    use std::path::Path;

    #[test]
    fn test_new_wallet() {
        let w1 = Wallet::new(String::from("../tmp/tests/wallet1"));
        assert!(Path::new("../tmp/tests/wallet1").exists());
        assert!(Path::new("../tmp/tests/wallet1/data").exists());
        assert!(Path::new("../tmp/tests/wallet1/html").exists());
        assert!(Path::new("../tmp/tests/wallet1/tmp").exists());
    }

    // #[test]
    // fn test_wallet_add() {
    //     let w1 = Wallet::new(String::from("../tmp/tests/wallet2"));
    // }
}
