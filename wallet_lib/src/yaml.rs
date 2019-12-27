
use std::path::PathBuf;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::string::ToString;
use std::fmt::{Display, Formatter, Result as FmtRes};
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
use yaml_rust::yaml::Hash;

#[derive(Debug)]
enum YamlFileKind {
    IndexFile,
    EpicsFile,
    MonthFile,
}

pub struct YamlFile {
    kind: YamlFileKind,
    path: PathBuf,
    changed: bool,
    content: Yaml,
}

impl YamlFile {
    pub fn open_index(path: PathBuf) -> Self {
        println!("-> YamlFile::open({:?})", path);
        YamlFile::open(YamlFileKind::IndexFile, path)
    }

    pub fn open_epics(path: PathBuf) -> Self {
        println!("-> YamlFile::open({:?})", path);
        YamlFile::open(YamlFileKind::EpicsFile, path)
    }

    pub fn open_month(path: PathBuf) -> Self {
        println!("-> YamlFile::open({:?})", path);
        YamlFile::open(YamlFileKind::MonthFile, path)
    }

    fn open(kind: YamlFileKind, path: PathBuf) -> Self {
        println!("-> YamlFile::open({:?}, {:?})", kind, path);

        let mut _f = Self {
            kind,
            path,
            changed: false,
            content: Yaml::Hash(Hash::new()),
        };
        _f.init();
        _f
    }

    fn init(&mut self) {
        println!("-> YamlFile::init()");

        if self.path.exists() && self.path.is_file() {
            // println!("-> read existing file");
            self.read();
        } else {
            println!("-> create new file");

            if let Yaml::Hash(ref mut content_ref) = self.content {
                match &self.kind {
                    YamlFileKind::IndexFile => {
                        println!("-> IndexFile");
                        let index_key = Yaml::String("index".to_string());
                        let index_val = Yaml::Array(Vec::new());
                        content_ref.insert(index_key, index_val);
                    },
                    YamlFileKind::MonthFile => {
                        println!("-> MonthFile");

                        // Meta
                        let index_key = Yaml::String("meta".to_string());
                        let index_val = Yaml::Hash(Hash::new());
                        content_ref.insert(index_key, index_val);

                        // Days
                        let index_key = Yaml::String("days".to_string());
                        let index_val = Yaml::Hash(Hash::new());
                        content_ref.insert(index_key, index_val);
                    }
                    _ => unreachable!("init() not implemented for {:?}", self.kind),
                }
            }
        }
    }

    fn read(&mut self) {
        println!("-> YamlFile::read()");
        let raw = read_to_string(&self.path).expect("Cannot read file");
        // println!("-> raw: '{}'", raw);

        let docs = YamlLoader::load_from_str(&raw).unwrap();
        // println!("-> docs: '{:?}'", docs);

        self.content = docs[0].clone();
    }

    pub fn add<T: ToYaml>(&mut self, i: T) {
        println!("-> YamlFile::add() -> {:?}", self.kind);
        // println!("-> YamlFile::add() -> {:?} '{:?}'", self.kind, i.to_string());

        // let v = i.clone().to_yaml();
        // println!("-> v: {:?}", v);

        if let Yaml::Hash(ref mut content_ref) = self.content {
            // println!("-> content_ref: {:?}", content_ref);

            match &self.kind {
                YamlFileKind::IndexFile => {
                    println!("-> YamlFile::add() IndexFile");

                    let index_key = "index".to_string().to_yaml();

                    if let Yaml::Array(ref mut index_ref) = content_ref[&index_key] {
                        // println!("-> index_ref: {:?}", index_ref);
                        index_ref.push(i.to_yaml());
                    }
                },
                YamlFileKind::MonthFile => {
                    println!("-> YamlFile::add() MonthFile");

                    let v = i.to_yaml();

                    if let Yaml::Hash(ref entry_ref) = v {
                        println!("-> entry_ref: {:?}", entry_ref);

                        let date_key = "date".to_string().to_yaml();
                        println!("-> date_key: {:?}", date_key);

                        if let Yaml::String(ref date_ref) = entry_ref[&date_key] {
                            println!("-> date_ref: {:?}", date_ref);

                            let index_key = "days".to_string().to_yaml();
                            println!("-> index_key: {:?}", index_key);

                            if let Yaml::Hash(ref mut index_ref) = content_ref[&index_key] {
                                // println!("-> index_ref: {:?}", index_ref);
                                // println!("-> key: {:?}", index_ref.contains_key(&entry_ref[&date_key]));

                                if !index_ref.contains_key(&entry_ref[&date_key]) {
                                    println!("-> create new day");
                                    index_ref.insert(entry_ref[&date_key].clone(), Yaml::Array(Vec::new()));
                                }

                                // println!("-> index_ref: {:?}", index_ref);
                                println!("-> date_key: {:?}", date_key);

                                if let Yaml::Array(ref mut day_ref) = index_ref[&entry_ref[&date_key]] {
                                    // println!("-> day_ref: {:?}", day_ref);
                                    day_ref.push(v);
                                }
                            }
                        }
                    }
                },
                _ => unreachable!("Yaml::add() not implemented for {:?}", self.kind),
            }
        }

        self.changed = true;
    }

