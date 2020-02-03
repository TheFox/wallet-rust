
use std::convert::From;
use std::env::current_dir;
use std::path::PathBuf;
use std::fs::create_dir_all;
use glob::glob;
use std::fmt::{Display, Formatter, Result as FmtRes};
use std::vec::Vec;
use std::collections::HashMap;
use crate::entry::Entry;
use crate::epic::Epic;
use crate::yaml::YamlFile;
use crate::date::Date;
use crate::command::CommandOptions;
use crate::mustache::{MustacheFileKind, MustacheFile};
use crate::number::Number;

pub type Year = i32;
pub type Month = u8;
pub type Day = u8;

pub struct AddedResult {
    month_file_name: String,
}

impl AddedResult {
    pub fn new() -> Self {
        AddedResult {
            month_file_name: String::new(),
        }
    }
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
    pub category: Option<String>,
    pub epic: Option<String>,
}

impl FilterOptions {
    pub fn new() -> Self {
        FilterOptions {
            date: None,
            filter_revenue: None,
            filter_expense: None,
            category: None,
            epic: None,
        }
    }
}

impl From<CommandOptions> for FilterOptions {
    fn from(options: CommandOptions) -> FilterOptions {
        let mut foptions = FilterOptions::new();

        foptions.date = options.date;
        foptions.filter_revenue = options.filter_revenue;
        foptions.filter_expense = options.filter_expense;
        foptions.category = options.category;
        foptions.epic = options.epic;

        foptions
    }
}

#[derive(Debug)]
struct DaySummary {
    //pub entries,

    // pub revenue: Number,
    // pub expense: Number,
    // pub balance: Number,

    // TODO: categories, epics
}

#[derive(Debug)]
struct MonthSummary {
    // pub days: HashMap<Day, DaySummary>,

    // pub revenue: Number,
    // pub expense: Number,
    // pub balance: Number,

    // TODO: categories, epics
}

#[derive(Debug)]
pub struct YearSummary<'a> {
    pub entries: Vec<&'a Entry>,
    // pub months: HashMap<Month, MonthSummary>,

    pub revenue: Number,
    // pub expense: Number,
    // pub balance: Number,

    // TODO: categories, epics
}

impl<'a> YearSummary<'a> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),

            revenue: Number::new(),
            // expense: Number::new(),
            // balance: Number::new(),
        }
    }

    pub fn add(&mut self, entry: &'a Entry) {
        println!("-> YearSummary::add()");
        self.entries.push(entry);
    }
}

#[derive(Debug)]
pub struct FilterResult<'a> {
    pub entries: Vec<Entry>,
    pub years: HashMap<Year, YearSummary<'a>>,

    pub revenue: Number,
    // pub expense: Number,
    // pub balance: Number,

    // TODO: categories, epics
}

// https://stackoverflow.com/questions/32682876/is-there-any-way-to-return-a-reference-to-a-variable-created-in-a-function
impl FilterResult<'_> {
    pub fn new() -> Self {
        FilterResult {
            entries: Vec::new(),
            years: HashMap::new(),

            revenue: Number::new(),
            // expense: Number::new(),
            // balance: Number::new(),
        }
    }

    pub fn add(&mut self, entry: Entry) {
        println!("-> FilterResult::add()");
        // println!("-> FilterResult::add({:?})", entry);

        let date = entry.date();
        let year = date.year();
        let month = date.month();
        let day = date.day();

        println!("  -> date: {:?} {:?} {:?}", year, month, day);

        // println!("  -> revenue A: {:?}", self.revenue);
        // println!("  -> expense A: {:?}", self.expense);
        // println!("  -> balance A: {:?}", self.balance);

        self.revenue += entry.revenue();
        // self.expense += entry.expense();
        // self.balance += entry.balance();

        // println!("  -> revenue B: {:?}", self.revenue);
        // println!("  -> expense B: {:?}", self.expense);
        // println!("  -> balance B: {:?}", self.balance);

        // let x: ref YearSummary = self.years.get(&year).unwrap();

        // if let Some(&mut year_summary) = &mut self.years.get(&year) {
        //     year_summary.year += 42;
        // }

        match self.years.get_mut(&year) {
            Some(year_summary) => {
                println!("  -> old year_summary: {:?}", year_summary);
                year_summary.add(&entry);
            },
            // Some(ref year_summary) => {
            //     println!("  -> old year_summary: {:?}", year_summary);
            //     year_summary.year += 42;
            // },
            None => {
                println!("  -> new year_summary");
                let mut year_summary = YearSummary::new();
                self.years.insert(year, year_summary);
            }
        }

        // Consume entry here.
        self.entries.push(entry);
    }
}

