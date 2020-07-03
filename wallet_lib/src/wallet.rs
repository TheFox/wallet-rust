
use std::convert::From;
// use std::env::current_dir;
use std::path::PathBuf;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::include_bytes;
use glob::glob;
use std::fmt::{Display, Formatter, Result as FmtRes};
use std::vec::Vec;
use std::collections::BTreeMap;
use std::rc::Rc;
// use serde::Serialize;
use crate::entry::Entry;
use crate::epic::Epic;
use crate::yaml::YamlFile;
use crate::date::Date;
use crate::command::CommandOptions;
use crate::mustache::{IndexMustacheFile};
use crate::number::{NumberType, Number};

pub type Year = i32;
pub type Month = u32;
pub type Day = u32;
pub type EntryRc = Rc<Entry>;
pub type Entries = Vec<EntryRc>;
pub type EntryRef<'a> = &'a Entry;
pub type EntriesRef<'a> = Vec<EntryRef<'a>>;
pub type CategorySet = Vec<CategorySummary>;
pub type SortedCategories = BTreeMap<String, CategorySummary>;
pub type Epics = BTreeMap<String, EpicSummary>;
pub type Days = BTreeMap<Day, DaySummary>;
pub type Months = BTreeMap<Month, MonthSummary>;
pub type Years = BTreeMap<Year, YearSummary>;

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

trait AddEntry {
    fn add(&mut self, entry_ref: EntryRc);
}

#[derive(Debug)]
pub struct CategorySummary {
    pub name: String,
    pub revenue: Number,
    pub expense: Number,
    pub balance: Number,
    pub balance_percent: NumberType,
}

impl CategorySummary {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            revenue: Number::new(),
            expense: Number::new(),
            balance: Number::new(),
            balance_percent: 0.0
        }
    }
}
impl AddEntry for CategorySummary {
    fn add(&mut self, entry_ref: EntryRc) {
        //println!("-> CategorySummary::add()");

        // Calc
        self.revenue += entry_ref.revenue();
        self.expense += entry_ref.expense();
        self.balance += entry_ref.balance();
    }
}

#[derive(Debug)]
pub struct EpicSummary {
    pub name: String, // TODO: set epic name
    pub revenue: Number,
    pub expense: Number,
    pub balance: Number,
}

impl EpicSummary {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            revenue: Number::new(),
            expense: Number::new(),
            balance: Number::new(),
        }
    }
}

impl AddEntry for EpicSummary {
    fn add(&mut self, entry_ref: EntryRc) {
        // println!("-> EpicSummary::add()");

        // Calc
        self.revenue += entry_ref.revenue();
        self.expense += entry_ref.expense();
        self.balance += entry_ref.balance();
    }
}

#[derive(Debug)]
pub struct DaySummary {
    pub revenue: Number,
    pub expense: Number,
    pub balance: Number,
}

impl DaySummary {
    pub fn new() -> Self {
        Self {
            revenue: Number::new(),
            expense: Number::new(),
            balance: Number::new(),
        }
    }
}

impl AddEntry for DaySummary {
    fn add(&mut self, entry_ref: EntryRc) {
        // println!("-> DaySummary::add()");

        // Calc
        self.revenue += entry_ref.revenue();
        self.expense += entry_ref.expense();
        self.balance += entry_ref.balance();
    }
}

#[derive(Debug)]
pub struct MonthSummary {
    pub entries: Entries,
    pub days: Days,
    pub categories: SortedCategories,
    pub epics: Epics,

    pub revenue: Number,
    pub expense: Number,
    pub balance: Number,
}

impl MonthSummary {
    pub fn new() -> Self {
        Self {
            entries: Entries::new(),
            days: Days::new(),
            categories: SortedCategories::new(),
            epics: Epics::new(),

            revenue: Number::new(),
            expense: Number::new(),
            balance: Number::new(),
        }
    }
}

