
/// https://github.com/nickel-org/rust-mustache

use std::fs::File;
// use std::fs::read_to_string;
// use std::io::{stdout, Write, Error};
// use std::io::Error;
use mustache::{MapBuilder, compile_str};
use std::include_bytes;
use std::env::current_dir;
// use std::path::PathBuf;
use chrono::{Local, DateTime};

const APP_NAME: &'static str = "WalletRust";
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
// const APP_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const APP_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");

// const INDEX_FILE_TPL: &'static str = "../../resources/views/index.mustache";
// const YEAR_FILE_TPL: &'static str = "../../resources/views/year.mustache";
// const MONTH_FILE_TPL: &'static str = "../../resources/views/month.mustache";

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
    }

    /// Render file.
    pub fn render(&self) {
        println!("-> MustacheFile::render()");

        let cwd = current_dir().expect("Cannot get current dir");
        println!("cwd: {}", cwd.display());

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
        let builder = self.builder();
        match self.kind {
            MustacheFileKind::IndexFile => {
            },
            MustacheFileKind::YearFile  => {
            },
            MustacheFileKind::MonthFile => {
            },
        }
        let data = builder.build();

        println!("-> File::create");
        let mut _file = match File::create(&self.path) {
            Ok(file) => file,
            Err(why) => panic!("Cannot create {}: {}", self.path, why),
        };

        template.render_data(&mut _file, &data).unwrap();
    }
}

#[cfg(test)]
mod tests_base {
    use super::{MustacheFile, MustacheFileKind};

    #[test]
    fn test_mustachefile1() {
        MustacheFile::new(MustacheFileKind::IndexFile, "../tmp/test1.html".to_string());
    }
}

#[cfg(test)]
mod tests_render {
    use super::{MustacheFile, MustacheFileKind};
    use std::fs::create_dir_all;

    fn setup() {
        create_dir_all("../tmp/tests/mustache")
            .expect("Cannot create mustache test directory.");
    }

    #[test]
    fn test_mustachefile_render_index_file() {
        setup();

        MustacheFile::new(MustacheFileKind::IndexFile, "../tmp/tests/mustache/index.html".to_string())
            .render();
    }

    #[test]
    fn test_mustachefile_render_year_file() {
        MustacheFile::new(MustacheFileKind::YearFile, "../tmp/tests/mustache/year.html".to_string())
            .render();
    }

    #[test]
    fn test_mustachefile_render_month_file() {
        MustacheFile::new(MustacheFileKind::MonthFile, "../tmp/tests/mustache/month.html".to_string())
            .render();
    }
}