#[derive(Debug)]
pub struct Wallet {
    path: PathBuf,
    data_dir: PathBuf,
    html_dir: PathBuf,
    tmp_dir: PathBuf,
    index_file: PathBuf,
    epics_file: PathBuf,
}

impl Wallet {
    /// New Wallet
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

    /// Add Entry
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

    /// Add Epic
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

    /// Retrieve Entries by a set of filters.
    pub fn filter(&self, options: FilterOptions) -> FilterResult {
        println!("-> Wallet::filter()");
        // println!("-> options: {:?}", options);

        // Result
        let mut all_items: Vec<Entry> = vec![];

        let data_dir = self.data_dir.join("month_");

        // Find files.
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
                        // println!("-> path: {:?}", path.display());
                        // println!("-> path: {:?}", path);

                        let month_file = YamlFile::open_month(path);
                        let mut month_items: Vec<Entry> = month_file.get();
                        // println!("-> month_items: {:?}", month_items);

                        all_items.append(&mut month_items);
                    },
                    _ => (),
                }
            }
        }

        // Filter
        let filter = all_items.iter().filter(|entry| -> bool {
            // println!("-> filter: {:?}", entry);

            // Date
            if let Some(odate) = options.date {
                // println!("-> odate: {:?} -> {:?}", odate, odate.to_string());

                let edate = entry.date();
                // println!("-> edate: {:?} -> {:?}", edate, edate.to_string());
                // println!("-> odate == edate: {:?}", odate == edate);

                if odate.has_day() {
                    // println!("-> filter year-month-day");
                    if odate != edate {
                        return false;
                    }
                } else if odate.has_month() && odate.year() == edate.year() && odate.month() == edate.month() {
                    // println!("-> filter year-month");
                } else if odate.has_year() && odate.year() == edate.year() {
                    // println!("-> filter year");
                } else {
                    // println!("-> date filter failed");
                    return false;
                }
            }

            // Revenue
            if let Some(filter_revenue) = options.filter_revenue {
                // println!("-> filter_revenue: {:?}", filter_revenue);

                if filter_revenue && !entry.has_revenue() {
                    return false;
                }
            }

            // Expense
            if let Some(filter_expense) = options.filter_expense {
                // println!("-> filter_expense: {:?}", filter_expense);

                if filter_expense && !entry.has_expense() {
                    return false;
                }
            }

            // Category
            if let Some(category) = &options.category {
                // println!("-> category: {:?}", category);

                if &entry.category() != category {
                    return false;
                }
            }

            // Epic
            if let Some(epic) = &options.epic {
                // println!("-> epic: {:?}", epic);

                if &entry.epic() != epic {
                    return false;
                }
            }

            true
        });

        // Result
        let mut result = FilterResult::new();

        // Apply filter.
        let entries: Vec<&Entry> = filter.collect();

        // Iterate entries.
        for entry in entries {
            // println!("-> entry: {:?}", entry);

            result.add(entry.clone());
        }

        // let mut filtered_items: Vec<&Entry> = filter.collect();
        // filtered_items.sort_by(|a, b| a.date().to_string().cmp(&b.date().to_string()));

        // let items = filtered_items.into_iter().map(|item| item.clone()).collect();
        // items

        result
    }

    /// HTML
    pub fn html(&self, _options: FilterOptions) {
        println!("-> Wallet::html()");

        let cwd = current_dir().expect("Cannot get current dir");
        println!("cwd: {}", cwd.display());

        let up = cwd.join("..");
        println!("up: {}", up.display());

        // let entries = self.filter(_options);
        // for _entry in entries {
        //     println!("-> entry");
        // }

        let index_file_path = self.html_dir.join("index.html");
        println!("index_file: {}", index_file_path.display());
        let index_file = MustacheFile::new(MustacheFileKind::IndexFile, index_file_path.to_str().unwrap().to_string());
        index_file.render();
    }
}

