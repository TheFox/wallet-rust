
/// https://github.com/nickel-org/rust-mustache

use std::convert::From;
use std::fs::File;
use std::include_bytes;
use std::cmp::Eq;
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

const APP_NAME: &'static str = "WalletRust";
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");

#[derive(Debug, Serialize)]
struct MustacheCategory {
    name: String,
    //revenue: String,
    //expense: String,
    balance: String,
}

impl MustacheCategory {
    fn new() -> Self {
        Self {
            name: String::new(),
            //revenue: format!("{}", cat_sum.revenue.to_display()),
            //expense: format!("{}", cat_sum.expense.to_display()),
            balance: String::new(),
        }
    }
}

impl From<&CategorySummary> for MustacheCategory {
    fn from(cat_sum: &CategorySummary) -> Self {
        println!("-> MustacheCategory::from() -> {:?}", cat_sum);
        Self {
            name: cat_sum.name.clone(),
            //revenue: format!("{}", cat_sum.revenue.to_display()),
            //expense: format!("{}", cat_sum.expense.to_display()),
            balance: format!("{}", cat_sum.balance.to_display()),
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
    balance_sum: String,
    balance_sum_class: String,
    categories: MustacheCategories
}

impl From<&YearSummary> for MustacheYear {
    fn from(year_sum: &YearSummary) -> Self {
        println!("-> MustacheYear::from()");
        Self {
            index: 0,
            year: year_sum.year,
            revenue: format!("{}", year_sum.revenue.to_display()),
            expense: format!("{}", year_sum.expense.to_display()),
            balance: format!("{}", year_sum.balance.to_display()),
            balance_sum: String::new(),
            balance_sum_class: String::new(),
            categories: vec![],
        }
    }
}

type MustacheYears = Vec<MustacheYear>;

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

        println!("-> IndexMustacheFile::render() File::create");
        let mut _file = match File::create(&self.path) {
            Ok(file) => file,
            Err(why) => panic!("Cannot create {}: {}", self.path, why),
        };

        let mut index: u32 = 0;
        let mut balance_sum = Number::new();

        // Mustache Years
        let _myears: MustacheYears = _result.years.values()
            .map(|year_sum| {
                index += 1;
                println!("-> Mustache Year: {:?}", year_sum.year);

                balance_sum += year_sum.balance;
                println!("-> balance_sum: {:.2}", balance_sum.to_display());

                let mut _myear = MustacheYear::from(year_sum);
                _myear.index = index;
                _myear.balance_sum = format!("{}", balance_sum.to_display());
                if balance_sum.is_negative() {
                    _myear.balance_sum_class = "red".to_string();
                }

                // Add Categories to Year.
                for (category_name, category_sum) in &year_sum.categories {
                    println!("  -> year {:?}, category: {:?}", year_sum.year, category_name);

                    let mut _mcategory = MustacheCategory::from(category_sum);
                    _myear.categories.push(_mcategory);
                }

                // Return Mustache Year.
                _myear
            })
            .collect();
        //println!("-> _myears: {:?}", _myears.size());

        // Mustache Categories
        let _mcategories: MustacheCategories = _result.categories.values()
            .map(|category_sum| {
                println!("-> Mustache Category => {:?}", category_sum);

                let _mcategory = MustacheCategory::from(category_sum);

                // Return Mustache Category.
                _mcategory
            })
            .collect();
        //println!("-> _mcategories: {:?}", _mcategories);

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
        let mut f_categories = move |mut builder: VecBuilder| {
            for c in &_mcategories {
                //println!("-> category {:?}", c);
                builder = builder.push(&c).unwrap();
            }
            builder
        };

        let data = MapBuilder::new()
            .insert_str("PROJECT_NAME", APP_NAME)
            .insert_str("PROJECT_VERSION_FULL", APP_VERSION)
            .insert_str("PROJECT_HOMEPAGE_URL", APP_HOMEPAGE)

            .insert_str("generated_at", now.format("%F %T %z").to_string())
            .insert_str("css_relative_path", ".")
            .insert_str("relative_path", ".")

            .insert_vec("years", f_years)
            .insert_vec("categories", f_categories)

            .build();

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
