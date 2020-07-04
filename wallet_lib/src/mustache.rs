
/// https://github.com/nickel-org/rust-mustache

use std::convert::From;
use std::fs::File;
use std::include_bytes;
use mustache::{MapBuilder, VecBuilder, compile_str};
use chrono::{Local, DateTime};
use serde::Serialize;
use crate::wallet::FilterResult;
use crate::wallet::{YearSummary, CategorySummary, EpicSummary};
use crate::wallet::Year;
use crate::number::Number;
use crate::number::ToDisplay;
use crate::number::HtmlDisplay;

const APP_NAME: &'static str = "WalletRust";
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");

type MustacheYears = Vec<MustacheYear>;
type UnsortedMustacheCategories = Vec<MustacheCategory>;
type UnsortedMustacheEpics = Vec<MustacheEpic>;

pub trait HtmlAble {
    fn get_balance(&self) -> Number;

    fn get_balance_class(&self) -> String {
        self.get_balance().html_class()
    }
}

#[derive(Debug, Serialize)]
struct MustacheCategory {
    name: String,
    //revenue: String,
    //expense: String,
    balance: String,
    balance_class: String,
    balance_percent: String,
    is_placeholder: bool,
}

impl MustacheCategory {
    fn new() -> Self {
        Self {
            name: String::new(),
            //revenue: format!("{}", cat_sum.revenue.to_display()),
            //expense: format!("{}", cat_sum.expense.to_display()),
            balance: String::new(),
            balance_class: String::new(),
            balance_percent: String::new(),
            is_placeholder: false,
        }
    }
}

impl From<&CategorySummary> for MustacheCategory {
    fn from(cat_sum: &CategorySummary) -> Self {
        //println!("-> MustacheCategory::from() -> {:?}", cat_sum);
        Self {
            name: cat_sum.name.clone(),
            //revenue: format!("{}", cat_sum.revenue.to_display()),
            //expense: format!("{}", cat_sum.expense.to_display()),
            balance: format!("{}", cat_sum.balance.to_display()),
            balance_class: cat_sum.get_balance_class(),
            balance_percent: format!("{:.2}", cat_sum.balance_percent),
            is_placeholder: false,
        }
    }
}

#[derive(Debug, Serialize)]
struct MustacheEpic {
    name: String,
    handle: String,
    balance: String,
    balance_class: String,
}

// impl MustacheEpic {
//     fn new() -> Self {
//         Self {
//             name: String::new(),
//             handle: String::new(),
//             balance: String::new(),
//             balance_class: String::new(),
//         }
//     }
// }

impl From<&EpicSummary> for MustacheEpic {
    fn from(epic_sum: &EpicSummary) -> Self {
        println!("-> epic_sum: {:?}", epic_sum);
        Self {
            name: epic_sum.name.clone(),
            handle: epic_sum.handle.clone(),
            balance: format!("{}", epic_sum.balance.to_display()),
            //balance_class: epic_sum.balance.get_balance_class(),
            balance_class: String::new(),
        }
    }
}

#[derive(Debug, Serialize)]
struct MustacheYear {
    index: u32, // Minimum: 1900
    year: Year,
    revenue: String,
    expense: String,
    balance: String,
    balance_class: String,
    balance_sum: String,
    balance_sum_class: String,
    categories: UnsortedMustacheCategories
}

impl From<&YearSummary> for MustacheYear {
    fn from(year_sum: &YearSummary) -> Self {
        //println!("-> MustacheYear::from()");

        Self {
            index: 0,
            year: year_sum.year,
            revenue: format!("{}", year_sum.revenue.to_display()),
            expense: format!("{}", year_sum.expense.to_display()),
            balance: format!("{}", year_sum.balance.to_display()),
            balance_class: year_sum.get_balance_class(),
            balance_sum: String::new(),
            balance_sum_class: String::new(),
            categories: UnsortedMustacheCategories::new(),
        }
    }
}

#[derive(Debug, Serialize)]
struct MustacheTotal {
    label: String,

    revenue: String,
    revenue_percent: String,

    expense: String,
    expense_percent: String,

    balance: String,
    balance_class: String,

    //has_categories: bool,
    //categories: UnsortedMustacheCategories,

    //has_epics: bool,
    //epics: MustacheEpics,
}

impl MustacheTotal {
    fn new() -> Self {
        Self {
            label: "TOTAL".to_string(),

            revenue: String::new(),
            revenue_percent: String::new(),

            expense: String::new(),
            expense_percent: String::new(),

            balance: String::new(),
            balance_class: String::new(),

            //has_categories: false,
            //categories: UnsortedMustacheCategories::new(),

            //has_epics: false,
            //epics: MustacheEpics::new(),
        }
    }
}

pub struct IndexMustacheFile {
    path: String,
}

impl IndexMustacheFile {
    /// New Index Mustache file.
    pub fn new(path: String) -> Self {
        Self {
            path,
        }
    }