#[cfg(test)]
mod tests_addedresult {
    use super::AddedResult;

    #[test]
    fn test_addedresult_new() {
        AddedResult::new();
    }
}

#[cfg(test)]
mod tests_addresult {
    use super::{AddedResult, AddResult};

    #[test]
    fn test_addresult1() {
        let r1 = AddResult::ExistsInIndex;
        assert_eq!("No", format!("{}", r1));
    }

    #[test]
    fn test_addresult2() {
        let r1 = AddResult::Added(AddedResult::new());
        assert_eq!("Yes", format!("{}", r1));
    }
}

#[cfg(test)]
mod tests_filteroptions_basic {
    use super::FilterOptions;

    #[test]
    fn test_filteroptions1() {
        FilterOptions::new();
    }
}

#[cfg(test)]
mod tests_filteroptions_from {
    use super::FilterOptions;
    use crate::command::CommandOptions;

    #[test]
    fn test_filteroptions_from_commandoptions1() {
        let mut copt1 = CommandOptions::new();
        copt1.category = Some("test1".to_string());

        let fopt1 = FilterOptions::from(copt1);
        if let Some(category) = fopt1.category {
            assert_eq!("test1", category);
        }
        else {
            assert!(false);
        }
    }
}

#[cfg(test)]
mod tests_filterresult_basic {
    use super::FilterResult;
    use crate::entry::Entry;

    #[test]
    fn test_filterresult1() {
        FilterResult::new();
    }

    #[test]
    fn test_filterresult2() {
        let e1 = Entry::from("Hi/2001-01-01/30/0");
        let e2 = Entry::from("Hi/2001-01-02/0/10");
        let e3 = Entry::from("Hi/2001-01-03/0/10");
        let e4 = Entry::from("Hi/2001-02-01/0/10");
        let e5 = Entry::from("Hi/2002-01-01/0/10");

        let mut r1 = FilterResult::new();
        r1.add(e1);
        r1.add(e2);
        r1.add(e3);
        r1.add(e4);
        r1.add(e5);

        // assert_eq!(30.0, r1.revenue.unwrap());
        // assert_eq!(-40.0, r1.expense.unwrap());
        // assert_eq!(-10.0, r1.balance.unwrap());

        // println!("revenue: {:?}", r1.revenue);
        // println!("expense: {:?}", r1.expense);
        // println!("balance: {:?}", r1.balance);
    }
}

#[cfg(test)]
mod tests_wallet_basic {
    use std::path::Path;
    use super::Wallet;

    #[test]
    fn test_wallet_new() {
        Wallet::new("../tmp/tests/wallet1".to_string());

        assert!(Path::new("../tmp/tests/wallet1").exists());
        assert!(Path::new("../tmp/tests/wallet1/data").exists());
        assert!(Path::new("../tmp/tests/wallet1/html").exists());
        assert!(Path::new("../tmp/tests/wallet1/tmp").exists());
    }
}

#[cfg(test)]
mod tests_wallet_entry {
    use std::str::FromStr;
    use super::{Wallet, AddResult, AddedResult};
    use crate::entry::Entry;
    use crate::date::Date;

    #[test]
    fn test_wallet_entry_add() {
        let d1 = Date::from_str("1987-02-21").unwrap();

        let mut e1 = Entry::new();
        e1.set_date(d1);

        let w1 = Wallet::new("../tmp/tests/wallet2".to_string());
        assert!(match w1.add(e1, false) {
            AddResult::Added(res) => {
                match res {
                    AddedResult { month_file_name } => {
                        assert_eq!("month_1987_02.yml", month_file_name);
                        true
                    },
                    // _ => false,
                }
            },
            _ => false,
        });
    }
}

#[cfg(test)]
mod tests_wallet_epic {
    use super::Wallet;
    use crate::epic::Epic;

    #[test]
    fn test_wallet_epic_add() {
        let mut e1 = Epic::new();
        e1.set_handle("h1".to_string());
        e1.set_title("t1".to_string());
        e1.set_bgcolor("#ff0000".to_string());

        let w1 = Wallet::new("../tmp/tests/wallet3".to_string());
        assert!(w1.add_epic(e1));
    }
}
