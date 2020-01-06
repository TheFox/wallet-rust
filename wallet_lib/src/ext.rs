/// Extern Implementations

use crate::types::Number;
use crate::yaml::ToYaml;
use yaml_rust::Yaml;

pub trait StringExt {
    fn replace_comma(&self) -> String;
    fn to_num(&self) -> Number;
}

impl StringExt for String {
    /// On some places on earth you write 1,23
    /// instead of 1.23 as a floating point number.
    /// This function replaces ',' with '.'.
    fn replace_comma(&self) -> String {
        // println!("-> String.replace_comma() -> {:?}", self);

        self.replace(",", ".")
    }

    /// Convert String to Number.
    fn to_num(&self) -> Number {
        // println!("-> String.to_num() -> {:?}", self);

        self.parse().expect("Failed to convert String to Number")
    }
}

impl ToYaml for String {
    fn to_yaml(self) -> Yaml {
        Yaml::String(self)
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

#[cfg(test)]
mod tests {
    use super::{StringExt, BoolExt};
    use crate::yaml::ToYaml;
    use yaml_rust::Yaml;

    #[test]
    fn test_strext_replace_comma1() {
        let s1 = String::from("1,2");
        assert_eq!("1.2", s1.replace_comma());
    }

    #[test]
    fn test_strext_to_num1() {
        let s1 = String::from("1.3");
        assert_eq!(1.3, s1.to_num());
    }

    #[test]
    fn test_strext_to_num2() {
        let s1 = String::from("1,3");
        assert_eq!(1.3, s1.replace_comma().to_num());
    }

    #[test]
    fn test_strext_to_toyaml1() {
        let s1 = String::from("hello1").to_yaml();

        assert!(match s1 {
            Yaml::String(s2) => {
                // println!("s2 '{}'", s2);
                assert_eq!("hello1", s2);
                true
            }
            _ => false,
        });
    }

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
