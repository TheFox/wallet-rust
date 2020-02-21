
/// https://github.com/nickel-org/rust-mustache

// use std::convert::From;
use std::fs::File;
use mustache::{MapBuilder, VecBuilder, compile_str};
// use mustache::serde;
use std::include_bytes;
// use std::env::current_dir;
use chrono::{Local, DateTime};
// use std::collections::HashMap;
use serde::Serialize;
// use std::fmt::Display;
use crate::wallet::FilterResult;
use crate::wallet::YearSummary;
use crate::wallet::Year;
use crate::number::Number;
use crate::number::ToDisplay;

const APP_NAME: &'static str = "WalletRust";
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");

#[derive(Debug, Serialize)]
struct MustacheYear {
    index: u32,
    year: Year,
    revenue: String,
    expense: String,
    balance: String,
    balance_sum: String,
}

impl MustacheYear {
    fn from_summary(year_sum: &YearSummary, index: u32, balance_sum: &Number) -> Self {
        Self {
            index,
            year: year_sum.year,
            revenue: format!("{}", year_sum.revenue.to_display()),
            expense: format!("{}", year_sum.expense.to_display()),
            balance: format!("{}", year_sum.balance.to_display()),
            balance_sum: format!("{}", balance_sum.to_display()),
        }
    }
}

// impl From<&YearSummary> for MustacheYear {
//     fn from(year_sum: &YearSummary) -> Self {
//         println!("-> MustacheFile::from()");
//         Self {
//             year: year_sum.year,
//             revenue: format!("{}", year_sum.revenue.to_display()),
//             expense: format!("{}", year_sum.expense.to_display()),
//             balance: format!("{}", year_sum.balance.to_display()),
//             balance_sum: String::new(),
//         }
//     }
// }

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

        println!("-> File::create");
        let mut _file = match File::create(&self.path) {
            Ok(file) => file,
            Err(why) => panic!("Cannot create {}: {}", self.path, why),
        };

        let mut index: u32 = 0;
        let mut balance_sum = Number::new();

        // let _i: Vec<MustacheYear> = _result.years.values().map(|year_sum| MustacheYear::from(year_sum)).collect();
        let _myears: Vec<MustacheYear> = _result.years.values()
            .map(|year_sum| {
                index += 1;
                println!("-> index: {:?}", index);

                balance_sum += year_sum.balance;
                println!("-> balance_sum: {:.2}", balance_sum.to_display());

                MustacheYear::from_summary(year_sum, index, &balance_sum)
            })
            .collect();
        println!("-> _myears: {:?}", _myears);

        let mut f_years = move |mut builder: VecBuilder| {
            // let mut balance_sum = Number::new();

            for y in &_myears {
                println!("-> year {:?}", y.year);
                builder = builder.push(&y).unwrap();
            }
            builder
        };

        // let mut planets = vec![
        //     Planet{ name: "Bhello".into() },
        //     Planet{ name: "Bworld".into() },
        // ];
        // let mut f_planets = move |mut builder: VecBuilder| {
        //     println!("-> f_planets");

        //     for item in &planets {
        //         builder = builder.push(&item).unwrap();
        //     }
        //     builder
        // };

        let data = MapBuilder::new()
            .insert_str("PROJECT_NAME", APP_NAME)
            .insert_str("PROJECT_VERSION_FULL", APP_VERSION)
            .insert_str("PROJECT_HOMEPAGE_URL", APP_HOMEPAGE)

            .insert_str("generated_at", now.format("%F %T %z").to_string())
            .insert_str("css_relative_path", ".")
            .insert_str("relative_path", ".")

            .insert_vec("years", f_years)

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