impl AddEntry for MonthSummary {
    fn add(&mut self, entry_ref: EntryRc) {
        // println!("-> MonthSummary::add()");

        // Calc
        self.revenue += entry_ref.revenue();
        self.expense += entry_ref.expense();
        self.balance += entry_ref.balance();

        let date = entry_ref.date();
        let day = date.day();
        let category = entry_ref.category();
        let epic = entry_ref.epic();

        // println!("  -> day: {:?}", day);

        // Days
        match self.days.get_mut(&day) {
            Some(day_summary) => {
                // println!("  -> old day_summary: {:?}", day_summary);

                day_summary.add(entry_ref.clone());
            },
            None => {
                // println!("  -> new day_summary");

                let mut day_summary = DaySummary::new();
                day_summary.add(entry_ref.clone());

                self.days.insert(day, day_summary);
            }
        }

        // Categories
        match self.categories.get_mut(&category) {
            Some(category_summary) => {
                //println!("  -> old category_summary");

                category_summary.add(entry_ref.clone());
            },
            None => {
                //println!("  -> new category_summary");

                let mut category_summary = CategorySummary::new();
                //category_summary.name =
                category_summary.add(entry_ref.clone());

                self.categories.insert(category, category_summary);
            }
        }

        // Epics
        match self.epics.get_mut(&epic) {
            Some(epic_summary) => {
                // println!("  -> old epic_summary");

                epic_summary.add(entry_ref.clone());
            },
            None => {
                // println!("  -> new epic_summary");

                let mut epic_summary = EpicSummary::new();
                epic_summary.add(entry_ref.clone());

                self.epics.insert(epic, epic_summary);
            }
        }

        self.entries.push(entry_ref);
    }
}

#[derive(Debug)]
pub struct YearSummary {
    pub year: Year,

    pub entries: Entries,
    pub months: Months,
    pub categories: SortedCategories,
    pub epics: Epics,

    pub revenue: Number,
    pub expense: Number,
    pub balance: Number,
    // pub balance_sum: Number,
}

impl YearSummary {
    pub fn new(year: Year) -> Self {
        Self {
            year,

            entries: Entries::new(),
            months: Months::new(),
            categories: SortedCategories::new(),
            epics: Epics::new(),

            revenue: Number::new(),
            expense: Number::new(),
            balance: Number::new(),
        }
    }

    /*pub fn get_balance_class(&self) -> String {
        if self.balance.is_negative() {
            "red"
        } else {
            ""
        }.into()
    }*/
}

impl AddEntry for YearSummary {
    fn add(&mut self, entry_ref: EntryRc) {
        // println!("-> YearSummary::add()");

        // Calc
        self.revenue += entry_ref.revenue();
        self.expense += entry_ref.expense();
        self.balance += entry_ref.balance();

        let date = entry_ref.date();
        let month = date.month();
        let category = entry_ref.category();
        let epic = entry_ref.epic();

        // println!("  -> month: {:?}", month);

        // Month
        match self.months.get_mut(&month) {
            Some(month_summary) => {
                // println!("  -> old month_summary");

                month_summary.add(entry_ref.clone());
            },
            None => {
                // println!("  -> new month_summary");

                let mut month_summary = MonthSummary::new();
                month_summary.add(entry_ref.clone());

                self.months.insert(month, month_summary);
            }
        }

        // Categories
        match self.categories.get_mut(&category) {
            Some(category_summary) => {
                // println!("  -> old category_summary");

                category_summary.add(entry_ref.clone());
            },
            None => {
                // println!("  -> new category_summary");

                let mut category_summary = CategorySummary::new();
                category_summary.name = category.clone();
                category_summary.add(entry_ref.clone());

                self.categories.insert(category, category_summary);
            }
        }

        // Epics
        match self.epics.get_mut(&epic) {
            Some(epic_summary) => {
                // println!("  -> old epic_summary");

                epic_summary.add(entry_ref.clone());
            },
            None => {
                // println!("  -> new epic_summary");

                let mut epic_summary = EpicSummary::new();
                epic_summary.add(entry_ref.clone());

                self.epics.insert(epic, epic_summary);
            }
        }

        self.entries.push(entry_ref);
    }
}

#[derive(Debug)]
pub struct FilterResult {
    pub entries: Entries,
    pub years: Years,
    pub categories: SortedCategories,
    //pub categories_balance: Number,
    //pub categories_volume: Number,
    pub epics: Epics,

    pub revenue: Number,
    pub expense: Number,
    pub balance: Number,
}

// https://stackoverflow.com/questions/32682876/is-there-any-way-to-return-a-reference-to-a-variable-created-in-a-function
impl FilterResult {
    pub fn new() -> Self {
        FilterResult {
            entries: Entries::new(),
            years: Years::new(),
            categories: SortedCategories::new(),
            //categories_balance: Number::new(),
            //categories_volume: Number::new(),
            epics: Epics::new(),

            revenue: Number::new(),
            expense: Number::new(),
            balance: Number::new(),
        }
    }

