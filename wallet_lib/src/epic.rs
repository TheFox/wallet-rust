
// use std::fmt::{Display, Formatter, Result as FmtRes};
use uuid::Uuid;
use crate::yaml::ToYaml;
use yaml_rust::Yaml;
use yaml_rust::yaml::Hash;

#[derive(Debug)]
pub struct Epic {
    id: String,
    handle: String,
    title: String,
    bg_color: String,
}

impl Epic {
    pub fn new() -> Self {
        println!("-> Epic::new()");

        Self {
            id: Uuid::new_v4().to_string(),
            handle: String::from("default"),
            title: String::from("Default"),
            bg_color: String::from("#ffffff"),
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn handle(&self) -> String {
        self.handle.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn bg_color(&self) -> String {
        self.bg_color.clone()
    }
}

impl ToYaml for Epic {
    fn to_yaml(self) -> Yaml {
        println!("-> Epic::to_yaml()");

        let mut epic = Hash::new();
        epic.insert("id".to_string().to_yaml(), self.id().to_yaml());
        epic.insert("handle".to_string().to_yaml(), self.handle().to_yaml());
        epic.insert("title".to_string().to_yaml(), self.title().to_yaml());
        epic.insert("bg_color".to_string().to_yaml(), self.bg_color().to_yaml());

        Yaml::Hash(epic)
    }
}

#[cfg(test)]
mod tests {
    use super::Epic;

    #[test]
    fn test_epic_default() {
        let e1 = Epic::new();
        assert_eq!("default", e1.handle());
        assert_eq!("Default", e1.title());
        assert_eq!("#ffffff", e1.bg_color());
    }
}
