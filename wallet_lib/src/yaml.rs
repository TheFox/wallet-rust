
use std::path::PathBuf;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::string::ToString;
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
use yaml_rust::yaml::Hash;
use chrono::{DateTime, Utc};

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
        //println!("-> YamlFile::open({:?})", path);
        YamlFile::open(YamlFileKind::IndexFile, path)
    }

    pub fn open_epics(path: PathBuf) -> Self {
        //println!("-> YamlFile::open({:?})", path);
        YamlFile::open(YamlFileKind::EpicsFile, path)
    }

    pub fn open_month(path: PathBuf) -> Self {
        //println!("-> YamlFile::open({:?})", path);
        YamlFile::open(YamlFileKind::MonthFile, path)
    }

    fn open(kind: YamlFileKind, path: PathBuf) -> Self {
        //println!("-> YamlFile::open({:?}, {:?})", kind, path);

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
        //println!("-> YamlFile::init()");

        if self.path.exists() && self.path.is_file() {
            // println!("-> read existing file");
            self.read();
        } else {
            //println!("-> create new file");

            self.changed = true;

            if let Yaml::Hash(ref mut content_ref) = self.content {
                match &self.kind {
                    YamlFileKind::IndexFile => {
                        //println!("-> IndexFile");
                        let index_key = "index".to_string().to_yaml();
                        let index_val = Yaml::Array(Vec::new());
                        content_ref.insert(index_key, index_val);
                    },
                    YamlFileKind::EpicsFile => {
                        //println!("-> EpicsFile");
                        let index_key = "epics".to_string().to_yaml();
                        let index_val = Yaml::Array(Vec::new());
                        content_ref.insert(index_key, index_val);
                    },
                    YamlFileKind::MonthFile => {
                        //println!("-> MonthFile");

                        // Meta
                        let mut meta = Hash::new();
                        meta.insert("version".to_string().to_yaml(), 3i64.to_yaml());

                        let utc: DateTime<Utc> = Utc::now();
                        meta.insert("created_at".to_string().to_yaml(), utc.format("%FT%T%:z").to_string().to_yaml());
                        meta.insert("updated_at".to_string().to_yaml(), utc.format("%FT%T%:z").to_string().to_yaml());

                        let index_key = "meta".to_string().to_yaml();
                        let index_val = Yaml::Hash(meta);
                        content_ref.insert(index_key, index_val);

                        // Days
                        let index_key = "days".to_string().to_yaml();
                        let index_val = Yaml::Hash(Hash::new());
                        content_ref.insert(index_key, index_val);
                    },
                    // _ => unreachable!("init() not implemented for {:?}", self.kind),
                }
            }
        }
    }

    fn read(&mut self) {
        //println!("-> YamlFile::read()");
        let raw = read_to_string(&self.path).expect("Cannot read file");
        // println!("-> raw: '{}'", raw);

        let docs = YamlLoader::load_from_str(&raw).unwrap();
        // println!("-> docs: '{:?}'", docs);

        self.content = docs[0].clone();
    }

    pub fn add<T: ToYaml>(&mut self, obj: T) {
        //println!("-> YamlFile::add() -> {:?}", self.kind);

        if let Yaml::Hash(ref mut content_ref) = self.content {
            // println!("-> content_ref: {:?}", content_ref);

            match &self.kind {
                YamlFileKind::IndexFile => {
                    //println!("-> IndexFile");

                    let index_key = "index".to_string().to_yaml();

                    if let Yaml::Array(ref mut index_ref) = content_ref[&index_key] {
                        // println!("-> index_ref: {:?}", index_ref);
                        index_ref.push(obj.to_yaml());
                    }
                },
                YamlFileKind::EpicsFile => {
                    //println!("-> EpicsFile");

                    let index_key = "epics".to_string().to_yaml();

                    if let Yaml::Array(ref mut index_ref) = content_ref[&index_key] {
                        // println!("-> index_ref: {:?}", index_ref);
                        index_ref.push(obj.to_yaml());
                    }
                },
                YamlFileKind::MonthFile => {
                    //println!("-> YamlFile::add() MonthFile");

                    let v = obj.to_yaml();

                    if let Yaml::Hash(ref entry_ref) = v {
                        // println!("-> entry_ref: {:?}", entry_ref);

                        let date_key = "date".to_string().to_yaml();
                        // println!("-> date_key: {:?}", date_key);

                        // TODO: date_ref not used?
                        if let Yaml::String(ref _date_ref) = entry_ref[&date_key] {
                            // println!("-> date_ref: {:?}", date_ref);

                            let index_key = "days".to_string().to_yaml();
                            // println!("-> index_key: {:?}", index_key);

                            if let Yaml::Hash(ref mut index_ref) = content_ref[&index_key] {
                                // println!("-> index_ref: {:?}", index_ref);
                                // println!("-> key: {:?}", index_ref.contains_key(&entry_ref[&date_key]));

                                if !index_ref.contains_key(&entry_ref[&date_key]) {
                                    // println!("-> create new day");
                                    index_ref.insert(entry_ref[&date_key].clone(), Yaml::Array(Vec::new()));
                                }

                                // println!("-> index_ref: {:?}", index_ref);
                                // println!("-> date_key: {:?}", date_key);

                                if let Yaml::Array(ref mut day_ref) = index_ref[&entry_ref[&date_key]] {
                                    // println!("-> day_ref: {:?}", day_ref);
                                    day_ref.push(v);
                                }
                            }
                        }
                    }
                },
                // _ => unreachable!("Yaml::add() not implemented for {:?}", self.kind),
            }
        }

        self.changed = true;
    }

    pub fn exists<T: ToYaml>(&self, id: T) -> bool {
        //println!("-> YamlFile::exists()");

        // let str1 = id.to_string();

        if let Yaml::Hash(ref content_ref) = self.content {
            // println!("-> content_ref: {:?}", content_ref);

            match &self.kind {
                YamlFileKind::IndexFile => {
                    //println!("-> IndexFile");

                    let index_key = "index".to_string().to_yaml();

                    // println!("-> index_key: '{:?}'", index_key);
                    // println!("-> val: {:?}", content_ref[&index_key]);

                    if let Yaml::Array(ref index_ref) = content_ref[&index_key] {
                        // println!("-> index_ref: {:?}", index_ref);
                        // let v = Yaml::String(id.to_string());
                        // let v = id.to_string().to_yaml();
                        let v = id.to_yaml();

                        // println!("-> v: {:?}", v);
                        // println!("-> contains: {:?}", index_ref.contains(&v));
                        return index_ref.contains(&v);
                    }
                },
                YamlFileKind::EpicsFile => {
                    // println!("-> EpicsFile");

                    let index_key = "epics".to_string().to_yaml();

                    if let Yaml::Array(ref index_ref) = content_ref[&index_key] {
                        // println!("-> index_ref: {:?}", index_ref);

                        let idy = id.to_yaml();

                        // Filter
                        let mut filter = index_ref.iter().filter(|x| -> bool {
                            // println!("-> x: {:?}", x);

                            if let Yaml::Hash(ref epic_ref) = x {
                                // println!("-> epic_ref: {:?}", epic_ref);

                                let handle_key = "handle".to_string().to_yaml();
                                // println!("-> id: '{:?}'", id);
                                // println!("-> handle A: '{:?}'", epic_ref[&handle_key]);
                                // println!("-> eq: '{:?}'", epic_ref[&handle_key] == id.to_yaml());
                                // println!("-> eq: '{:?}'", epic_ref[&handle_key] == idy);

                                // if let Yaml::String(handle_ref) = &epic_ref[&handle_key] {
                                //     println!("-> handle B: {:?}", handle_ref);
                                // //     return handle_ref == &id;
                                //     // return handle_ref == &id.to_yaml();
                                // }
                                return epic_ref[&handle_key] == idy;
                            }

                            false
                        });

                        // Find
                        if let Some(_) = filter.next() {
                            return true;
                        }
                    }
                },
                _ => unreachable!("Not implemented"),
            }
        }

        false
    }

    pub fn get<T: FromYaml>(&self) -> Vec<T> {
        //println!("-> YamlFile::get() -> {:?}", self.kind);

        let mut items: Vec<T> = vec![];

        if let Yaml::Hash(ref content_ref) = self.content {
            // println!("-> content_ref: {:?}", content_ref);

            match &self.kind {
                YamlFileKind::MonthFile => {
                    // println!("-> MonthFile");

                    let index_key = "days".to_string().to_yaml();
                    // println!("-> index_key: {:?}", index_key);

                    if let Yaml::Hash(ref index_ref) = content_ref[&index_key] {
                        // println!("-> index_ref: {:?}", index_ref);

                        for (_, day) in index_ref.iter() {
                            // println!("-> day: {:?}", day);

                            if let Yaml::Array(ref day_ref) = day {
                                // println!("-> day_ref: {:?}", day_ref);

                                for item in day_ref.iter() {
                                    // println!("-> item: {:?}", item);
                                    items.push(T::from_yaml(item));
                                }
                            }
                        }
                    }
                },
                _ => unreachable!("Yaml::get() not implemented for {:?}", self.kind),
            }
        }

        items
    }

    fn write(&mut self) {
        // println!("-> YamlFile::write()");
        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(&self.content).unwrap();
        }
        out_str.push_str("\n");
        // println!("out: '{}'", out_str);

        // println!("-> File::create");
        let mut file = File::create(&self.path)
            .expect("Cannot open file for writing");

        // println!("-> file.write_all");
        file.write_all(out_str.as_bytes())
            .expect("Cannot write file");

        self.changed = false;
    }

    /// Write file if content has changed.
    fn close(&mut self) {
        // println!("-> YamlFile::close()");

        if !self.changed {
            return;
        }

        if let Yaml::Hash(ref mut content_ref) = self.content {
            // println!("content_ref: {:?}", content_ref);

            match &self.kind {
                YamlFileKind::MonthFile => {
                    // println!("-> MonthFile");

                    let index_key = "meta".to_string().to_yaml();

                    if let Yaml::Hash(ref mut index_ref) = content_ref[&index_key] {
                        // println!("-> index_ref: {:?}", index_ref);

                        // Version
                        let version_key = "version".to_string().to_yaml();
                        // println!("-> version_key: {:?}", index_ref[&version_key]);

                        if let Yaml::Integer(version) = index_ref[&version_key] {
                            // println!("-> version: {:?}", version);

                            if version < 3 {
                                // println!("-> new version");

                                index_ref[&version_key] = 3i64.to_yaml();
                            }
                        }

                        // Updated At
                        let updatedat_key = "updated_at".to_string().to_yaml();
                        // println!("-> updatedat_key: {:?}", index_ref[&updatedat_key]);

                        let utc: DateTime<Utc> = Utc::now();
                        // index_ref.insert("updated_at".to_string().to_yaml(), "x".to_string().to_yaml());
                        index_ref.insert("updated_at".to_string().to_yaml(), utc.format("%FT%T%:z").to_string().to_yaml());
                    }
                },
                _ => (),
            }
        }

        self.write();
    }
}