    pub fn add(&mut self, entry: Entry) {
        //println!("-> FilterResult::add()");

        let date = entry.date();
        let year = date.year();
        let category = entry.category();
        let epic = entry.epic();

        // println!("  -> year: {:?}", year);

        // Consume entry here.
        let entry_ref = Rc::new(entry);

        // Calc
        self.revenue += entry_ref.revenue();
        self.expense += entry_ref.expense();
        self.balance += entry_ref.balance();
        //self.categories_balance += entry_ref.balance();
        //self.categories_volume += entry_ref.balance().abs();

        // Years
        match self.years.get_mut(&year) {
            Some(year_summary) => {
                // println!("  -> old year_summary");

                year_summary.add(entry_ref.clone());
            },
            None => {
                // println!("  -> new year_summary");

                let mut year_summary = YearSummary::new(year);
                year_summary.add(entry_ref.clone());

                self.years.insert(year, year_summary);
            }
        }

        // Categories
        match self.categories.get_mut(&category) {
            Some(category_summary) => {
                println!("  -> old category_summary");

                category_summary.add(entry_ref.clone());
            },
            None => {
                println!("  -> new category_summary => {:?}", category);

                let mut category_summary = CategorySummary::new();
                category_summary.name = category.clone();
                category_summary.add(entry_ref.clone());

                self.categories.insert(category, category_summary);
            }
        }

        // Epics
        match self.epics.get_mut(&epic) {
            Some(epic_summary) => {
                // println!("  -> old epic_summary");

                epic_summary.add(entry_ref.clone());
            },
            None => {
                // println!("  -> new epic_summary");

                let mut epic_summary = EpicSummary::new();
                epic_summary.add(entry_ref.clone());

                self.epics.insert(epic, epic_summary);
            }
        }

        // Consume entry ref here.
        self.entries.push(entry_ref);
    }

    fn post_calc(&mut self) {
        println!("-> FilterResult::post_calc()");

        let total_category_volume: NumberType = self.categories.values().map(|cs| cs.balance.abs().unwrap()).sum();
        println!("-> total_category_volume: {:?}", total_category_volume);

        // Categories Balance Volume
        for (category_name, category_sum) in &mut self.categories {
            //println!("-> category_sum: {:?} -> {}", category_name, category_sum.balance);
            let _v = category_sum.balance.abs().unwrap();
            category_sum.balance_percent = _v / total_category_volume * 100.0;
            println!("-> category_sum: {:?} -> {:?} {:?}", category_name, category_sum.balance, _v);
        }
    }
}

impl From<EntriesRef<'_>> for FilterResult {
    fn from(entries: EntriesRef) -> Self {
        //println!("-> FilterResult::from() -> {:?}", entries);
        let mut result = FilterResult::new();
        for entry in entries {
            result.add(entry.clone());
        }
        result.post_calc();
        result
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

    pub fn set_html_path(&mut self, path: String) {
        println!("-> Wallet::set_html_path({})", path);
        self.html_dir = path.into();
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
        println!("-> options: {:?}", options);

        // Result
        let mut all_items: Vec<Entry> = vec![];

        let data_dir = self.data_dir.join("month_");

        // Find files.
        println!("-> find files");
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
            println!("-> for path {}, found files: {}", g, glob(&g).unwrap().count());

            for entry in entries {
                match entry {
                    Ok(path) => {
                        println!("-> collect file: {:?}", path);

                        let month_file = YamlFile::open_month(path);
                        let mut month_items: Vec<Entry> = month_file.get();

                        all_items.append(&mut month_items);
                    },
                    _ => (),
                }
            }
        }

        println!("-> entries: {}", all_items.len());

        // Filter
        let filter = all_items.iter().filter(|entry| -> bool {
            //println!("-> entry: {:?}", entry.id());

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

        // Apply filter.
        let entries: EntriesRef = filter.collect();

        // Result
        FilterResult::from(entries)
    }

    /// HTML
    pub fn html(&self, _options: FilterOptions) {
        println!("-> Wallet::html()");

        // Create html directory.
        create_dir_all(&self.html_dir).expect("Cannot create html directory.");

        // let cwd = current_dir().expect("Cannot get current dir");
        // println!("-> cwd: {}", cwd.display());

        // let up = cwd.join("..");
        // println!("-> up: {}", up.display());

        let _result = self.filter(_options);

        // CSS File
        {
            let css_file_path = self.html_dir.join("style.css");
            println!("-> css_file: {}", css_file_path.display());
            let mut css_file = match File::create(&css_file_path) {
                Ok(file) => file,
                Err(why) => panic!("Cannot create {}: {}", css_file_path.display(), why),
            };

            let bytes = include_bytes!("../../resources/css/style.css");
            css_file.write_all(bytes)
                .expect("Cannot write style file.");
        }

        // Index File
        {
            // Path
            let index_file_path = self.html_dir.join("index.html");
            println!("-> index_file: {}", index_file_path.display());

            // File
            let index_file = IndexMustacheFile::new(index_file_path.to_str().unwrap().to_string());
            index_file.render(&_result);
        }
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
