
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

        create_dir_all(self.path.clone()).expect("Cannot create base path.");
        // TODO
        // create_dir_all(self.data_dir.clone());
        // create_dir_all(self.html_dir.clone());
        // create_dir_all(self.tmp_dir.clone());
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
