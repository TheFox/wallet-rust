/// Extern Implementations

use crate::number::NumberType;
use crate::yaml::ToYaml;
use crate::string::{ShortString, ToShortString};
use yaml_rust::Yaml;
use yaml_rust::yaml::Hash;

pub trait StringExt {
    fn replace_comma(&self) -> String;
    fn to_num(&self) -> NumberType;
}

impl StringExt for String {
    /// On some places on earth you write 1,23
    /// instead of 1.23 as a floating point number.
    /// This function replaces ',' with '.'.
    fn replace_comma(&self) -> String {
        self.replace(",", ".")
    }

    /// Convert String to Number.
    fn to_num(&self) -> NumberType {
        self.parse().expect("Failed to convert String to NumberType")
    }
}

impl ToYaml for String {
    fn to_yaml(self) -> Yaml {
        Yaml::String(self)
    }
}

impl ToShortString for String {
    fn to_short_string(self, max_len: usize) -> ShortString {
        ShortString::from(self, max_len)
    }
}

pub trait BoolExt {
    fn yn(self) -> String;
}

impl BoolExt for bool {
    fn yn(self) -> String {
        println!("-> bool.yn()");

        String::from(if self {
            "YES"
        } else {
            "No"
        })
    }
}

impl ToYaml for i64 {
    fn to_yaml(self) -> Yaml {
        Yaml::Integer(self)
    }
}

impl ToYaml for f64 {
    fn to_yaml(self) -> Yaml {
        Yaml::Real(self.to_string())
    }
}

impl ToYaml for Hash {
    fn to_yaml(self) -> Yaml {
        Yaml::Hash(self)
    }
}

#[cfg(test)]
mod tests_str_ext {
    use super::StringExt;
    use crate::yaml::ToYaml;
    use yaml_rust::Yaml;

    #[test]
    fn test_strext_replace_comma1() {
        let s1 = "1,2".to_string();
        assert_eq!("1.2", s1.replace_comma());
    }

    #[test]
    fn test_strext_to_num1() {
        let s1 = "1.3".to_string();
        assert_eq!(1.3, s1.to_num());
    }

    #[test]
    fn test_strext_to_num2() {
        let s1 = "1,3".to_string();
        assert_eq!(1.3, s1.replace_comma().to_num());
    }

    #[test]
    fn test_strext_to_yaml1() {
        let s1 = "hello1".to_string().to_yaml();

        assert!(match s1 {
            Yaml::String(s2) => {
                // println!("s2 '{}'", s2);
                assert_eq!("hello1", s2);
                true
            }
            _ => false,
        });
    }
}

#[cfg(test)]
mod tests_bool_ext {
    use super::BoolExt;

    #[test]
    fn test_boolext_yn1() {
        let b1 = false;
        let x: String = b1.yn();
        assert_eq!("No", x);
        assert_eq!("No", b1.yn());
        assert_eq!("No", false.yn());
        assert!(!b1);
    }

    #[test]
    fn test_boolext_yn2() {
        let b1 = true;
        let x: String = b1.yn();
        assert_eq!("YES", x);
        assert_eq!("YES", b1.yn());
        assert_eq!("YES", true.yn());
        assert!(b1);
    }
}

#[cfg(test)]
mod tests_to_yaml {
    use yaml_rust::Yaml;
    use yaml_rust::yaml::Hash;
    use crate::yaml::ToYaml;

    #[test]
    fn test_hash_toyaml() {
        let h = Hash::new();
        let x = h.to_yaml();

        assert!(match x {
            Yaml::Hash(_y) => true,
            _ => false,
        });
    }
}

#[cfg(test)]
mod tests_to_short_string {
    use crate::string::ToShortString;

    #[test]
    fn test_to_short_string1() {
        let s1 = "ABCDEFGH".to_string().to_short_string(5);
        assert_eq!("AB...", format!("{}", s1));
    }
}
