
use uuid::Uuid;
use crate::yaml::ToYaml;
use yaml_rust::Yaml;
use yaml_rust::yaml::Hash;

#[derive(Debug)]
pub struct Epic {
    id: String,
    handle: String,
    title: String,
    bgcolor: String,
}

impl Epic {
    pub fn new() -> Self {
        println!("-> Epic::new()");

        Self {
            id: Uuid::new_v4().to_string(),
            handle: "default".to_string(),
            title: "Default".to_string(),
            bgcolor: "#ffffff".to_string(),
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn handle(&self) -> String {
        self.handle.clone()
    }

    pub fn set_handle(&mut self, handle: String) {
        self.handle = handle;
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn bgcolor(&self) -> String {
        self.bgcolor.clone()
    }

    pub fn set_bgcolor(&mut self, bgcolor: String) {
        self.bgcolor = bgcolor;
    }
}

impl ToYaml for Epic {
    fn to_yaml(self) -> Yaml {
        println!("-> Epic::to_yaml()");

        let mut epic = Hash::new();
        epic.insert("id".to_string().to_yaml(), self.id().to_yaml());
        epic.insert("handle".to_string().to_yaml(), self.handle().to_yaml());
        epic.insert("title".to_string().to_yaml(), self.title().to_yaml());
        epic.insert("bg_color".to_string().to_yaml(), self.bgcolor().to_yaml());

        Yaml::Hash(epic)
    }
}

#[cfg(test)]
mod tests_basic {
    use super::Epic;

    #[test]
    fn test_epic1() {
        let e1 = Epic::new();
        assert_eq!("default", e1.handle());
        assert_eq!("Default", e1.title());
        assert_eq!("#ffffff", e1.bgcolor());
    }
}

#[cfg(test)]
mod tests_to_yaml {
    use super::Epic;
    use crate::yaml::ToYaml;
    use yaml_rust::Yaml;

    #[test]
    fn test_epic_to_yaml1() {
        let e1 = Epic::new();
        let h1 = e1.to_yaml();

        if let Yaml::Hash(ref epic) = h1 {
            let key = "handle".to_string().to_yaml();
            if let Yaml::String(ref val_ref) = epic[&key] {
                assert_eq!("default", val_ref);
            } else {
                assert!(false);
            }

            let key = "title".to_string().to_yaml();
            if let Yaml::String(ref val_ref) = epic[&key] {
                assert_eq!("Default", val_ref);
            } else {
                assert!(false);
            }

            let key = "bg_color".to_string().to_yaml();
            if let Yaml::String(ref val_ref) = epic[&key] {
                assert_eq!("#ffffff", val_ref);
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }
}
