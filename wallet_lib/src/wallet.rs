
use std::convert::From;
use std::path::PathBuf;
use std::fs::create_dir_all;
use glob::glob;
use std::io::Write;
use std::fmt;
use std::fmt::{Display, Formatter, Result as FmtRes};
use std::vec::Vec;
use std::cmp::Ordering;
use crate::entry::Entry;
use crate::epic::Epic;
use crate::yaml::YamlFile;
use crate::date::Date;
use crate::command::CommandOptions;

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

#[derive(Debug)]
pub struct FilterOptions {
    pub date: Option<Date>,
    pub filter_revenue: Option<bool>,
    pub filter_expense: Option<bool>,
    pub epic: Option<String>,
}

impl FilterOptions {
    pub fn new() -> Self {
        FilterOptions {
            date: None,
            filter_revenue: None,
            filter_expense: None,
            epic: None,
        }
    }
}

impl From<CommandOptions> for FilterOptions {
    fn from(options: CommandOptions) -> FilterOptions {
        println!("-> FilterOptions::from()");

        let mut foptions = FilterOptions::new();
        foptions.date = options.date;
        foptions.filter_revenue = options.filter_revenue;
        foptions.filter_expense = options.filter_expense;
        foptions.epic = options.epic;

        foptions
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

    pub fn add(&self, entry: Entry, force: bool) -> AddResult {
        println!("-> Wallet::add(f={:?})", force);
        println!("-> entry {:?}", entry);

        // Index
        let mut index_file = YamlFile::open_index(self.index_file.clone());

        if index_file.exists(entry.id()) {
            if ! force {
                return AddResult::ExistsInIndex;
            }
        } else {
            index_file.add(entry.id());
        }

        // Epics
        let mut epics_file = YamlFile::open_epics(self.epics_file.clone());
        if epics_file.exists(entry.epic()) {
            // println!("-> epic exist");
        } else {
            // println!("-> NO epic");
            let mut epic = Epic::new();
            epic.set_handle(entry.epic());
            epics_file.add(epic);
        }

        // Month file
        let month_file_name = format!("month_{}.yml", entry.date().fym("_"));
        // println!("-> month_file_name: {:?}", month_file_name);

        let month_file_path = self.data_dir.join(month_file_name.clone());
        // println!("-> month_file_path: {:?}", month_file_path);

        let mut month_file = YamlFile::open_month(month_file_path);
        month_file.add(entry);

        AddResult::Added(AddedResult {
            month_file_name,
        })
    }

    pub fn add_epic(&self, epic: Epic) -> bool {
        let mut epics_file = YamlFile::open_epics(self.epics_file.clone());

        if epics_file.exists(epic.handle()) {
            false
        } else {
            println!("-> NO epic");
            epics_file.add(epic);
            true
        }
    }

    // TODO
    pub fn filter(&self, options: FilterOptions) -> Vec<Entry> {
        println!("-> Wallet::filter()");
        println!("-> options: {:?}", options);

        // Result
        let mut all_items: Vec<Entry> = vec![];

        let data_dir = self.data_dir.join("month_");

        if let Some(path) = data_dir.to_str() {
            let mut g = String::from(path);

            // Filter Date
            if let Some(date) = options.date {
                if date.has_year() && date.has_month() {
                    g.push_str(&date.rym());
                } else if date.has_year() {
                    g.push_str(&date.year().to_string());
                    g.push_str("_*");
                } else if date.has_month() {
                    g.push_str(&date.rym());
                } else if date.has_day() {
                    g.push_str(&date.rym());
                } else {
                    g.push_str("*");
                }
            } else {
                g.push_str("*");
            }

            g.push_str(".yml");

            // Get files.
            let entries = glob(&g).expect("Failed to read glob pattern");
            for entry in entries {
                match entry {
                    Ok(path) => {
                        println!("-> path: {:?}", path.display());
                        println!("-> path: {:?}", path);

                        let month_file = YamlFile::open_month(path);
                        let mut month_items: Vec<Entry> = month_file.get();
                        println!("-> month_items: {:?}", month_items);

                        all_items.append(&mut month_items);
                    },
                    _ => (),
                }
            }
        }

        // Filter
        let filter = all_items.iter().filter(|entry| -> bool {
            println!("-> filter: {:?}", entry);

            // Date
            if let Some(odate) = options.date {
                println!("-> odate: {:?} -> {:?}", odate, odate.to_string());

                let edate = entry.date();
                println!("-> edate: {:?} -> {:?}", edate, edate.to_string());
                println!("-> odate == edate: {:?}", odate == edate);

                if odate.has_day() && odate == edate {
                    println!("-> filter year-month-day");
                } else if odate.has_month() && odate.year() == edate.year() && odate.month() == edate.month() {
                    println!("-> filter year-month");
                } else if odate.has_year() && odate.year() == edate.year() {
                    println!("-> filter year");
                } else {
                    println!("-> date filter failed");
                    return false;
                }
            }

            // Revenue
            if let Some(filter_revenue) = options.filter_revenue {
                println!("-> filter_revenue: {:?}", filter_revenue);

                if filter_revenue && !entry.has_revenue() {
                    return false;
                }
            }

            // Expense
            if let Some(filter_expense) = options.filter_expense {
                println!("-> filter_expense: {:?}", filter_expense);

                if filter_expense && !entry.has_expense() {
                    return false;
                }
            }

            // Epic
            if let Some(epic) = &options.epic {
                println!("-> epic: {:?}", epic);

                if &entry.epic() != epic {
                    return false;
                }
            }

            true
        });


        let mut filtered_items: Vec<&Entry> = filter.collect();
        println!("-> filtered_items: {:?}", filtered_items.len());

        filtered_items.sort_by(|a, b| a.date().to_string().cmp(&b.date().to_string()));

        let items = filtered_items.into_iter().map(|item| item.clone()).collect();

        items
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
    use std::string::ToString;
    use super::{Wallet, AddResult, AddedResult};
    use crate::entry::Entry;
    use crate::epic::Epic;
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
    fn test_wallet_add_entry() {
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

    #[test]
    fn test_wallet_add_epic() {
        let mut e1 = Epic::new();
        e1.set_handle("h1".to_string());
        e1.set_title("t1".to_string());
        e1.set_bgcolor("#ff0000".to_string());

        let w1 = Wallet::new(String::from("../tmp/tests/wallet3"));
        assert!(w1.add_epic(e1));
    }
}
