
/// https://github.com/nickel-org/rust-mustache

use std::convert::From;
use std::fs::File;
use std::include_bytes;
use std::cmp::Eq;
use std::cmp::Ordering;
use mustache::{MapBuilder, VecBuilder, compile_str};
// use mustache::serde;
// use std::env::current_dir;
use chrono::{Local, DateTime};
// use std::collections::HashMap;
use serde::Serialize;
// use std::fmt::Display;
use crate::wallet::FilterResult;
use crate::wallet::{YearSummary, CategorySummary};
use crate::wallet::Year;
use crate::number::Number;
use crate::number::ToDisplay;
use crate::number::HtmlDisplay;

const APP_NAME: &'static str = "WalletRust";
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");

trait HtmlAble {
    fn get_balance(&self) -> Number;

    fn get_balance_class(&self) -> String {
        self.get_balance().html_class()
    }
}

impl HtmlAble for CategorySummary {
    fn get_balance(&self) -> Number {
        //println!("-> CategorySummary::get_balance()");
        self.balance
    }
}

impl HtmlAble for YearSummary {
    fn get_balance(&self) -> Number {
        //println!("-> YearSummary::get_balance()");
        self.balance
    }
}

#[derive(Debug, Serialize)]
struct MustacheCategory {
    name: String,
    //revenue: String,
    //expense: String,
    balance: String,
    balance_class: String,
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
            is_placeholder: false,
        }
    }
}

type MustacheCategories = Vec<MustacheCategory>;

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
    categories: MustacheCategories
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
            categories: MustacheCategories::new(),
        }
    }
}

type MustacheYears = Vec<MustacheYear>;

#[derive(Debug, Serialize)]
struct MustacheTotal {
    label: String,

    revenue: String,
    revenue_percent: String,

    expense: String,
    expense_percent: String,

    balance: String,
    balance_class: String,

    has_categories: bool,
    categories: MustacheCategories,

    has_epics: bool,
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

            has_categories: false,
            categories: MustacheCategories::new(),

            has_epics: false,
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

        let mut index: u32 = 0;
        let mut revenue_sum = Number::new();
        let mut expense_sum = Number::new();
        let mut balance_sum = Number::new();

        // Mustache Years
        let mut _myears: MustacheYears = _result.years.values()
            .map(|year_sum| {
                index += 1;
                //println!("-> Mustache Year: {:?}", year_sum.year);

                revenue_sum += year_sum.revenue;
                expense_sum += year_sum.expense;
                balance_sum += year_sum.balance;
                //println!("  -> balance_sum: {:.2}", balance_sum.to_display());

                let mut _myear = MustacheYear::from(year_sum);
                _myear.index = index;
                _myear.balance_sum = format!("{}", balance_sum.to_display());
                _myear.balance_sum_class = balance_sum.html_class();
                // if balance_sum.is_negative() {
                //     _myear.balance_sum_class = "red".to_string();
                // }

                // Add Categories to Year. Iterate over all common categories.
                for (category_name, category_sum) in &_result.categories {
                    println!("-> year {:?}, category: {:?}", year_sum.year, category_name);

                    // Search common category in Year Categories.
                    if let Some(_ycategory) = year_sum.categories.get(category_name) {
                        //println!("  -> year {:?}, get: {:?}", year_sum.year, _ycategory.name);

                        //let mut _mcategory = MustacheCategory::from(_ycategory);
                        //println!("  -> year {:?}, mcategory: {:?}", year_sum.year, _mcategory);
                        _myear.categories.push(MustacheCategory::from(_ycategory));
                    } else {
                        //println!("  -> year category not found: {}", category_name);

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

        // Mustache Categories
        let mut _mcategories: MustacheCategories = _result.categories.values()
            .map(|category_sum| {
                //println!("-> Mustache Category => {:?}", category_sum);

                let _mcategory = MustacheCategory::from(category_sum);

                // Return Mustache Category.
                _mcategory
            })
            .collect();
        println!("-> _mcategories len: {}", _mcategories.len());

        // Sort Years
        /*_myears.sort_by(|a, b| -> Ordering {
            println!("-> sort Years: {} {} => {:?}", a.year, b.year, a.year.cmp(&b.year));
            a.year.cmp(&b.year)
        });*/

        // Sort Categories
        /*_mcategories.sort_by(|a, b| -> Ordering {
            let mut is_default = a.name == "default" || b.name == "default";
            println!("-> sort Categories: '{}' '{}' {:?}", a.name, b.name, is_default);

            if a.name == "default" {
                Ordering::Less
            } else {
                if b.name == "default" {
                    Ordering::Greater
                } else {
                    a.name.cmp(&b.name)
                }
            }
        });*/

        // Build Years
        let mut f_years = move |mut builder: VecBuilder| {
            // let mut balance_sum = Number::new();

            for y in &_myears {
                //println!("-> year {:?}", y.year);
                builder = builder.push(&y).unwrap();
            }
            builder
        };

        // Build Categories
        let mut f_categories = |mut builder: VecBuilder| {
            for c in &_mcategories {
                //println!("-> category {:?}", c);
                builder = builder.push(&c).unwrap();
            }
            builder
        };

        let total_volume = revenue_sum.unwrap() + expense_sum.unwrap().abs();
        let revenue_percent = revenue_sum.unwrap() / total_volume * 100.0;
        let expense_percent = expense_sum.unwrap() / total_volume * 100.0;

        // Index Total
        let mut index_total = MustacheTotal::new();
        index_total.revenue = format!("{}", revenue_sum.to_display());
        index_total.revenue_percent = format!("{:.2}", revenue_percent);
        index_total.expense = format!("{}", expense_sum.to_display());
        index_total.expense_percent = format!("{:.2}", expense_percent);
        index_total.balance = format!("{}", balance_sum.to_display());
        index_total.balance_class = balance_sum.html_class();

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
            .insert_str("category_count", _mcategories.len().to_string())
        ;

        builder = builder.insert("total", &index_total)
            .expect("Cannot add 'total' field to builder");

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
