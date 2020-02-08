
/// https://github.com/nickel-org/rust-mustache

use std::fs::File;
use mustache::{MapBuilder, compile_str};
use mustache::VecBuilder;
use std::include_bytes;
use std::env::current_dir;
use chrono::{Local, DateTime};
// use std::collections::HashMap;

const APP_NAME: &'static str = "WalletRust";
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");

pub struct MustacheContent {
}
impl MustacheContent {
    pub fn new() -> Self {
        Self {}
    }
}

pub enum MustacheFileKind {
    IndexFile,
    YearFile,
    MonthFile,
}

pub struct MustacheFile {
    kind: MustacheFileKind,
    path: String,
}

impl MustacheFile {
    /// New Mustache file.
    pub fn new(kind: MustacheFileKind, path: String) -> Self {
        Self {
            kind,
            path,
        }
    }

    /// Create Mustache Builder.
    fn builder(&self) -> MapBuilder {
        // Now
        let now: DateTime<Local> = Local::now();

        let cwd = current_dir().expect("Cannot get current dir");
        println!("cwd: {}", cwd.display());

        let up = cwd.join("..");
        println!("up: {}", up.display());

        MapBuilder::new()
            .insert_str("PROJECT_NAME", APP_NAME)
            .insert_str("PROJECT_VERSION_FULL", APP_VERSION)
            .insert_str("PROJECT_HOMEPAGE_URL", APP_HOMEPAGE)

            .insert_str("generated_at", now.format("%F %T %z").to_string())
            .insert_str("css_relative_path", ".")
            .insert_str("relative_path", ".")
    }

    /// Render file.
    pub fn render(&self, content: MustacheContent) {
        println!("-> MustacheFile::render()");

        let cwd = current_dir().expect("Cannot get current dir");
        // println!("-> cwd: {}", cwd.display());

        let raw = match self.kind {
            MustacheFileKind::IndexFile => {
                let bytes = include_bytes!("../../resources/views/index.mustache");
                String::from_utf8_lossy(bytes)
            },
            MustacheFileKind::YearFile  => {
                let bytes = include_bytes!("../../resources/views/year.mustache");
                String::from_utf8_lossy(bytes)
            },
            MustacheFileKind::MonthFile => {
                let bytes = include_bytes!("../../resources/views/month.mustache");
                String::from_utf8_lossy(bytes)
            },
        };

        let template = compile_str(&raw).unwrap();

        println!("-> File::create");
        let mut _file = match File::create(&self.path) {
            Ok(file) => file,
            Err(why) => panic!("Cannot create {}: {}", self.path, why),
        };

        let mut users = vec!["hello", "world"];

        let data = self.builder()
            // .insert_fn("users", move |_| {
            //     println!("-> users: {:?}", users);
            //     users.pop().unwrap().into()
            // })
            .insert_vec("users", move |mut builder| {
                for user in &users {
                    builder = builder.push_str(&user);
                }
                builder
            })
            .build();

        println!("-> render_data");
        template.render_data(&mut _file, &data)
            .expect("Failed to render");
    }
}

#[cfg(test)]
mod tests_base {
    use super::{MustacheFile, MustacheFileKind};

    #[test]
    fn test_mustachefile1() {
        MustacheFile::new(MustacheFileKind::IndexFile, "../tmp/tests/test1.html".to_string());
    }
}

#[cfg(test)]
mod tests_render {
    use super::{MustacheFile, MustacheFileKind, MustacheContent};
    use std::fs::create_dir_all;
    // use crate::wallet::GetEntries;

    // struct TestResult {}
    // impl TestResult {
    //     fn new() -> Self {
    //         TestResult {}
    //     }
    // }
    // impl GetEntries for TestResult {}

    fn setup() {
        create_dir_all("../tmp/tests/mustache")
            .expect("Cannot create mustache test directory.");
    }

    #[test]
    fn test_mustachefile_render_index_file() {
        setup();

        let c1 = MustacheContent::new();

        let f1 = MustacheFile::new(MustacheFileKind::IndexFile, "../tmp/tests/mustache/index.html".to_string());
        f1.render(c1);
    }

//     #[test]
//     fn test_mustachefile_render_year_file() {
//         let res1 = setup();

//         let f1 = MustacheFile::new(MustacheFileKind::YearFile, "../tmp/tests/mustache/year.html".to_string());
//         f1.render(res1);
//     }

//     #[test]
//     fn test_mustachefile_render_month_file() {
//         let res1 = setup();

//         let f1 = MustacheFile::new(MustacheFileKind::MonthFile, "../tmp/tests/mustache/month.html".to_string());
//         f1.render(res1);
//     }
}
