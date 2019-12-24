
use std::fs::{read_to_string, create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use std::fmt;
use std::string::ToString;
use std::vec::Vec;
// use yaml_rust::linked_hash_map::LinkedHashMap;
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
use yaml_rust::yaml::Hash;
use crate::entry::Entry;

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
    pub fn new(path: String) -> Self {
        println!("-> Wallet::new({})", path);

        // let basedir = PathBuf::new(path.clone());

        let mut basedir = PathBuf::new();
        basedir.push(path);

        let mut data_dir = basedir.clone();
        data_dir.push("data");

        let mut html_dir = basedir.clone();
        html_dir.push("html");

        let mut tmp_dir = basedir.clone();
        tmp_dir.push("tmp");

        let mut index_file = data_dir.clone();
        index_file.push("index.yml");

        let mut epics_file = data_dir.clone();
        epics_file.push("epics.yml");

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

    // TODO
    pub fn add(&self, entry: Entry, force: bool) -> bool {
        println!("-> Wallet::add(f={:?})", force);
        // println!("-> entry {:?}", entry);

        // Index
        let mut index_file = YamlFile::open_index(self.index_file.clone());
        println!("-> exists: {:?}", index_file.exists(entry.id()));
        if !force && index_file.exists(entry.id()) {
            return false;
        }
        index_file.add(entry.id());

        // Epics
        // let mut epics_file = YamlFile::open_epics(self.epics_file.clone());
        // index_file.add(2 as u16);
        // index_file.add(3);
        // index_file.add(entry);

        // Month file
        // let month_file_name = format!("month_{}.yml", entry.date().fym("_"));
        // println!("-> month_file_name: {:?}", month_file_name);

        // let mut month_file_path = self.data_dir.clone();
        // month_file_path.push(month_file_name);
        // println!("-> month_file_path: {:?}", month_file_path);
        // println!("-> month_file_path: {}", month_file_path.to_string());

        // let mut month_file = YamlFile::open_month(month_file_path);
        // month_file.add(entry);

        true
    }

    // TODO
    pub fn list(&self) {
        println!("-> Wallet::list()");
    }

    // TODO
    pub fn html(&self) {
        println!("-> Wallet::html()");
    }
}

#[derive(Debug)]
enum YamlFileKind {
    IndexFile,
    EpicsFile,
    MonthFile,
}

struct YamlFile {
    kind: YamlFileKind,
    path: PathBuf,
    changed: bool,
    content: Yaml,
}

impl YamlFile {
    fn open_index(path: PathBuf) -> Self {
        println!("-> YamlFile::open({:?})", path);
        YamlFile::open(YamlFileKind::IndexFile, path)
    }

    fn open_epics(path: PathBuf) -> Self {
        println!("-> YamlFile::open({:?})", path);
        YamlFile::open(YamlFileKind::EpicsFile, path)
    }

    // fn open_month(path: PathBuf) -> Self {
    //     println!("-> YamlFile::open()");
    //     YamlFile::open(YamlFileKind::MonthFile, path)
    // }

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
                    _ => unreachable!("Not implemented"),
                }
            }
        }
    }

    fn read(&mut self) {
        println!("-> YamlFile::read()");
        let raw = read_to_string(&self.path).expect("Cannot read file");
        // println!("-> raw: '{}'", raw);

        let docs = YamlLoader::load_from_str(&raw).unwrap();
        println!("-> docs: '{:?}'", docs);

        self.content = docs[0].clone();
    }

    fn add<T: ToString>(&mut self, i: T) {
        println!("-> YamlFile::add() -> {:?} '{:?}'", self.kind, i.to_string());

        if let Yaml::Hash(ref mut content_ref) = self.content {
            println!("-> content_ref: {:?}", content_ref);

            match &self.kind {
                YamlFileKind::IndexFile => {
                    println!("-> IndexFile");

                    let index_key = Yaml::String("index".to_string());

                    println!("-> index_key: '{:?}'", index_key);
                    println!("-> val: {:?}", content_ref[&index_key]);

                    // content_ref[&key].push(i);

                    // let key2 = Yaml::String(i.to_string());
                    // println!("-> key2: {:?}", key2);
                    // println!("-> key2: {:?}", content_ref["OK"].is_badvalue());

                    if let Yaml::Array(ref mut index_ref) = content_ref[&index_key] {
                        println!("-> index_ref: {:?}", index_ref);
                        let v = Yaml::String(i.to_string());

                        println!("-> v: {:?}", v);
                        index_ref.push(v);
                    }
                },
                YamlFileKind::MonthFile => {
                    match i {
                        Entry => (),
                        _ => unreachable!(),
                    }

                    println!("-> YamlFile::add() Entry");
                },
                _ => unreachable!("Not implemented"),
            }
        }

        self.changed = true;
    }

    fn exists(&self, i: String) -> bool {
        println!("-> YamlFile::exists({:?})", i);

        if let Yaml::Hash(ref content_ref) = self.content {
            println!("-> content_ref: {:?}", content_ref);

            match &self.kind {
                YamlFileKind::IndexFile => {
                    println!("-> IndexFile");

                    let index_key = Yaml::String("index".to_string());

                    println!("-> index_key: '{:?}'", index_key);
                    println!("-> val: {:?}", content_ref[&index_key]);

                    if let Yaml::Array(ref index_ref) = content_ref[&index_key] {
                        println!("-> index_ref: {:?}", index_ref);
                        let v = Yaml::String(i.to_string());

                        println!("-> v: {:?}", v);
                        println!("-> contains: {:?}", index_ref.contains(&v));
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

        self.changed = true;

        if !self.changed {
            println!("-> nothing changed");
            return;
        }

        println!("-> content changed");
        self.write();
    }
}

impl Drop for YamlFile {
    fn drop(&mut self) {
        // println!("-> YamlFile::drop()");
        self.close();
    }
}

#[cfg(test)]
mod tests {
    use super::Wallet;
    // use std::fs;
    use std::path::Path;

    #[test]
    fn test_new_wallet() {
        let w1 = Wallet::new(String::from("../tmp/tests/wallet1"));
        assert!(Path::new("../tmp/tests/wallet1").exists());
        assert!(Path::new("../tmp/tests/wallet1/data").exists());
        assert!(Path::new("../tmp/tests/wallet1/html").exists());
        assert!(Path::new("../tmp/tests/wallet1/tmp").exists());
    }

    // #[test]
    // fn test_wallet_add() {
    //     let w1 = Wallet::new(String::from("../tmp/tests/wallet2"));
    // }
}
