
use std::path::PathBuf;
use std::fs::create_dir_all;
use std::io::Write;
use std::fmt;
use std::fmt::{Display, Formatter, Result as FmtRes};
use std::vec::Vec;
use crate::entry::Entry;
use crate::yaml::YamlFile;
// use crate::yaml::ToYaml;

#[derive(Debug)]
pub struct Wallet {
    path: PathBuf,
    data_dir: PathBuf,
    html_dir: PathBuf,
    tmp_dir: PathBuf,
    index_file: PathBuf,
    epics_file: PathBuf,
}

pub struct AddedResult {
    month_file_name: String,
}

pub enum AddResult {
    ExistsInIndex,
    Added(AddedResult),
}

impl Display for AddResult {
    fn fmt(&self, f: &mut Formatter) -> FmtRes {
        write!(f, "{}", match self {
            AddResult::Added(_) => "Yes",
            _ => "No",
        })
    }
}

impl Wallet {
    pub fn new(path: String) -> Self {
        println!("-> Wallet::new({})", path);

        let mut basedir = PathBuf::new();
        basedir.push(path);

        let data_dir = basedir.join("data");
        let html_dir = basedir.join("html");
        let tmp_dir = basedir.join("tmp");
        let index_file = data_dir.join("index.yml");
        let epics_file = data_dir.join("epics.yml");

        println!("-> basedir  {:?}", basedir);
        println!("-> data_dir {:?}", data_dir);
        println!("-> html_dir {:?}", html_dir);
        println!("-> tmp_dir  {:?}", tmp_dir);
        println!("-> index_file {:?}", index_file);
        println!("-> epics_file {:?}", epics_file);

        let _w = Wallet {
            path: basedir,
            data_dir,
            html_dir,
            tmp_dir,
            index_file,
            epics_file,
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
    pub fn add(&self, entry: Entry, force: bool) -> AddResult {
        println!("-> Wallet::add(f={:?})", force);
        println!("-> entry {:?}", entry);

        // Index
        let mut index_file = YamlFile::open_index(self.index_file.clone());
        println!("-> exists: {:?}", index_file.exists(entry.id()));

        if index_file.exists(entry.id()) {
            if ! force {
                return AddResult::ExistsInIndex;
            }
        } else {
            index_file.add(entry.id());
        }

        // TODO
        // Epics
        // let mut epics_file = YamlFile::open_epics(self.epics_file.clone());

        // Month file
        let month_file_name = format!("month_{}.yml", entry.date().fym("_"));
        println!("-> month_file_name: {:?}", month_file_name);

        let month_file_path = self.data_dir.join(month_file_name.clone());
        println!("-> month_file_path: {:?}", month_file_path);

        let mut month_file = YamlFile::open_month(month_file_path);
        month_file.add(entry);

        AddResult::Added(AddedResult {
            month_file_name,
        })
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
    use std::path::Path;
    use std::str::FromStr;
    use super::{Wallet, AddResult, AddedResult};
    use crate::entry::Entry;
    use crate::date::Date;

    #[test]
    fn test_new_wallet() {
        let w1 = Wallet::new(String::from("../tmp/tests/wallet1"));
        assert!(Path::new("../tmp/tests/wallet1").exists());
        assert!(Path::new("../tmp/tests/wallet1/data").exists());
        assert!(Path::new("../tmp/tests/wallet1/html").exists());
        assert!(Path::new("../tmp/tests/wallet1/tmp").exists());
    }

    #[test]
    fn test_wallet_add() {
        let d1 = Date::from_str("1987-02-21").unwrap();

        let mut e1 = Entry::new();
        e1.set_date(d1);

        let w1 = Wallet::new(String::from("../tmp/tests/wallet2"));
        assert!(match w1.add(e1, false) {
            AddResult::Added(res) => {
                match res {
                    AddedResult { month_file_name } => {
                        assert_eq!("month_1987_02.yml", month_file_name);
                        true
                    },
                    _ => false,
                }
            },
            _ => false,
        });
    }
}