    /// Render file.
    pub fn render(&self, _result: &FilterResult) {
        println!("-> MustacheFile::render()");

        // Now
        let now: DateTime<Local> = Local::now();

        // Current working directory.
        // let cwd = current_dir().expect("Cannot get current dir");

        // Template Source
        let bytes = include_bytes!("../../resources/views/index.mustache");
        let raw = String::from_utf8_lossy(bytes);

        // Compile Template
        let template = compile_str(&raw).unwrap();

        //println!("-> IndexMustacheFile::render() File::create");
        let mut _file = match File::create(&self.path) {
            Ok(file) => file,
            Err(why) => panic!("Cannot create {}: {}", self.path, why),
        };

        // Total
        let mut index: u32 = 0;
        let mut revenue_sum = Number::new();
        let mut expense_sum = Number::new();
        let mut balance_sum = Number::new();

        // Index Total
        let mut index_total = MustacheTotal::new();

        // Mustache Years
        let mut _myears: MustacheYears = _result.years.values()
            .map(|year_sum| {
                index += 1;
                println!("-> Mustache Year: {:?} #{}", year_sum.year, index);

                revenue_sum += year_sum.revenue;
                expense_sum += year_sum.expense;
                balance_sum += year_sum.balance;
                //println!("  -> balance_sum: {:.2}", balance_sum.to_display());

                let mut _myear = MustacheYear::from(year_sum);
                _myear.index = index;
                _myear.balance_sum = format!("{}", balance_sum.to_display());
                _myear.balance_sum_class = balance_sum.html_class();

                // Iterate over all common categories. Add Categories to Year.
                for category_name in _result.categories.keys() {
                    //println!("  -> category: {:?}", category_sum);

                    // Search common category in Year Categories.
                    if let Some(_ycategory) = year_sum.categories.get(category_name) {
                        _myear.categories.push(MustacheCategory::from(_ycategory));
                    } else {
                        // Placeholder Category
                        let mut _cplaceholder = MustacheCategory::new();
                        _cplaceholder.is_placeholder = true;
                        _myear.categories.push(_cplaceholder);
                    }
                }

                // Return Mustache Year.
                _myear
            })
            .collect();
        println!("-> _myears len: {}", _myears.len());

        // Unsorted Mustache Categories
        let mut _mcategories: UnsortedMustacheCategories = _result.categories.values()
            .map(|category_sum| {
                println!("-> Mustache Category => {}", category_sum.name);

                let _mcategory = MustacheCategory::from(category_sum);

                // Return Mustache Category.
                _mcategory
            })
            .collect();
        let _mcategories_len = _mcategories.len();
        println!("-> _mcategories len: {}", _mcategories_len);
        println!("-> _mcategories: {:?}", _mcategories);

        // Epics
        let _mepics: UnsortedMustacheEpics = _result.epics.values()
            .map(|epic_sum| {
                println!("-> Mustache Epic => {}", epic_sum.name);

                let _mepic = MustacheEpic::from(epic_sum);

                // Return Mustache Epic.
                _mepic
            })
            .collect();
        let _mepics_len = _mepics.len();
        println!("-> _mepics len: {}", _mepics_len);
        println!("-> has _mepics: {}", _mepics_len > 0);
        println!("-> _mepics: {:?}", _mepics);

        // Build Years
        let f_years = move |mut builder: VecBuilder| {
            // let mut balance_sum = Number::new();

            for y in &_myears {
                //println!("-> year {:?}", y.year);
                builder = builder.push(&y).unwrap();
            }
            builder
        };

        // Build Categories
        let f_categories = move |mut builder: VecBuilder| {
            for c in &_mcategories {
                //println!("-> category {:?}", c);
                builder = builder.push(&c).unwrap();
            }
            builder
        };

        // Build Epics
        let f_epics = move |mut builder: VecBuilder| {
            for e in &_mepics {
                println!("-> epic {:?}", e);
                builder = builder.push(&e).unwrap();
            }
            builder
        };

        // Percentages
        let total_volume = revenue_sum.unwrap() + expense_sum.unwrap().abs();
        let revenue_percent = revenue_sum.unwrap() / total_volume * 100.0;
        let expense_percent = expense_sum.unwrap() / total_volume * 100.0;

        // Index Total
        //let mut index_total = MustacheTotal::new();
        index_total.revenue = format!("{}", revenue_sum.to_display());
        index_total.revenue_percent = format!("{:.2}", revenue_percent);
        index_total.expense = format!("{}", expense_sum.to_display());
        index_total.expense_percent = format!("{:.2}", expense_percent);
        index_total.balance = format!("{}", balance_sum.to_display());
        index_total.balance_class = balance_sum.html_class();
        //index_total.has_categories = category_count > 0;
        //index_total.categories = _mcategories;

        // Mustache Builder
        let mut builder = MapBuilder::new()
            .insert_str("PROJECT_NAME", APP_NAME)
            .insert_str("PROJECT_VERSION_FULL", APP_VERSION)
            .insert_str("PROJECT_HOMEPAGE_URL", APP_HOMEPAGE)

            .insert_str("generated_at", now.format("%F %T %z").to_string())
            .insert_str("css_relative_path", ".")
            .insert_str("relative_path", ".")

            .insert_vec("years", f_years)

            .insert_vec("categories", f_categories)
            .insert_str("category_count", _mcategories_len.to_string())

            .insert_vec("epics", f_epics)
            .insert_str("epic_count", _mepics_len.to_string())
        ;

        builder = builder.insert("total", &index_total)
            .expect("Cannot add 'total' field to builder");

        builder = builder.insert("has_categories", &(_mcategories_len > 0))
            .expect("Cannot add 'has_categories' field to builder");

        builder = builder.insert("has_epics", &(_mepics_len > 0))
            .expect("Cannot add 'has_epics' field to builder");

        let data = builder.build();

        println!("-> render_data");
        template.render_data(&mut _file, &data)
            .expect("Failed to render");
    }
}

#[cfg(test)]
mod tests_index_mustache_file {
    use super::IndexMustacheFile;
    use crate::wallet::FilterResult;
    use std::fs::create_dir_all;

    fn setup() {
        create_dir_all("../tmp/tests/mustache")
            .expect("Cannot create mustache test directory.");
    }

    #[test]
    fn test_index_mustache_file1() {
        setup();

        let r1 = FilterResult::new();

        let f1 = IndexMustacheFile::new("../tmp/tests/mustache/index.html".to_string());
        f1.render(&r1);
    }
}