    pub fn exists(&self, i: String) -> bool {
        println!("-> YamlFile::exists({:?})", i);

        if let Yaml::Hash(ref content_ref) = self.content {
            // println!("-> content_ref: {:?}", content_ref);

            match &self.kind {
                YamlFileKind::IndexFile => {
                    println!("-> IndexFile");

                    let index_key = Yaml::String("index".to_string());

                    // println!("-> index_key: '{:?}'", index_key);
                    // println!("-> val: {:?}", content_ref[&index_key]);

                    if let Yaml::Array(ref index_ref) = content_ref[&index_key] {
                        // println!("-> index_ref: {:?}", index_ref);
                        let v = Yaml::String(i.to_string());

                        // println!("-> v: {:?}", v);
                        // println!("-> contains: {:?}", index_ref.contains(&v));
                        return index_ref.contains(&v);
                    }
                },
                _ => unreachable!("Not implemented"),
            }
        }

        false
    }

    fn write(&mut self) {
        println!("-> YamlFile::write()");
        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(&self.content).unwrap();
        }
        out_str.push_str("\n");
        // println!("out: '{}'", out_str);

        println!("-> File::create");
        let mut file = File::create(&self.path)
            .expect("Cannot open file for writing");

            // println!("-> file.write_all");
        file.write_all(out_str.as_bytes())
            .expect("Cannot write file");

        self.changed = false;
    }

    /// Write file if content has changed.
    fn close(&mut self) {
        println!("-> YamlFile::close()");

        // Debug
        // self.changed = true;

        if !self.changed {
            // println!("-> nothing changed");
            return;
        }

        // println!("-> content changed");
        self.write();
    }
}

impl Drop for YamlFile {
    fn drop(&mut self) {
        // println!("-> YamlFile::drop()");
        self.close();
    }
}

pub trait ToYaml {
    fn to_yaml(self) -> Yaml;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::fs::remove_file;
    use std::str::FromStr;
    use super::{YamlFile, ToYaml};
    // use crate::ext::;
    use crate::entry::Entry;
    use crate::date::Date;

    #[test]
    fn test_yaml_index() {
        let ps1 = "../tmp/tests/index.yml";
        let p1 = PathBuf::from(ps1);
        let mut f1 = YamlFile::open_index(p1);
        f1.add("hi".to_string());
        f1.close();

        let p1 = PathBuf::from(ps1);
        assert!(p1.is_file());
    }

    #[test]
    fn test_yaml_month() {
        let d1 = Date::from_str("1987-02-21").unwrap();
        let mut e1 = Entry::new();
        e1.set_date(d1);
        e1.set_revenue(123.45);
        e1.set_expense(456.78);

        let ps1 = "../tmp/tests/month.yml";
        let p1 = PathBuf::from(ps1);
        let mut f1 = YamlFile::open_month(p1);
        f1.add(e1);
        f1.close();

        let p1 = PathBuf::from(ps1);
        assert!(p1.is_file());
        // assert!(false);
    }
}