impl Drop for YamlFile {
    fn drop(&mut self) {
        self.close();
    }
}

pub trait ToYaml {
    fn to_yaml(self) -> Yaml;
}

pub trait FromYaml {
    fn from_yaml(_: &Yaml) -> Self;
}

#[cfg(test)]
mod tests_file {
    use std::path::PathBuf;
    use std::str::FromStr;
    use super::YamlFile;
    use crate::entry::Entry;
    use crate::epic::Epic;
    use crate::date::Date;

    #[test]
    fn test_yaml_index() {
        let ps1 = "../tmp/tests/index.yml";
        let p1 = PathBuf::from(ps1);
        let mut f1 = YamlFile::open_index(p1);
        f1.add("hi".to_string());
        f1.close();

        assert!(f1.exists("hi".to_string()));

        let p1 = PathBuf::from(ps1);
        assert!(p1.is_file());
    }

    #[test]
    fn test_yaml_epics() {
        let mut e1 = Epic::new();
        e1.set_handle("h1".to_string());

        let ps1 = "../tmp/tests/epics.yml";
        let p1 = PathBuf::from(ps1);
        let mut f1 = YamlFile::open_epics(p1);
        f1.add(e1);
        f1.close();

        assert!(f1.exists("h1".to_string()));
        assert!(!f1.exists("h2".to_string()));

        let p1 = PathBuf::from(ps1);
        assert!(p1.is_file());

        // assert!(false);
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
